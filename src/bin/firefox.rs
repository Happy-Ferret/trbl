use std::ffi::OsString;

fn main() {
    let args = std::env::args_os().collect::<Vec<OsString>>();
    println!("{:?}", args);
}
