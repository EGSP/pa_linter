use std::path::Path;

pub fn get_path_name(path: &Path) -> String {
    path.file_name().unwrap().to_str().unwrap().to_string()
}
