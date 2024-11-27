// tests/integration_test.rs
use std::process::Command;

fn run_test(file_path: &str, expected_output: &str) {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg(file_path)
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), expected_output);
}
#[test]
fn test_valid() {
    run_test("tests/test.txt", "valid");
}
