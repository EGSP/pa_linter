// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use core::panic;
use std::path::Path;

use nodes::{ArenaTree, Node};
use serde_json::{Map, Value};
use walkdir::{DirEntry, WalkDir};

mod analyzer;
mod nodes;

fn main() {
    const test_folder_path: &str = "c:/Workroot/softdev/pa_linter_test/";

    let results = analyzer::analyze_folder(test_folder_path);
    for result in results {
        println!("{:#?}", result);
    }

    const tree_test_folder_path: &str =
        "c:/Workroot/softdev/pa_linter_test_tree/Consultant-Balance-main";

    let arena_tree = scan_project_folder(Path::new(tree_test_folder_path)).unwrap();
    println!("{} nodes in arena tree", arena_tree.nodes_map.len());
    println!("{:?}", arena_tree);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, analyze_folder,
            get_project_folder_arena_tree])
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

fn scan_project_folder(folder_path: &Path) -> Result<ArenaTree, String> {
    // if folder contains modinfo.json file, then it is a project folder
    if !folder_path.join("modinfo.json").exists() {
        return Err(
            "Folder is not a project folder. Project folder must contain modinfo.json file"
                .to_string(),
        );
    }

    let mut arena_tree = ArenaTree::new();

    // // add root node
    // let root_folder_name = get_path_name(folder_path);
    // let root_node = nodes::Node::new(root_folder_name, String::from(""));
    // arena_tree.add_root_node(root_node);

    // iterate directory
    iterate_directory(folder_path, &None, &mut arena_tree);

    Ok(arena_tree)
}

fn iterate_directory(folder_path: &Path, previous_node: &Option<i32>, arena_tree: &mut ArenaTree) {
    let folder_node: Node = Node::new(get_path_name(folder_path), String::from(""));
    let folder_node_id: i32;

    if previous_node.is_none() {
        if arena_tree.get_root_node().is_none() {
           folder_node_id = arena_tree.add_root_node(folder_node); 
        }else {
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
        } 
        else {
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
