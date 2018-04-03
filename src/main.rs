use std::ffi::OsString;
use std::process::Command;

fn main() {
    let mut args = std::env::args_os().skip(1).collect::<Vec<OsString>>();
    args.push(OsString::from("--app"));
    args.push(OsString::from("../Resources/qbrt/application.ini"));

    Command::new("./firefox")
        .current_dir(std::env::current_exe().unwrap().parent().unwrap())
        .args(args)
        .status().expect("runtime failed");
}
