use std::{
    env,
    path::{Path, PathBuf},
    fs::{self, File},
    io::{Read, Write}
};
use binuid_shared::zip;

pub(crate) fn save_zip(root: &str, filename: &str, files: &[PathBuf]) -> zip::result::ZipResult<()> {
    let Ok(mut current_dir) = env::current_dir() else {
        return Ok(());
    };
    current_dir.push("target");
    let _ = fs::create_dir_all(&current_dir);
    current_dir.push(filename);
    let path = Path::new(filename);
    let file = File::create(current_dir.as_path()).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);

    files.iter().for_each(|f| {
        let content = fs::read(f).unwrap();
        
        let path = f.as_path().to_string_lossy();
        let name = path.replace(root, "");
        let name = name.trim_start_matches("\\").replace("\\", "/");

        let _ = zip.start_file(name, options);
        let _ = zip.write_all(&content);
    });
    zip.finish()?;
    Ok(())
}

pub(crate) fn extract_lib_from_zip(name: &str, version: &str) {
    let lib_zip_name = format!("{}_v_{}.zip", name, version);
    let lib_name = format!("{}_v_{}", name, version);
    let Ok(mut current_dir) = env::current_dir() else {
        return;
    };
    current_dir.push("deps");
    let mut dest = current_dir.clone();
    current_dir.push(format!("{lib_zip_name}"));
    dest.push(format!("lib{lib_name}.rlib"));
    let mut buf: Vec<u8> = vec![];
    let path = Path::new(&current_dir);
    let file = File::open(path).unwrap();
    let mut zip = zip::ZipArchive::new(file).unwrap();
    let mut lib_entry = zip.by_name(&format!("dist/lib{}.rlib", lib_name)).unwrap();
    let _ = lib_entry.read_to_end(&mut buf);
    let dest_path = Path::new(&dest);
    let mut dest_file = File::create(dest_path).unwrap();
    let _ = dest_file.write_all(&buf);
}