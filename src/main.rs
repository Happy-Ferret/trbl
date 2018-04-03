use std::process::Command;

fn main() {
    let output = Command::new("./firefox")
        .current_dir(std::env::current_exe().unwrap().parent().unwrap())
        .args(&["--app", "../Resources/qbrt/application.ini"])
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
    });

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("command succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("command failed and stderr was:\n{}", s);
    }
}
