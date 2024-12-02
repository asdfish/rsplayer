use std::{
    io::Result,
    fs::{
        DirEntry,
        FileType,
        read_dir,
    },
};

pub enum EntryType {
    Directory,
    File,
}

fn directory_entry_is_entry_type(directory_entry: &DirEntry, entry_type: &EntryType) -> bool {
    let file_type: Result<FileType> = directory_entry.file_type();
    if file_type.is_err() {
        return false;
    }
    let file_type: FileType = file_type.unwrap();


    match entry_type {
        EntryType::Directory => file_type.is_dir(),
        EntryType::File => file_type.is_file(),
    }
}

pub fn get_entries(path: &str, entry_type: EntryType) -> Result<Vec<String>> {
    let mut entries: Vec<String> = Vec::new();

    for entry in read_dir(path)? {
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

    Result::Ok(entries)
}
