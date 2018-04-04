use std::path::Path;
use std::process::Command;

#[cfg(target_os = "macos")]
const APPLICATION_INI: &'static str = "../Resources/qbrt/application.ini";
#[cfg(not(target_os = "macos"))]
const APPLICATION_INI: &'static str = "qbrt/application.ini";

#[test]
fn run_trbl() {
    let current_exe = std::env::current_exe().unwrap();
    let parent_dir = current_exe.parent().unwrap().parent().unwrap();
    let command_name = if cfg!(target_os = "windows") {
        Path::new(parent_dir).join("trbl.exe")
    } else {
        Path::new(parent_dir).join("trbl")
    };

    let output = Command::new(command_name)
        .current_dir(std::env::current_exe().unwrap().parent().unwrap())
        .output()
        .expect("error running trbl");
    assert_eq!(String::from_utf8_lossy(&output.stdout),
        format!(r#"["--app", "{}"]"#, APPLICATION_INI));
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn run_trbl_with_arg() {
    let current_exe = std::env::current_exe().unwrap();
    let parent_dir = current_exe.parent().unwrap().parent().unwrap();
    let command_name = if cfg!(target_os = "windows") {
        Path::new(parent_dir).join("trbl.exe")
    } else {
        Path::new(parent_dir).join("trbl")
    };

    let output = Command::new(command_name)
        .current_dir(std::env::current_exe().unwrap().parent().unwrap())
        .arg("foo")
        .output()
        .expect("error running trbl");
    assert_eq!(String::from_utf8_lossy(&output.stdout),
        format!(r#"["foo", "--app", "{}"]"#, APPLICATION_INI));
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn run_trbl_with_exit_code() {
    let current_exe = std::env::current_exe().unwrap();
    let parent_dir = current_exe.parent().unwrap().parent().unwrap();
    let command_name = if cfg!(target_os = "windows") {
        Path::new(parent_dir).join("trbl.exe")
    } else {
        Path::new(parent_dir).join("trbl")
    };

    let output = Command::new(command_name)
        .current_dir(std::env::current_exe().unwrap().parent().unwrap())
        .args(&["--exit-code", "1"])
        .output()
        .expect("error running trbl");
    assert_eq!(String::from_utf8_lossy(&output.stdout),
        format!(r#"["--exit-code", "1", "--app", "{}"]"#, APPLICATION_INI));
    assert_eq!(output.status.code(), Some(1));
}
