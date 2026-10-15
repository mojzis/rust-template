//! End-to-end tests of the built binary.

use assert_cmd::Command;
use predicates::str::contains;

fn bin() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("rust-template"))
}

#[test]
fn greet_prints_to_stdout_and_exits_zero() {
    bin().args(["greet", "Ada"]).assert().success().stdout("Hello, Ada!\n");
}

#[test]
fn greet_defaults_to_world() {
    bin().arg("greet").assert().success().stdout("Hello, world!\n");
}

#[test]
fn unknown_subcommand_is_a_usage_error() {
    bin().arg("nope").assert().code(2).stderr(contains("unrecognized subcommand"));
}
