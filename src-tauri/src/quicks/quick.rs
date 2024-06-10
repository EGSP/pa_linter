use std::path::Path;

pub fn get_path_name(path: &Path) -> String {
    path.file_name().unwrap().to_str().unwrap().to_string()
}

pub fn get_relative_path(path: &String, root_path: &String) -> Option<String> {
    let option = path.strip_prefix(root_path);
    if option.is_some() {
        Some(option.unwrap().to_string())
    }else{
        None
    }
}
