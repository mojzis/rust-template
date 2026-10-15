//! Greeting helpers.

/// Return a greeting addressed to `name`, ignoring surrounding whitespace.
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name.trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addresses_the_name() {
        assert_eq!(greet("world"), "Hello, world!", "plain name");
    }

    #[test]
    fn trims_surrounding_whitespace() {
        assert_eq!(greet("  world \n"), "Hello, world!", "whitespace is trimmed");
    }
}
