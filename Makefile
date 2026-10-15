.PHONY: review review-quick fmt fmt-check lint test doc audit deny coverage mutants wheel setup

# Full review — run before pushing or merging
review: fmt-check lint test doc audit deny
	@echo ""
	@echo "✅ All review checks passed"

# Quick review — skip slower network checks
review-quick: fmt-check lint test doc
	@echo ""
	@echo "✅ Quick review passed"

# Install the prek commit hook (once per clone)
setup:
	@prek install

fmt:
	@cargo fmt --all

fmt-check:
	@echo "📐 Checking formatting..."
	@cargo fmt --all -- --check

lint:
	@echo "🔍 Running clippy..."
	@cargo clippy --all-targets --all-features -- -D warnings

test:
	@echo "🧪 Running tests..."
	@if command -v cargo-nextest > /dev/null 2>&1; then \
		cargo nextest run --all-features; \
	else \
		cargo test --all-features; \
	fi

# Broken intra-doc links render as plain text on docs.rs and nothing else
# catches them, so they are part of the lint gate.
doc:
	@echo "📖 Checking rustdoc..."
	@RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features

audit:
	@echo "🔒 Running security audit..."
	@if command -v cargo-audit > /dev/null 2>&1; then \
		cargo audit; \
	else \
		echo "⚠️  cargo-audit not installed. Run: cargo install cargo-audit"; \
	fi

deny:
	@echo "🚫 Checking dependency policies..."
	@if command -v cargo-deny > /dev/null 2>&1; then \
		cargo deny check; \
	else \
		echo "⚠️  cargo-deny not installed. Run: cargo install cargo-deny"; \
	fi

coverage:
	@echo "📊 Generating coverage report..."
	@if command -v cargo-llvm-cov > /dev/null 2>&1; then \
		cargo llvm-cov --all-features --workspace --html; \
		echo "Report: target/llvm-cov/html/index.html"; \
	else \
		echo "⚠️  cargo-llvm-cov not installed. Run: cargo install cargo-llvm-cov"; \
	fi

mutants:
	@echo "🧬 Running mutation testing on recent changes..."
	@if command -v cargo-mutants > /dev/null 2>&1; then \
		cargo mutants --in-diff HEAD~1..HEAD; \
	else \
		echo "⚠️  cargo-mutants not installed. Run: cargo install cargo-mutants"; \
	fi

# Build the Python wheel that ships the binary. `bindings = "bin"` in
# pyproject.toml is what makes the wheel a binary wheel rather than a module.
wheel:
	@echo "📦 Building the wheel..."
	@if command -v maturin > /dev/null 2>&1; then \
		maturin build --release; \
	else \
		echo "⚠️  maturin not installed. Run: uv tool install maturin"; \
	fi
