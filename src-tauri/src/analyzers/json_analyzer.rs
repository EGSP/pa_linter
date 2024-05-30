use serde_json::{Map, Value};

use crate::{
    analyzer::{AnalysisResult, Tip},
    nodes::NodeId,
    project::project::Project,
};

pub struct JsonAnalyzeTask<'a> {
    pub project: &'a Project,
}

impl<'a> JsonAnalyzeTask<'a> {
    pub fn new(project: &Project) -> JsonAnalyzeTask {
        JsonAnalyzeTask { project }
    }

    pub fn run(&self) -> Vec<AnalysisResult> {
        let jsons = Self::get_all_jsons(&self.project);
        Self::analyze_jsons(self, &self.project, &jsons)
    }

    fn get_all_jsons(project: &Project) -> Vec<i32> {
        let nodes = project.arena_tree.nodes_map.values();

        nodes
            .filter(|node| node.value.ends_with(".json"))
            .map(|node| node.id)
            .collect()
    }

    fn analyze_jsons(&self, project: &Project, json_nodes: &Vec<i32>) -> Vec<AnalysisResult> {
        let mut analyzis_results: Vec<AnalysisResult> = Vec::new();
        for node_id in json_nodes {
            let json = project.read_json_node(node_id.clone());
            let path = project.get_node_absolute_path(node_id.clone());

            if json.is_err() {
                continue;
            }
            let tips = Self::analyze_json(self, &json.unwrap());

            analyzis_results.push(AnalysisResult::new(path, tips))
        }

        analyzis_results
    }

    fn analyze_json(&self, json: &serde_json::Value) -> Vec<Tip> {
        let mut tips = Vec::new();
        tips.append(&mut Self::iterate_json_value(
            self,
            &String::from("root"),
            json,
        ));
        tips
    }

    fn iterate_json_value(&self, v_keyname: &String, v: &serde_json::Value) -> Vec<Tip> {
        let tips = match v {
            Value::String(s) => Self::analyze_json_string(self, &v_keyname, s),
            Value::Object(o) => Self::iterate_json_object(self, &format!("{{{}}}", v_keyname), o),
            Value::Array(a) => Self::iterate_json_array(self, &format!("[{}]", v_keyname), a),
            _ => Vec::new(),
        };

        tips
    }

    fn iterate_json_object(&self, o_keyname: &String, o: &Map<String, Value>) -> Vec<Tip> {
        let mut tips = Vec::new();
        //println!("{}:", o_keyname);
        for (keyname, v) in o {
            tips.append(&mut Self::iterate_json_value(self, keyname, v))
        }

        tips
    }

    fn iterate_json_array(&self, a_keyname: &String, a: &Vec<Value>) -> Vec<Tip> {
        let mut tips = Vec::new();

        //println!("{}:", a_keyname);
        let mut i = 0;
        for v in a {
            let v_keyname = &(a_keyname.to_owned() + &format!("[{}]", i));
            tips.append(&mut Self::iterate_json_value(self, v_keyname, v));
            i += 1;
        }

        tips
    }

    /// Analyzes JSON string `s` and returns a vector of tips.
    ///
    /// This function checks if the string is a valid relative path
    /// to a JSON file. It checks if the string starts with a slash
    /// and if it contains only forward slashes. If it does not, it
    /// returns a vector of tips. If it does, it checks if it ends
    /// with the ".json" extension and if it does not, it returns a
    /// tip.
    fn analyze_json_string(&self, s_keyname: &String, s: &String) -> Vec<Tip> {
        if !Self::has_slash(s) {
            return Vec::new(); // not a relative path
        }

        let mut tips = Vec::new();
        if !Self::has_first_slash(s) {
            tips.push(Tip::new(
                s_keyname.to_string(),
                s.to_string(),
                "Missing leading slash".to_string(),
            ));
        }

        if Self::has_incorrect_slash(s) {
            tips.push(Tip::new(
                s_keyname.to_string(),
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

        let dependency_path = fixed_string.to_string();
        let does_exist = self.project.find_file_by_relative_path(&dependency_path);

        if does_exist.is_err() {
            tips.push(Tip::new(
                s_keyname.to_string(),
                s.to_string(),
                format!(
                    "File does not exist even with correct path: {}",
                    &dependency_path
                ),
            ));
        }

        tips
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
}
