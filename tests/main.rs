use std::process::Command;

#[test]
fn prints_greeting() {
    let output = Command::new(env!("CARGO_BIN_EXE_rust-template"))
        .output()
        .expect("failed to run the CLI");

    assert!(output.status.success());
    assert_eq!(output.stdout, b"Hello, world!\n");
}
