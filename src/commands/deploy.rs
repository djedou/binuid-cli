use std::process::{Command, Stdio};

pub(crate) fn deploy(host: &str, port: u16) {
    match Command::new(get_duid_deploy())
        .args(["--host", &host, "--port", &port.to_string()])
        .stdout(Stdio::inherit())
        .status()
    {
        Ok(_) => {},
        Err(err) => {
            println!("Err: {:#?}", err);
        }
    }
}


// The function is only included in the build when compiling for macOS
#[cfg(target_os = "macos")]
fn get_duid_deploy() -> &'static str {
    "macos" // TODO
}

// The function is only included in the build when compiling for windows
#[cfg(target_os = "windows")]
fn get_duid_deploy() -> &'static str {
    "./tools/binuid-deploy.exe"
}


// The function is only included in the build when compiling for linux
#[cfg(target_os = "linux")]
fn get_duid_deploy() -> &'static str {
    "linux" // TODO
}