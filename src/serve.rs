use std::process::{Command, Stdio};

pub(crate) fn serve(host: &str, port: u16) {

    match Command::new(get_duid_serve())
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
fn get_duid_serve() -> &'static str {
    "macos" // TODO
}

// The function is only included in the build when compiling for windows
#[cfg(target_os = "windows")]
fn get_duid_serve() -> &'static str {
    "./tools/binuid-serve.exe"
}


// The function is only included in the build when compiling for linux
#[cfg(target_os = "linux")]
fn get_duid_serve() -> &'static str {
    "linux" // TODO
}