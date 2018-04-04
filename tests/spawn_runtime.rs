#[macro_use]
extern crate lazy_static;

use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(target_os = "macos")]
const APPLICATION_INI: &'static str = "../Resources/qbrt/application.ini";
#[cfg(not(target_os = "macos"))]
const APPLICATION_INI: &'static str = "qbrt/application.ini";

lazy_static! {
    static ref CURRENT_EXE: PathBuf = std::env::current_exe().unwrap();
    static ref PARENT_DIR: PathBuf = PathBuf::from(CURRENT_EXE.parent().unwrap());
    static ref GRANDPARENT_DIR: PathBuf = PathBuf::from(PARENT_DIR.parent().unwrap());
    static ref COMMAND_NAME: PathBuf = Path::new(GRANDPARENT_DIR.as_path()).join(
        "trbl".to_string() + if cfg!(target_os = "windows") { ".exe" } else { "" });
}

#[test]
fn run_trbl() {
    let output = Command::new(COMMAND_NAME.as_os_str())
        .current_dir(PARENT_DIR.as_path())
        .output()
        .expect("error running trbl");
    assert_eq!(String::from_utf8_lossy(&output.stdout),
        format!(r#"["--app", "{}"]"#, APPLICATION_INI));
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn run_trbl_with_arg() {
    let output = Command::new(COMMAND_NAME.as_os_str())
        .current_dir(PARENT_DIR.as_path())
        .arg("foo")
        .output()
        .expect("error running trbl");
    assert_eq!(String::from_utf8_lossy(&output.stdout),
        format!(r#"["foo", "--app", "{}"]"#, APPLICATION_INI));
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn run_trbl_with_exit_code() {
    let output = Command::new(COMMAND_NAME.as_os_str())
        .current_dir(PARENT_DIR.as_path())
        .args(&["--exit-code", "1"])
        .output()
        .expect("error running trbl");
    assert_eq!(String::from_utf8_lossy(&output.stdout),
        format!(r#"["--exit-code", "1", "--app", "{}"]"#, APPLICATION_INI));
    assert_eq!(output.status.code(), Some(1));
}
