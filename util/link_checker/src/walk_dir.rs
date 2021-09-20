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
