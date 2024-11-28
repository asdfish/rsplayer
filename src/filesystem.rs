use std::{
    io::Result,
    fs,
};

pub enum EntryType {
    DIRECTORY,
    FILE,
}

fn directory_entry_is_entry_type(directory_entry: &fs::DirEntry, entry_type: &EntryType) -> bool {
    let file_type: Result<fs::FileType> = directory_entry.file_type();
    if !file_type.is_ok() {
        return false;
    }
    let file_type: fs::FileType = file_type.unwrap();


    return match entry_type {
        EntryType::DIRECTORY => file_type.is_dir(),
        EntryType::FILE => file_type.is_file(),
    };
}

pub fn get_entries(path: &str, entry_type: EntryType) -> Result<Vec<String>> {
    let mut entries: Vec<String> = Vec::new();

    for entry in fs::read_dir(path)? {
        match entry {
            Result::Ok(entry) => {
                if ! directory_entry_is_entry_type(&entry, &entry_type) {
                    continue;
                }

                match entry.file_name().into_string() {
                    Ok(file_name) => entries.push(file_name),
                    _ => continue,
                }
            },
            _ => continue,
        }
    }

    return Result::Ok(entries);
}
