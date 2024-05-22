use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

#[derive(Serialize,Deserialize,Debug)]
pub struct DirectoryImage{
    pub name: String,
    pub files: Vec<String>
}



impl DirectoryImage{
    pub fn new(name: String, files: Vec<String>) -> Self{
        Self{
            name,
            files
        }
    }
}

pub fn take_directory_image(path: &str) -> DirectoryImage {
    let path = std::path::Path::new(path);
    let name = path.file_name().unwrap().to_str().unwrap().to_string();
    let mut files = Vec::new();
    
    for entry in WalkDir::new(path).contents_first(true) {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        
        let entry_path = entry_path.strip_prefix(path).unwrap_or(entry_path);
        let mut entry_path = entry_path.to_str().unwrap().to_string();
        entry_path = entry_path.replace("\\", "/");
        // Убрал, потому что с форматированием корневые файлы тоже с "/"
        // entry_path = format!("/{}", entry_path);

        if entry.file_type().is_file() {
            files.push(entry_path);
        }
    }

    DirectoryImage::new(name, files)
}

