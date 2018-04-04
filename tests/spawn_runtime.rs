use std::process::Command;

#[cfg(target_os = "macos")]
const APPLICATION_INI: &'static str = "../Resources/qbrt/application.ini";
#[cfg(not(target_os = "macos"))]
const APPLICATION_INI: &'static str = "qbrt/application.ini";

#[test]
fn run_trbl() {
    let output = Command::new("../trbl")
        .current_dir(std::env::current_exe().unwrap().parent().unwrap())
        .output()
        .expect("error running trbl");
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(),
        format!(r#"["./firefox", "--app", "{}"]"#, APPLICATION_INI));
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
        format!(r#"["./firefox", "foo", "--app", "{}"]"#, APPLICATION_INI));
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
        format!(r#"["./firefox", "--exit-code", "1", "--app", "{}"]"#, APPLICATION_INI));
    assert_eq!(output.status.code(), Some(1));
}
