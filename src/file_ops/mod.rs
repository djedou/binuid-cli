use std::path::{Path, PathBuf};

pub(crate) fn gather_files<'a, T: Into<&'a Path>>(kip: Option<&[String]>, path: T, files: &mut Vec<PathBuf>) {
    let path: &Path = path.into();

    for entry in path.read_dir().unwrap() {
        match entry {
            Ok(e) => {
                if e.path().is_dir() {
                    gather_files(kip, e.path().as_ref(), files);
                } else if e.path().is_file() {
                    match kip {
                        Some(k) => {
                            if !k.iter().any(|a| e.path().starts_with(a)) {
                                files.push(e.path());
                            }
                        },
                        None => {
                            files.push(e.path());
                        }
                    }
                }
            }
            Err(_) => todo!(),
        }
    }
}

/*
pub(crate) fn gather_ll_files<'a, T: Into<&'a Path>>(path: T, files: &mut Vec<PathBuf>) {
    let path: &Path = path.into();

    for entry in path.read_dir().unwrap() {
        match entry {
            Ok(e) => {
                if e.path().is_dir() {
                    gather_ll_files(e.path().as_ref(), files);
                } 
                else {
                    match e.path().extension().map_or(None, |d| d.to_str()) {
                        Some("ll") => {
                            files.push(e.path());
                        },
                        _ => {}
                    }
                }
            }
            Err(_) => todo!(),
        }
    }
}
*/