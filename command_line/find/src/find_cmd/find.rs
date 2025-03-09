use crate::Args;
use anyhow::Result;
use clap::builder::PossibleValue;
use clap::ValueEnum;
use walkdir::DirEntry;
use walkdir::WalkDir;

///
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum EntryType {
    /// Directory
    Dir,
    /// File Entry
    File,
    /// Link Entry
    Link,
}

///
impl ValueEnum for EntryType {
    ///
    fn value_variants<'a>() -> &'a [Self] {
        &[EntryType::Dir, EntryType::File, EntryType::Link]
    }

    ///
    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            EntryType::Dir => PossibleValue::new("d"),
            EntryType::File => PossibleValue::new("f"),
            EntryType::Link => PossibleValue::new("l"),
        })
    }
}

/// type_filter
/// name_filter
/// path walk
pub fn run(arg: Args) -> Result<()> {
    //    println!("run command");
    let type_filter = |entry: &DirEntry| {
        //
        arg.entry_types.is_empty()
            || arg.entry_types.iter().any(|entry_type| match entry_type {
                EntryType::Link => entry.file_type().is_symlink(),
                EntryType::File => entry.file_type().is_file(),
                EntryType::Dir => entry.file_type().is_dir(),
            })
    };
    let name_filter = |entry: &DirEntry| {
        arg.names.is_empty()
            || arg
                .names
                .iter()
                .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };

    for path in &arg.paths {
        //        println!("{}", path);
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("{e}");
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<String>>();
        //        println!("{:?}", entries);
        println!("{}", entries.join("\n"));
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    ///
    #[test]
    fn test_args() -> Result<()> {
        Ok(())
    }
}
