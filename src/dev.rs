use std::process::{Command, Stdio};

pub(crate) fn dev() {
    match Command::new("cargo")
            .args(["watch", "-w", "src", "-s", ".\\tools\\binuid-dev.exe & wasm-pack build --target web --release"])
            .stdout(Stdio::inherit())
            .status()
        {
            Ok(_) => {},
            Err(err) => {
                println!("Err: {:#?}", err);
            }
        }
}
