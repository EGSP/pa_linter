use serde_json::Value;
use serde_json::Map;

#[derive(Debug)]
pub struct Tip {
    pub property_name: String,
    pub property_value: String,
    pub message: String,
}

impl Tip {
    pub fn new(property_name: String, property_value: String, message: String) -> Self {
        Self {
            property_name,
            property_value,

            message,
        }
    }
}

pub fn analyze_json(json: &serde_json::Value) -> Vec<Tip> {
    let mut tips = Vec::new();
    tips.append(&mut iterate_json_value(&String::from("root"), json));
    tips 
}

fn iterate_json_value(v_keyname: &String, v: &serde_json::Value) -> Vec<Tip> {
    let mut tips = match v {
        Value::String(s) => analyze_json_string(&v_keyname, s),
        Value::Object(o) => iterate_json_object(&format!("{{{}}}", v_keyname), o),
        Value::Array(a) => iterate_json_array(&format!("[{}]", v_keyname), a),
        _ => Vec::new(),
    };

    tips
}

fn iterate_json_object(o_keyname: &String, o: &Map<String, Value>) -> Vec<Tip> {
    let mut tips = Vec::new();
    //println!("{}:", o_keyname);
    for (keyname, v) in o {
        tips.append(&mut iterate_json_value(keyname, v))
    }

    tips
}

fn iterate_json_array(a_keyname: &String, a: &Vec<Value>) -> Vec<Tip> {
    let mut tips = Vec::new();

    //println!("{}:", a_keyname);
    let mut i = 0;
    for v in a {
        let v_keyname = &(a_keyname.to_owned() + &format!("[{}]", i));
        tips.append(&mut iterate_json_value(v_keyname, v));
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
fn analyze_json_string(s_keyname: &String, s: &String) -> Vec<Tip> {
    if (!has_slash(s)) {
        return Vec::new(); // not a relative path to a JSON file
    }

    let mut tips = Vec::new();
    if(!has_first_slash(s)) {
        tips.push(Tip::new(s_keyname.to_string(), s.to_string(), "Missing leading slash".to_string()));
    }

    if(has_incorrect_slash(s)) {
        tips.push(Tip::new(s_keyname.to_string(), s.to_string(), "Incorrect slash".to_string()));
    }

    // Сделать продвинутую проверку для разных типов файлов: джсон, звуки, эффекты
    // Не у всех путей есть расширения, но у всех есть корневые папки
    // if(!has_json_extension(s)) {
    //     tips.push(Tip::new(s_keyname.to_string(), s.to_string(), "Missing .json extension".to_string()));
    // }

    tips
}

/// Returns true if `s` contains either '/' or '\'.
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

/// Prepends a slash to `s` if it doesn't have one already.
///
/// This function is used to normalize paths by adding a leading slash if it's missing.
fn prepend_slash(s: &str) -> String {
    if has_first_slash(s) {
        s.to_string()
    } else {
        format!("/{}", s)
    }
}

/// Replaces '\' with '/' in `s` if it contains '/' or '\'.
/// Otherwise returns a clone of `s`.
fn replace_slash(s: &str) -> String {
    if has_slash(s) {
        s.replace('\\', "/")
    } else {
        // No '\' or '/' found, so return a clone of `s`.
        s.to_string()
    }
}

fn has_json_extension(s: &str) -> bool {
    s.ends_with(".json")
}


// fn iterate_json_value(v_keyname: &String, v: &serde_json::Value) {
//     match v {
//         Value::String(s) => println!("{}: {}", v_keyname, s),
//         Value::Object(o) => iterate_json_object(&format!("{{{}}}",v_keyname), o),
//         Value::Array(a) => iterate_json_array(&format!("[{}]", v_keyname), a),
//         Value::Number(n) => println!("{}: {}", v_keyname, n),
//         Value::Bool(b) => println!("{}: {}", v_keyname, b),
//         Value::Null => println!("{}: null", v_keyname),
//         _ => println!("parse error: {}", v_keyname),
//     }
// }