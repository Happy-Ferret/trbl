use std::ffi::OsString;
use std::process::{ Command };

#[test]
fn spawn_runtime() {
    println!("{:?}", std::env::current_exe());
    let status = Command::new("../trbl")
        .current_dir(std::env::current_exe().unwrap().parent().unwrap())
        // .args(args)
        .status()
        .expect("error spawning runtime");
    assert_eq!(status.code(), Some(0));
}
