use std::process::{Command, Stdio};

pub(crate) fn dev() {
    match Command::new("watchexec")
        .args(["--quiet", "--exts", "rs", "binuid", "build"])
        .stdout(Stdio::inherit())
        .status()
    {
        Ok(_) => {},
        Err(err) => {
            println!("Err: {:#?}", err);
        }
    }
}