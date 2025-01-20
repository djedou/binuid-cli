use std::process::{Command, Stdio};
use std::{fs, env};
use crate::{Mode, read_binuid_config, write_binuid_config};
use std::mem::swap;
use binuid_shared_wasm::console::error;


pub(crate) fn generate_template(mode: &Mode, name: &str) {
    match Command::new("cargo")
        .args(["install", "cargo-generate"])
        .stdout(Stdio::inherit())
        .status()  
    {
        Ok(_) => {},
        Err(output) => {
            error!("Err: {:#?}", output);
        }
    }

    match Command::new("cargo")
        .args(["generate", "--git", "https://github.com/djedou/binuid-template", "--name", name])
        .stdout(Stdio::inherit())
        .status() 
    {
        Ok(_) => {},
        Err(err) => {
            error!("Err: {:#?}", err);
        }
    }

    match Command::new("rustup")
        .args(["target", "add", "wasm32-unknown-unknown"])
        .stdout(Stdio::inherit())
        .status() 
    {
        Ok(_) => {},
        Err(err) => {
            error!("Err: {:#?}", err);
        }
    }

    match Command::new("cargo")
        .args(["install", "watchexec-cli"])
        .stdout(Stdio::inherit())
        .status() 
    {
        Ok(_) => {},
        Err(err) => {
            error!("Err: {:#?}", err);
        }
    }

    /*match Command::new("cargo")
        .args(["install", "trunk"])
        .stdout(Stdio::inherit())
        .status()  
    {
        Ok(_) => {},
        Err(output) => {
            error!("Err: {:#?}", output);
        }
    }*/
    match mode {
        Mode::Bin => {
            rename_cargo_to_binuid(&name);
            let Ok(mut config) = read_binuid_config(name) else {
                return;
            };
            config.package.mode = Some("bin".to_string());
            write_binuid_config(&name, config);
        },
        Mode::Lib => {
            remove_app(&name);
            create_lib_file(&name);
            rename_cargo_to_binuid(&name);
            let Ok(mut config) = read_binuid_config(name) else {
                return;
            };
            config.package.mode = Some("lib".to_string());
            write_binuid_config(&name, config);
        },
        Mode::Ws => {
            remove_app(&name);
            rename_cargo_to_binuid(&name);
            let Ok(mut config) = read_binuid_config(name) else {
                return;
            };
            config.package.mode = Some("ws".to_string());
            write_binuid_config(&name, config);
            /*
            let ws = Workspace {
                name: package.name.clone(),
                authors: package.authors.clone(),
                members: Vec::with_capacity(0)
            };
            swap(&mut Some(ws), &mut config.workspace);
            swap(&mut None, &mut config.package);
            swap(&mut None, &mut config.dependencies);
            */
        }
    }
}

fn rename_cargo_to_binuid(name: &str) {
    let Ok(mut current_dir) = env::current_dir() else {
        return;
    };
    current_dir.push(name);
    let mut root_cargo = current_dir.clone();
    let mut root_binuid = current_dir.clone();
    root_cargo.push("Cargo.toml");
    root_binuid.push("binuid.toml");
    let _ = fs::rename(&root_cargo, &root_binuid);
}

fn remove_app(name: &str) {
    let Ok(mut current_dir) = env::current_dir() else {
        return;
    };
    current_dir.push(name);
    current_dir.push("app");
    let _ = fs::remove_dir_all(&current_dir);
}

fn create_lib_file(name: &str) {
    let Ok(mut current_dir) = env::current_dir() else {
        return;
    };
    current_dir.push(name);
    current_dir.push("lib");
    let _ = fs::create_dir(&current_dir);
    let lib_name = format!("{}.rs", name.replace("-", "_"));
    current_dir.push(&lib_name);
    let _ = fs::File::create_new(&current_dir);
}