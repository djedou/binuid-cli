use std::env;

pub(crate) fn get_duid_lib_build(name: &str, version: &str, deps: &[String]) -> Vec<String> {
    let new_name = name.replace("-", "_");
    let Ok(mut current_dir) = env::current_dir() else {
        return vec![];
    };
    current_dir.push("dist");
    let output_directory = current_dir.clone();
    current_dir.push(format!("{new_name}.rs"));
    let mut args = vec![
        "-O".to_string(),
        "--crate-type=lib".to_string(),
        format!("--crate-name={new_name}_v_{version}"),
        format!("--out-dir={}", output_directory.display()),
        "--edition=2021".to_string(),
        "--emit=link".to_string()
    ];
    args.extend_from_slice(&deps);
    args.push(format!("{}", current_dir.display()));
    args
}

pub(crate) fn get_duid_bin_lib_build(name: &str, version: &str, deps: &[String]) -> Vec<String> {
    let Ok(mut current_dir) = env::current_dir() else {
        return vec![];
    };
    let mut output_directory = current_dir.clone();
    output_directory.push("dist");
    current_dir.push("dist");
    current_dir.push("lib");
    current_dir.push("mod.rs");
    let mut args = vec![
        "-O".to_string(),
        "--crate-type=lib".to_string(),
        format!("--crate-name={name}_v_{version}"),
        format!("--out-dir={}", output_directory.display()),
        "--edition=2021".to_string(),
        "--emit=link".to_string()
    ];
    args.extend_from_slice(&deps);
    args.push(format!("{}", current_dir.display()));
    args
}