use std::{path::{Path, PathBuf}, process::Command};

use serde::{Deserialize, Serialize};

use crate::directory_image::DirectoryImage;

#[derive(Serialize,Deserialize,Debug)]
pub struct EditorEnvironment{
    pub executable_file_path : String,
    pub executable_folder_path : String
}

const WORKSPACE_FOLDER_NAME : &str = "workspace";
const IMAGES_FOLDER_NAME : &str = "images";

impl EditorEnvironment{

    pub fn new(executable_file_path : String, executable_folder_path : String) -> EditorEnvironment{
        EditorEnvironment{executable_file_path: executable_file_path, executable_folder_path: executable_folder_path}

    }
    
    pub fn get_workspace_folder(&self) -> Result<PathBuf,String>{
        let workspace_folder = PathBuf::from(&self.executable_folder_path).join(WORKSPACE_FOLDER_NAME);

        Ok(workspace_folder)
    }

    pub fn get_or_create_folder(&self, folder_name : &str) -> Result<PathBuf,String>{
        let folder = PathBuf::from(&self.get_workspace_folder().unwrap()).join(folder_name);
        if !folder.exists(){
            // create folder
            if std::fs::create_dir(&folder).is_err() {
                return Err(String::from("Could not create folder"));
            }
        }
        Ok(folder)
    }

    fn ini_workspace_folder(&self) -> Result<PathBuf,String>{
   
        let workspace_folder = PathBuf::from(&self.executable_folder_path).join(WORKSPACE_FOLDER_NAME);
        if workspace_folder.exists(){
            Ok(workspace_folder)
        }else{
            // create folder
            if std::fs::create_dir(&workspace_folder).is_err() {
                return Err(String::from("Could not create workspace folder"));
            }
            Ok(workspace_folder)
        }
    }
    
    #[deprecated(note = "use get_or_create_folder instead")]
    pub fn ini_images_folder(&self) -> Result<PathBuf,String>{
        let images_folder = self.get_workspace_folder().unwrap().join(IMAGES_FOLDER_NAME);
        if images_folder.exists(){
            Ok(images_folder)
        }else{
            // create folder
            if std::fs::create_dir(&images_folder).is_err() {
                return Err(String::from("Could not create images folder"));
            }
            Ok(images_folder)
        }
    }
}

pub fn try_ini_editor_environment() -> Result<EditorEnvironment,String>{
    let executable_file_path = std::env::current_exe();
    if executable_file_path.is_err(){
        return Err(String::from("Could not get editor executable path"));
    }
    let executable_file_path = executable_file_path.unwrap();

    let binding = std::env::current_exe().unwrap();
    let executable_folder_path = binding.parent();
    if executable_folder_path.is_none(){
        return Err(String::from("Could not get editor folder path"));
    }
    let executable_folder_path = executable_folder_path.unwrap();

    let editor_environment = EditorEnvironment::new(executable_file_path.to_str().unwrap().to_string(), executable_folder_path.to_str().unwrap().to_string());
    let ini_workspace_folder = editor_environment.ini_workspace_folder();
    if ini_workspace_folder.is_err(){
        return Err(ini_workspace_folder.err().unwrap());
    }
    let ini_images_folder = editor_environment.ini_images_folder();
    if ini_images_folder.is_err(){
        return Err(ini_images_folder.err().unwrap());
    }

    Ok(editor_environment)
}


pub fn reveal_in_explorer(path: &Path)-> Result<(),String>{
    if !path.exists(){
        return Err(String::from("Path does not exist"));
    }

    let _ = Command::new("explorer")
        .arg(path) // <- Specify the directory you'd like to open.
        .spawn()
        .unwrap();

    Ok(())
}

// pub fn debug_editor_environment() -> Result<(),String>{
//     println!("DEBUG EDITOR ENVIRONMENT:");
//     let editor_env = try_ini_editor_environment();
//     if editor_env.is_err(){
//         return Err(editor_env.err().unwrap());
//     }
//     let editor_env = editor_env.unwrap();
//     println!("EDITOR ENV: {:#?}", editor_env);

//     let directory_images = editor_env.get_directory_images();
//     println!("DIRECTORY IMAGES: ");
//     for image in directory_images {
//         println!("IMAGE name: {}; files: {}", image.name, image.files.len());
//     }

//     Ok(())
// }