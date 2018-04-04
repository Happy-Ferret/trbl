use std::ffi::OsString;

fn main() {
    let exit_code = echo();
    std::process::exit(exit_code);
}

fn echo() -> i32 {
    let args = std::env::args_os().skip(1).collect::<Vec<OsString>>();
    print!("{:?}", args);

    let mut iter = std::env::args_os().skip_while(|i| *i != OsString::from("--exit-code")).skip(1);

    match iter.next() {
        Some(code) => code.clone().into_string().unwrap().parse::<i32>().unwrap(),
        None => 0,
    }
}
