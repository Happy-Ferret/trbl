use std::process::Command;

#[test]
fn run_trbl() {
    let output = Command::new("../trbl")
        .current_dir(std::env::current_exe().unwrap().parent().unwrap())
        .output()
        .expect("error running trbl");
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(),
        r#"["./firefox", "--app", "../Resources/qbrt/application.ini"]"#);
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn run_trbl_with_arg() {
    let output = Command::new("../trbl")
        .current_dir(std::env::current_exe().unwrap().parent().unwrap())
        .arg("foo")
        .output()
        .expect("error running trbl");
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(),
        r#"["./firefox", "foo", "--app", "../Resources/qbrt/application.ini"]"#);
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn run_trbl_with_exit_code() {
    let output = Command::new("../trbl")
        .current_dir(std::env::current_exe().unwrap().parent().unwrap())
        .args(&["--exit-code", "1"])
        .output()
        .expect("error running trbl");
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(),
        r#"["./firefox", "--exit-code", "1", "--app", "../Resources/qbrt/application.ini"]"#);
    assert_eq!(output.status.code(), Some(1));
}
