use std::{fs, io::Read, sync::atomic::AtomicI32};

use atomic_counter::{AtomicCounter, RelaxedCounter};
use owo_colors::OwoColorize;
use serde_json::{Map, Value};

use crate::{
    analyzer::{AnalysisResult, Tip},
    analyzers::analyzer::Category,
    editor::editor_runtime::EditorRuntimeData,
    logs::logbox::Logbox,
    nodes::NodeId,
    project::{
        project::Project,
        repos::{
            repository::RepositoryInfo,
            repository_tree::{
                self, get_entry_relative_path, RelativePath, RepositoryTree, RepositoryTreeEntry,
            },
        },
    },
};

use super::analyzer::{FileAnalysisResult, Hint};

pub struct JsonAnalyzeTask<'a> {
    editor_runtime_data: &'a EditorRuntimeData,
}

impl<'a> JsonAnalyzeTask<'a> {
    pub fn new(editor_runtime_data: &'a EditorRuntimeData) -> JsonAnalyzeTask {
        JsonAnalyzeTask {
            editor_runtime_data,
        }
    }

    pub fn run(&self) -> Vec<FileAnalysisResult> {
        let results = self.analyze_repository_trees();
        results
    }

    fn analyze_repository_trees(&self) -> Vec<FileAnalysisResult> {
        let mut results: Vec<FileAnalysisResult> = Vec::new();
        for repository_tree in &self.editor_runtime_data.repository_trees {
            let json_entries: Vec<&RepositoryTreeEntry> = repository_tree
                .entries
                .iter()
                .filter(|entry| entry.path.ends_with(".json"))
                .collect();

            let mut tree_logbox = Logbox::new();
            let mut COUNTER = RelaxedCounter::new(0);
            for entry in json_entries {
                let mut entry_logbox = Logbox::new();
                entry_logbox.push_message(format!(
                    "{}: {:?}",
                    "scanning entry".magenta(),
                    entry.path
                ));
                let timecheck = std::time::Instant::now();

                let result =
                    self.analyze_repository_entry(&repository_tree, entry, &mut entry_logbox);

                let elapsed = timecheck.elapsed().as_secs_f32();

                if (elapsed > 3.0) {
                    entry_logbox.push_message(format!(
                        "scanned entry: {:?} in {} sec",
                        repository_tree::get_entry_relative_path(repository_tree, entry),
                        elapsed.to_string().red()
                    ));
                    // println!("scanned entry: {:?} in {} sec", repository_tree::get_entry_relative_path(repository_tree, entry), elapsed.to_string().red());
                } else {
                    entry_logbox.push_message(format!(
                        "scanned entry: {:?} in {} sec",
                        repository_tree::get_entry_relative_path(repository_tree, entry),
                        elapsed.to_string().blue()
                    ));
                    // println!("scanned entry: {:?} in {} sec", repository_tree::get_entry_relative_path(repository_tree, entry), elapsed.to_string().blue());
                }

                if let Some(result) = result {
                    results.push(result);
                }

                tree_logbox.push_logbox(entry_logbox);

                COUNTER.inc();
                if (COUNTER.get() % 10 == 0) {
                    tree_logbox.push_message(format!("{}: {:?}", "printing 10 entries".on_magenta().bold(), COUNTER.get()));
                    tree_logbox.print();
                }
            }

            tree_logbox.print();
        }

        return results;
    }

    fn analyze_repository_entry(
        &self,
        repository_tree: &RepositoryTree,
        entry: &RepositoryTreeEntry,
        logbox: &mut Logbox,
    ) -> Option<FileAnalysisResult> {
        let path = entry.path.clone();
        if !path.ends_with(".json") {
            return None;
        }

        let content = fs::read_to_string(&path).unwrap();
        let json = serde_json::from_str(&content).unwrap();

        let messages = self.analyze_json(&json, logbox);
        Some(FileAnalysisResult {
            file_path: path,
            repository_info: repository_tree.repository_info.clone(),
            messages,
        })
    }

    fn analyze_json(&self, json: &serde_json::Value, logbox: &mut Logbox) -> Vec<Hint> {
        let mut messages: Vec<Hint> = Vec::new();
        self.iterate_json_value(&mut messages, &String::from("json"), json, logbox);

        return messages;
    }

    fn iterate_json_value(
        &self,
        messages: &mut Vec<Hint>,
        v_keyname: &String,
        v: &serde_json::Value,
        logbox: &mut Logbox,
    ) {
        match v {
            Value::String(s) => self.analyze_json_string(messages, &v_keyname, s, logbox),
            Value::Object(o) => {
                self.iterate_json_object(messages, &format!("{{{}}}", v_keyname), o, logbox)
            }
            Value::Array(a) => {
                self.iterate_json_array(messages, &format!("[{}]", v_keyname), a, logbox)
            }
            Value::Null => return,
            Value::Bool(_) => return,
            Value::Number(_) => return,
        };
    }

