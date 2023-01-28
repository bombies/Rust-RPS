use serde_json::json;

use crate::json_file;

pub struct StatsJson {
    json_file: json_file::JsonFile,
}

impl StatsJson {
    pub fn new() -> Self {
        let mut ret = StatsJson { json_file: json_file::JsonFile::new("data", "stats") };
        ret.json_file.set("wins".to_string(), json!(0));
        ret.json_file.set("losses".to_string(), json!(0));
        ret.json_file.set("draws".to_string(), json!(0));
        match ret.json_file.update_json() {
            Ok(_) => ret,
            Err(e) => panic!("{:?}", e),
        }
    }

    pub fn from(content: String) -> Self {
        let mut ret = StatsJson { json_file: json_file::JsonFile::new("data", "stats") };
        ret.json_file.set_json(serde_json::from_str(content.as_str()).unwrap());
        ret
    }
 
    fn set(&mut self, key: &str, value: serde_json::Value) {
        self.json_file.set(key.to_string(), value);
        match self.json_file.update_json() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }

    pub fn get_wins(&self) -> i8 {
        self.json_file.get_i8("wins")
    }

    pub fn get_losses(&self) -> i8 {
        self.json_file.get_i8("losses")
    }

    pub fn get_draws(&self) -> i8 {
        self.json_file.get_i8("draws")
    }

    pub fn inc_wins(&mut self) {
        self.set("wins", json!(self.get_wins() + 1))
    }

    pub fn inc_losses(&mut self) {
        self.set("losses", json!(self.get_losses() + 1))
    }

    pub fn inc_draws(&mut self) {
        self.set("draws", json!(self.get_draws() + 1))
    }
}