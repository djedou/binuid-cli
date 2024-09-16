use binuid_shared_wasm::{
    //serde::{self, Deserialize, Serialize},
    toml::{Table, de::Error, from_str, to_string_pretty, Value}
};
use super::Dependency;

pub(crate) fn read_dependencies_from_table(table: &Table) -> Vec<Dependency> {
    let keys: Vec<String> = table.keys().into_iter().map(|k| k.clone().to_string()).collect();
    keys.iter().map(|key| {
        let mut dep = Dependency::default();
        match table.get(key) {
            Some(Value::String(value)) => {
                dep.version = Some(value.to_string());
            },
            Some(Value::Table(table)) => {
                if let Some(Value::String(value)) = table.get("version") {
                    dep.version = Some(value.to_string());
                }
                if let Some(Value::String(value)) = table.get("path") {
                    dep.path = Some(value.to_string());
                }
            },
            _ => {}
        }

        dep.name = key.to_owned();
        dep
    })
    .collect::<Vec<Dependency>>()
}

pub(crate) fn get_dep_path(name: &str, version: Option<&str>, path: Option<&str>) -> String {
    match (path, version) {
        (Some(p), Some(ver)) => {
            format!("{p}/lib{name}_v_{}.rlib", ver.replace(".", "_"))
        },
        (None, Some(ver)) => {
            format!("./deps/lib{name}_v_{}.rlib", ver.replace(".", "_"))
        },
        (None, None) => {
            format!("./deps/lib{name}_v_0_0_0.rlib")
        },
        _ => String::with_capacity(0)
    }
}

pub(crate) fn get_dep_zip_path(name: &str, version: Option<&str>, path: Option<&str>) -> String {
    
    match (path, version) {
        (Some(p), Some(ver)) => {
            format!("{p}/{}_v_{}.zip", name.replace("-", "_"), &ver.replace(".", "_"))
        },
        (None, Some(ver)) => {
            format!("deps/{}_v_{}.zip", name.replace("-", "_"), &ver.replace(".", "_"))
        },
        (None, None) => {
            format!("deps/{}_v_0_0_0.zip", name.replace("-", "_"))
        },
        _ => String::with_capacity(0)
    }
}