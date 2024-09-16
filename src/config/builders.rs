use std::env;

pub(crate) fn get_duid_lib_build(name: &str, version: &str, deps: &[String]) -> Vec<String> {
    let new_name = name.replace("-", "_");
    let Ok(mut current_dir) = env::current_dir() else {
        return vec![];
    };
    current_dir.push("lib");
    current_dir.push(format!("{new_name}.rs"));
    let mut args = vec![
        "--crate-type=lib".to_string(),
        format!("--crate-name={new_name}_v_{version}"),
        "--edition=2021".to_string()
    ];
    args.extend_from_slice(&deps);
    args.push(format!("{}", current_dir.display()));
    args
}

pub(crate) fn get_duid_bin_build(path: Option<&str>, deps: &[String]) -> Vec<String> {
    let new_name = path.map_or("".to_owned(), |d| {
        let paths = d.split("app").collect::<Vec<_>>();
        let res = &paths[1].trim_start_matches("\\").replace("\\", "_");
        res.replace(".rs", "")
    });
    let mut args = vec![
        "--crate-type=lib".to_string(),
        format!("--crate-name={new_name}"),
        "--edition=2021".to_string()
    ];
    args.extend_from_slice(&deps);
    args.push(format!("{}", path.map_or("", |p| p)));
    args
}