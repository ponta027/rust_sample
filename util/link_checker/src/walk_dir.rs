use std::fs;
use std::io;
use std::path::Path;

pub fn read_dir(path: &Path) -> io::Result<Vec<String>> {
    //
    Ok(fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_file() {
                Some(entry.file_name().to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect())
}

/*
fn walk_dir(path: &Path) -> () {
    println!("{:?}", path.display());
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            walk_dir(&path);
        } else {
            println!("{:?}", path.display());
        }
    }
    ()
}
*/
