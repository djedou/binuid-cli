use std::process::{Command, Stdio};

pub(crate) fn build() {
    match Command::new(get_duid_build())
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
fn get_duid_build() -> &'static str {
    "macos" // TODO
}

// The function is only included in the build when compiling for windows
#[cfg(target_os = "windows")]
fn get_duid_build() -> &'static str {
    "./tools/binuid-build.exe"
}


// The function is only included in the build when compiling for linux
#[cfg(target_os = "linux")]
fn get_duid_build() -> &'static str {
    "linux" // TODO
}