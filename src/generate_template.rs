use std::process::{Command, Stdio};


pub(crate) fn generate_template(name: &str) {
    match Command::new("cargo")
        .args(["install", "cargo-generate"])
        .stdout(Stdio::inherit())
        .status()  
        {
            Ok(_) => {},
            Err(output) => {
                println!("Err: {:#?}", output);
            }
        }


    match Command::new("cargo")
            .args(["install", "cargo-watch"])
            .stdout(Stdio::inherit())
            .status() 
        {
            Ok(_) => {},
            Err(watch) => {
                println!("Err: {:#?}", watch);
            }
        }

    match Command::new("cargo")
            .args(["generate", "--git", "https://github.com/djedou/binuid-template", "--name", name])
            .stdout(Stdio::inherit())
            .status() 
        {
            Ok(_) => {},
            Err(err) => {
                println!("Err: {:#?}", err);
            }
        }

    match Command::new("rustup")
        .args(["target", "add", "wasm32-unknown-unknown"])
        .stdout(Stdio::inherit())
        .status() 
    {
        Ok(_) => {},
        Err(err) => {
            println!("Err: {:#?}", err);
        }
    }        
}