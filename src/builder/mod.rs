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

/*
pub(crate) fn get_duid_bin_build(path: Option<&str>, deps: &[String]) -> Vec<String> {
    let Ok(mut current_dir) = env::current_dir() else {
        return vec![];
    };
    current_dir.push("dist");
    current_dir.push("libs");
    let (new_name, directory) = path.map_or(("".to_owned(), format!("{}", current_dir.display())), |d| {
        let paths = d.split("pages").collect::<Vec<_>>();
        let mut res = &mut paths[1].trim_start_matches("\\").split("\\").collect::<Vec<_>>();
        match res.len() {
            0 => ("".to_owned(), format!("{}", current_dir.display())),
            1 => (res[0].replace(".rs", ""), format!("{}", current_dir.display())),
            _ => {
                res.reverse();
                let name = res[..1].iter().next().map_or("".to_owned(), |d| d.to_string());
                let mut paths = res[1..].into_iter().collect::<Vec<_>>();
                paths.reverse();
                paths.iter().for_each(|p| {
                    current_dir.push(p);
                });

                (name.replace(".rs", ""), format!("{}", current_dir.display()))
            }
        }
    });
    let mut args = vec![
        "-O".to_string(),
        "--crate-type=lib".to_string(),
        format!("--crate-name={new_name}"),
        format!("--out-dir={directory}"),
        "--edition=2021".to_string(),
        "--emit=link,llvm-ir".to_string()
    ];
    args.extend_from_slice(&deps);
    args.push(format!("{}", path.map_or("", |p| p)));
    args
}
    */