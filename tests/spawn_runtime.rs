use std::process::Command;

#[test]
fn run_trbl() {
    let status = Command::new("../trbl")
        .current_dir(std::env::current_exe().unwrap().parent().unwrap())
        .status()
        .expect("error running trbl");
    assert_eq!(status.code(), Some(0));
}

#[test]
fn run_trbl_with_arg() {
    let status = Command::new("../trbl")
        .current_dir(std::env::current_exe().unwrap().parent().unwrap())
        .arg("foo")
        .status()
        .expect("error running trbl");
    assert_eq!(status.code(), Some(0));
}

#[test]
fn run_trbl_with_exit_code_1() {
    let status = Command::new("../trbl")
        .current_dir(std::env::current_exe().unwrap().parent().unwrap())
        .args(&["--exit-code", "1"])
        .status()
        .expect("error running trbl");
    assert_eq!(status.code(), Some(1));
}
