use std::process::{Command, Stdio};

fn esobox() -> Command {
    test_bin::get_test_bin("esobox")
}

#[test]
fn integration_print_bf() {
    let mut esobox = esobox();
    let lang = "brainfuck";
    let source = "tests/brainfuck/print_bf.bf";
    let output = esobox
        .args([lang, source])
        .stdin(Stdio::null())
        .output()
        .expect("Failed to run process");
    let stdout = String::from_utf8(output.stdout).expect("Output is not valid UTF-8");
    let stderr = String::from_utf8(output.stderr).expect("Output is not valid UTF-8");
    assert_eq!(stdout, "brainfuck");
    assert!(stderr.is_empty());
}
