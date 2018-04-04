use std::ffi::OsString;
use std::process::{ Command };

fn main() {
    let exit_code = spawn_runtime();
    std::process::exit(exit_code)
}

fn spawn_runtime() -> i32 {
    let application_ini = if cfg!(target_os = "macos") {
        OsString::from("../Resources/qbrt/application.ini")
    } else {
        OsString::from("qbrt/application.ini")
    };

    let mut args = std::env::args_os().skip(1).collect::<Vec<OsString>>();

    args.push(OsString::from("--app"));
    args.push(application_ini);

    let status = Command::new("./firefox")
        .current_dir(std::env::current_exe().unwrap().parent().unwrap())
        .args(args)
        .status()
        .expect("error spawning runtime");

    match status.code() {
        Some(code) => code,
        None => -1,
    }
}