    fn iterate_json_object(
        &self,
        messages: &mut Vec<Hint>,
        property_name: &String,
        o: &Map<String, Value>,
        logbox: &mut Logbox,
    ) {
        //println!("{}:", o_keyname);
        for (keyname, v) in o {
            self.iterate_json_value(messages, keyname, v, logbox)
        }
    }

    fn iterate_json_array(
        &self,
        messages: &mut Vec<Hint>,
        property_name: &String,
        a: &Vec<Value>,
        logbox: &mut Logbox,
    ) {
        //println!("{}:", a_keyname);
        let mut i = 0;
        for v in a {
            let array_value_name = &(property_name.to_owned() + &format!("[{}]", i));
            self.iterate_json_value(messages, array_value_name, v, logbox);
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
        &self,
        messages: &mut Vec<Hint>,
        property_name: &String,
        string_value: &String,
        logbox: &mut Logbox,
    ) {
        if !has_slash(string_value) {
            return; // not a relative path
        }

        if !has_first_slash(string_value) {
            messages.push(Hint::JSON {
                category: Category::Warning,
                property_name: property_name.to_string(),
                property_value: string_value.to_string(),
                message: "Missing leading slash".to_string(),
            });
        }

        if has_incorrect_slash(string_value) {
            messages.push(Hint::JSON {
                category: Category::Warning,
                property_name: property_name.to_string(),
                property_value: string_value.to_string(),
                message: "Incorrect slash".to_string(),
            });
        }

        // TODO: Сделать продвинутую проверку для разных типов файлов: джсон, звуки, эффекты
        let property_value_file_type = string_value.split('.').last().unwrap();
        if property_value_file_type != "json" {
            return;
        }

        // ПРОВЕРКА СВЯЗЕЙ
        // мы умные, поэтому проверять связи нужно сразу с исправленой строкой
        let mut fixed_string = string_value.replace("\\", "/");
        if !fixed_string.starts_with("/") {
            fixed_string = "/".to_owned() + &fixed_string;
        }

        let path_value = fixed_string.to_string();
        let Timecheck = std::time::Instant::now();
        let found_result = repository_tree::find_repository_entry(
            &RelativePath::new(path_value.clone()),
            &self.editor_runtime_data.repository_trees,
        );

        let found_result_string = if found_result.is_some() {
            "found"
        } else {
            "not found"
        };
        if (found_result.is_some()) {
            logbox.push_message(format!(
                "{} searched property: {} in {:?} sec. {}: {}",
                "tree".on_blue(),
                path_value.cyan(),
                Timecheck.elapsed().as_secs_f32(),
                "result".on_blue(),
                found_result_string.green().bold()
            ));
        } else {
            logbox.push_message(format!(
                "{} searched property: {} in {:?} sec. {}: {}",
                "tree".on_blue(),
                path_value.cyan(),
                Timecheck.elapsed().as_secs_f32(),
                "result".on_blue(),
                found_result_string.bright_red().italic()
            ));

            let TIMECHECK = std::time::Instant::now();
            let exist_in_images = does_file_exist_in_images(
                &self.editor_runtime_data,
                RelativePath::new(path_value.clone()),
            );
            let ELAPSED = TIMECHECK.elapsed().as_secs_f32();
            if exist_in_images {
                logbox.push_message(format!(
                    "{} searched property: {} in {:?} sec. {}: {}",
                    "image".on_yellow(),
                    path_value.cyan(),
                    ELAPSED,
                    "result".on_yellow(),
                    "found".green().bold()
                ));
            } else {
                logbox.push_message(format!(
                    "{} searched property: {} in {:?} sec. {}: {}",
                    "image".on_yellow(),
                    path_value.cyan(),
                    ELAPSED,
                    "result".on_yellow(),
                    "not found".bright_red().italic()
                ));
            }
        }

        // Добавить проверку на наличие файла в снимках
        if found_result.is_none() {
            messages.push(Hint::JSON {
                category: Category::Warning,
                property_name: property_name.to_string(),
                property_value: string_value.to_string(),
                message: "File not found".to_string(),
            })
        }
    }
}

fn does_file_exist_in_images(
    editor_runtime_data: &EditorRuntimeData,
    file_path: RelativePath,
) -> bool {
    let directory_images = &editor_runtime_data.directory_images;

    for image in directory_images {
        if image.files.contains(&file_path.value.to_string()) {
            return true;
        }
    }

    false
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
