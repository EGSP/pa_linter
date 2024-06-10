use std::{fs, io::Read};

use serde_json::{Map, Value};

use crate::{
    analyzer::{AnalysisResult, Tip},
    editor::editor_runtime::EditorRuntimeData,
    nodes::NodeId,
    project::{
        project::Project,
        repos::{
            repository::RepositoryInfo,
            repository_tree::{
                get_entry_relative_path, RelativePath, RepositoryTree, RepositoryTreeEntry,
            },
        },
    },
};

use super::analyzer::{AnalysisMessage, FileAnalysisResult};

pub struct JsonAnalyzeTask<'a> {
    editor_runtime_data: &'a EditorRuntimeData,
}

impl<'a> JsonAnalyzeTask<'a> {
    pub fn new(editor_runtime_data: &'a EditorRuntimeData) -> JsonAnalyzeTask {
        JsonAnalyzeTask {
            editor_runtime_data,
        }
    }

    pub fn run(&self) -> Vec<AnalysisResult> {
        let jsons = Self::get_all_jsons(&self.project);
        Self::analyze_jsons(self, &self.project, &jsons)
        // Пройтись по каждому дереву

        // Сделать функцию для поиска файлов в других репозиториях

        // В модуле дерева сделать функцию для поиска файла поотносительному пути.
    }

    fn analyze_repository_trees(&self) -> Vec<FileAnalysisResult> {
        let mut results: Vec<FileAnalysisResult> = Vec::new();
        for repository_tree in &self.editor_runtime_data.repository_trees {
            let json_entries: Vec<&RepositoryTreeEntry> = repository_tree
                .entries
                .iter()
                .filter(|entry| entry.path.ends_with(".json"))
                .collect();

            for entry in json_entries {
                let result = self.analyze_repository_entry(&repository_tree, entry);

                if let Some(result) = result {
                    results.push(result);
                }
            }
        }

        return results;
    }

    fn analyze_repository_entry(
        &self,
        repository_tree: &RepositoryTree,
        entry: &RepositoryTreeEntry,
    ) -> Option<FileAnalysisResult> {
        let path = entry.path.clone();
        if !path.ends_with(".json") {
            return None;
        }

        let content = fs::read_to_string(&path).unwrap();
        let json = serde_json::from_str(&content).unwrap();

        let messages = self.analyze_json(&json);
        Some(FileAnalysisResult {
            file_path: path,
            repository_info: repository_tree.repository_info.clone(),
            messages,
        })
    }

    fn analyze_json(&self, json: &serde_json::Value) -> Vec<AnalysisMessage> {
        let mut messages: Vec<AnalysisMessage> = Vec::new();
        iterate_json_value(&mut messages, &String::from("json"), json);

        fn iterate_json_value(
            messages: &mut Vec<AnalysisMessage>,
            v_keyname: &String,
            v: &serde_json::Value,
        ) {
            match v {
                Value::String(s) => analyze_json_string(messages, &v_keyname, s),
                Value::Object(o) => iterate_json_object(messages, &format!("{{{}}}", v_keyname), o),
                Value::Array(a) => iterate_json_array(messages, &format!("[{}]", v_keyname), a),
                Value::Null => return,
                Value::Bool(_) => return,
                Value::Number(_) => return,
            };
        }

        fn iterate_json_object(
            messages: &mut Vec<AnalysisMessage>,
            property_name: &String,
            o: &Map<String, Value>,
        ) {
            //println!("{}:", o_keyname);
            for (keyname, v) in o {
                iterate_json_value(messages, keyname, v)
            }
        }

        fn iterate_json_array(
            messages: &mut Vec<AnalysisMessage>,
            property_name: &String,
            a: &Vec<Value>,
        ) {
            //println!("{}:", a_keyname);
            let mut i = 0;
            for v in a {
                let array_value_name = &(property_name.to_owned() + &format!("[{}]", i));
                iterate_json_value(messages, array_value_name, v);
                i += 1;
            }
        }

        /// Analyzes JSON string `s` and returns a vector of tips.
        ///
        /// This function checks if the string is a valid relative path
        /// to a JSON file. It checks if the string starts with a slash
        /// and if it contains only forward slashes. If it does not, it
        /// returns a vector of tips. If it does, it checks if it ends
        /// with the ".json" extension and if it does not, it returns a
        /// tip.
        fn analyze_json_string(
            messages: &mut Vec<AnalysisMessage>,
            property_name: &String,
            s: &String,
        ) {
            if !has_slash(s) {
                return; // not a relative path
            }

            if !has_first_slash(s) {
                messages.push(AnalysisMessage::new(
                    property_name.to_string(),
                    s.to_string(),
                    "Missing leading slash".to_string(),
                ));
            }

            if has_incorrect_slash(s) {
                tips.push(Tip::new(
                    property_name.to_string(),
                    s.to_string(),
                    "Incorrect slash".to_string(),
                ));
            }

            // TODO: Сделать продвинутую проверку для разных типов файлов: джсон, звуки, эффекты
            let file_type = s.split('.').last().unwrap();
            if file_type != "json" {
                return tips;
            }

            // ПРОВЕРКА СВЯЗЕЙ
            // мы умные, поэтому проверять связи нужно сразу с исправленой строкой
            let mut fixed_string = s.replace("\\", "/");
            if !fixed_string.starts_with("/") {
                fixed_string = "/".to_owned() + &fixed_string;
            }

            let path_value = fixed_string.to_string();
            let does_exist = self.project.find_file_by_relative_path(&path_value);

            if does_exist.is_err() {
                tips.push(Tip::new(
                    property_name.to_string(),
                    s.to_string(),
                    format!(
                        "File does not exist even with correct path: {}",
                        &path_value
                    ),
                ));
            }

            tips
        }

        return messages;
    }
}

fn has_slash(s: &str) -> bool {
    s.contains('/') || s.contains('\\')
}

fn has_incorrect_slash(s: &str) -> bool {
    s.contains('\\')
}

/// Returns true if `s` starts with '/'.
///
/// This function is used to check if a given path already has a leading slash.
/// If it does, we don't need to prepend another slash, otherwise we do.
fn has_first_slash(s: &str) -> bool {
    s.starts_with('/')
}
