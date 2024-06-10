use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::editor::editor::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectoryImage {
    pub name: String,
    pub files: Vec<String>,
}

const DIRECTORY_IMAGES_FOLDER_NAME: &str = "images";

impl DirectoryImage {
    pub fn new(name: String, files: Vec<String>) -> Self {
        Self { name, files }
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

pub fn save_directory_image(image: &DirectoryImage, editor_env: &EditorEnvironment) {
    let directory_images_folder = editor_env
        .get_or_create_folder(DIRECTORY_IMAGES_FOLDER_NAME)
        .unwrap();
    let file_path = directory_images_folder.join(format!("{}.json", image.name));
    let serialized = serde_json::to_string(&image).unwrap();
    std::fs::write(&file_path, serialized).unwrap();
}

pub fn get_directory_images(editor_env: &EditorEnvironment) -> Vec<DirectoryImage> {
    let directory_images_folder = editor_env
        .get_or_create_folder(DIRECTORY_IMAGES_FOLDER_NAME)
        .unwrap();
    
    let mut directory_images: Vec<DirectoryImage> = Vec::new();
    for entry in WalkDir::new(directory_images_folder).contents_first(true) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() && entry.file_name().to_str().unwrap().ends_with(".json") {
            let file_content = std::fs::read_to_string(entry.path()).unwrap();
            let directory_image = serde_json::from_str(&file_content);
            if directory_image.is_ok() {
                directory_images.push(directory_image.unwrap());
            }
        }
    }
    directory_images
}
