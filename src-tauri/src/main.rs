// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::panic;
use std::{cell::OnceCell, path::Path, process::Command, sync::OnceLock};

use nodes::{ArenaTree, Node, NodeId};
use rand::Rng;
use serde_json::{Map, Value};
use tauri::api::file;
use walkdir::{DirEntry, WalkDir};

use crate::{
    analyzers::json_analyzer::JsonAnalyzeTask,
    directory_image::{take_directory_image, DirectoryImage},
    project::Project,
};

mod analyzer;
mod analyzers;
mod directory_image;
mod nodes;
mod project;

static PROJECT: OnceLock<Project> = OnceLock::new();

fn main() {
    const test_folder_path: &str = "c:/Workroot/softdev/pa_linter_test/";

    // let results = analyzer::analyze_folder(test_folder_path);
    // for result in results {
    //     println!("{:#?}", result);
    // }

    // const tree_test_folder_path: &str =
    //     "c:/Workroot/softdev/pa_linter_test_tree/Consultant-Balance-main";
    const PROJECT_TEST_FOLDER_PATH: &str =
        "c:/Workroot/softdev/pa_linter_test_tree/Consultant-Balance-main";

    let project = Project::try_initilize_project(PROJECT_TEST_FOLDER_PATH);
    if project.is_err() {
        panic!("{}", project.err().unwrap());
    }
    let project = project.unwrap();
    let _ = PROJECT.set(project);

    let project = PROJECT.get().unwrap();
    let arena_tree = &project.arena_tree;

    // let arena_tree = scan_project_folder(Path::new(tree_test_folder_path)).unwrap();
    println!("{} nodes in arena tree", arena_tree.nodes_map.len());
    // println!("{:?}", arena_tree);

    // build paths for 3 random nodes
    // for _ in 0..3 {
    //     let node_id = get_random_node_from_arena_tree(&arena_tree);
    //     // let paths = build_node_path_from_arena_tree(&arena_tree, node_id);

    //     let path = project.build_node_path(arena_tree,node_id);

    //     let node = arena_tree.get_node_by_id(node_id).unwrap();
    //     println!("{} path is {}", node.value, path);
    // }

    for _ in 0..3 {
        println!();
    }

    // let json_task = JsonAnalyzeTask::new(&project);
    // let results = json_task.run();
    // for result in results {
    //     println!("{:#?}", result);
    // }

    let directory_image = take_directory_image(PROJECT_TEST_FOLDER_PATH);
    println!("STRUCT: {:#?}", directory_image);
    // let directory_image_json = serde_json::to_string(&directory_image).unwrap();
    // println!("JSON: {}", directory_image_json);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            analyze_folder,
            get_project_folder_arena_tree,
            c_take_directory_image
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn analyze_folder(folder_path: &str) -> Vec<analyzer::AnalysisResult> {
    analyzer::analyze_folder(folder_path)
}

#[tauri::command]
fn get_project_folder_arena_tree(folder_path: &Path) -> Result<ArenaTree, String> {
    let arena_tree = scan_project_folder(folder_path).unwrap();

    let serialized = serde_json::to_string(&arena_tree).unwrap();
    println!("{}", serialized);
    Ok(arena_tree)
}

#[tauri::command]
fn c_take_directory_image(folder_path: &Path) {
    let image = take_directory_image(folder_path.to_str().unwrap());

    let serialized = serde_json::to_string(&image).unwrap();
    let closure_serialized = serialized.clone();
    tauri::api::dialog::FileDialogBuilder::new()
        .set_title("Save Directory Image")
        .set_file_name("directory_image.json")
        .save_file(move |file_path| {
            if file_path.is_some() {
                let file_path = file_path.unwrap();
                let file_directory = file_path.parent().unwrap();
                std::fs::write(&file_path, closure_serialized)
                    .expect("Failed to write to file");
                
                Command::new("explorer")
                    .arg(file_directory) // <- Specify the directory you'd like to open.
                    .spawn()
                    .unwrap();
            }
        });
    println!("{}", serialized);
}

fn scan_project_folder(folder_path: &Path) -> Result<ArenaTree, String> {
    // if folder contains modinfo.json file, then it is a project folder
    if !folder_path.join("modinfo.json").exists() {
        return Err(
            "Folder is not a project folder. Project folder must contain modinfo.json file"
                .to_string(),
        );
    }

    let mut arena_tree = ArenaTree::new();

    // iterate directory
    iterate_directory(folder_path, &None, &mut arena_tree);

    Ok(arena_tree)
}

fn build_node_path_from_arena_tree(arena_tree: &ArenaTree, node_id: i32) -> Vec<String> {
    let mut node = arena_tree.get_node_by_id(node_id).unwrap();

    let mut paths: Vec<String> = Vec::new();
    paths.push(node.value.clone());

    while node.parent.is_some() {
        node = arena_tree.get_node_by_id(node.parent.unwrap()).unwrap();
        paths.push(node.value.clone());
    }

    paths.reverse();
    paths
}

fn get_random_node_from_arena_tree(arena_tree: &ArenaTree) -> i32 {
    let nodes = arena_tree.get_nodes_all();
    let random_index = rand::thread_rng().gen_range(0..nodes.len());
    nodes[random_index].id
}

fn iterate_directory(folder_path: &Path, previous_node: &Option<i32>, arena_tree: &mut ArenaTree) {
    let folder_node: Node = Node::new(get_path_name(folder_path), String::from(""));
    let folder_node_id: i32;

    if previous_node.is_none() {
        if arena_tree.get_root_node().is_none() {
            folder_node_id = arena_tree.add_root_node(folder_node);
        } else {
            // корневые ноды можно определять по наличию родителя - можно сделать когда-нибудь.
            // folder_node_id = arena_tree.add_node(folder_node);
            panic!("Root node already exists in arena tree");
        }
    } else {
        folder_node_id = arena_tree.add_node_to_parent_id(previous_node.unwrap(), folder_node);
    }

    for entry in WalkDir::new(folder_path.to_str().unwrap())
        .min_depth(1)
        .max_depth(1)
    {
        let entry = entry.unwrap();
        let entry_name = entry.file_name().to_str().unwrap();
        let entry_path = folder_path.join(entry_name);
        let entry_path_ref = entry_path.as_path();

        println!("{} + {}", folder_path.to_str().unwrap(), entry_name);
        println!(" ");

        if entry.file_type().is_dir() {
            iterate_directory(entry_path_ref, &Some(folder_node_id), arena_tree);
        } else if entry.file_type().is_file() {
            let file_node = Node::new(entry_name.to_string(), String::from(""));
            arena_tree.add_node_to_parent_id(folder_node_id, file_node);
        } else {
            // do nothing
        }
    }
}

fn get_path_name(path: &Path) -> String {
    path.file_name().unwrap().to_str().unwrap().to_string()
}

// найти свойства в json файлах со строковыми значениями.
// если значение это относительный путь, то нужно проверить его на корректность.

// проверка на корректность состоит из двух этапов:
// 1) проверить формат строки
// [строка должна начинаться на '/' и все символы '\' должны быть заменены на '/']
// 2) проверить наличие строки в справочнике, в котором хранится снимок файловой структуры
// если строка не прошла проверку,
// то нужно показать пользователю файл и название свойства

// справочник содержит снимок файловой структуры и название справочника
// снимок файловой структуры состоит из узлов
// узел хранит строковое значение, ссылку на родительский узел, список альтернативных узлов
// и список дочерних узлов,
// также узел хранит название своего справочника
// альтернативный узел - это узел, который имеет то же значение, но принадлежит другому справочнику

// если справочников несколько, то их нужно объединить под одним именем,
// объединить структуры, используя функционал альтернативных узлов

// перебор вглубь идет по ткому алгоритму:
// 1) у нас есть значение следующего узла
// 2) смотрим есть ли дочерний узел с таким значением в текущем узле
// 3) если нет, то продолжаем перебор в альтернативных узлах текущего узла
// 4) если нигде не нашли, то - нигде не нашли

// [альтернативные узлы позволяют по своей природе сохранять
// и держать всю исходную информацию, без потери её во время слияния деревьев]

// В программе есть:
// - справочники оригинальных структур
// - справочники рабочих структур

// справочники рабочих структур нужно обновлять при изменении файловой структуры или
// содержимого файлов.

// если какой-то файл изменяется или удаляется, то
// файл в рабочем справочнике нужно обновить.

// ДОПОЛНИТЕЛЬНО:
// узлы справочников могут хранить ссылки друг на друга, если
// свойства в файлах ссылаются на другие файлы. Но думаю это лучше делать
// отдельной мета-структурой.

// Добавить авто-фикс проблем с путями.
