use std::{fs};

pub struct JsonFile {
    dir: String,
    file_name: String,
    current_content: serde_json::Value
}

#[derive(Debug)]
pub enum JSONError {
    UnableToUpdateFile(std::io::Error)
}

impl JsonFile {
    pub fn new(dir: &str, file_name: &str) -> Self {
        JsonFile { dir: dir.to_string(), file_name: file_name.to_string(), current_content: serde_json::from_str("{}").unwrap() }
    }

    pub fn get_path(&self) -> String {
        return self.dir.clone().to_string() + "/" + &self.file_name + ".json"
    }

    pub fn get(&self, key: String) -> serde_json::Value {
        self.current_content[key].clone()
    }

    pub fn set_json(&mut self, content: serde_json::Value) {
        self.current_content = content;
    }

    pub fn get_i8(&self, key: &str) -> i8 {
        self.get(key.to_string()).as_i64().unwrap().try_into().unwrap()
    }

    pub fn set(&mut self, key: String, value: serde_json::Value) {
        self.current_content[key] = value;
    }

    pub fn update_json(&self) -> Result<(), JSONError> {
        match fs::write(self.get_path(), self.current_content.to_string()) {
            Ok(_) => Ok(()),
            Err(e) => Err(JSONError::UnableToUpdateFile(e))
        }
    }
}