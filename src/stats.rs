use std::{fs::{self, File}, path::Path};

use crate::stats_file::StatsJson;

pub struct Stats {
    stats_file: StatsJson
}

impl Stats {
    pub fn new() -> Self {
        let path = "./data/stats.json";
        let json_file: StatsJson;

        // Attempt to load from JSON file.
        match fs::read_to_string(path) {
            // The file exists and data was successfully retrieved.
            Ok(str) => json_file = StatsJson::from(str),
            Err(ex) => {
                // If the data directory doesn't exist, go ahead and create it.
                if !Path::new("./data").exists() {
                    match fs::create_dir("./data") {
                        Ok(_) => {},
                        Err(e) => panic!("{}", e),
                    }
                }

                // If the file already doesn't exist, go ahead and create it then populate it.
                if !Path::new(path).exists() {
                    match File::create(path) {
                        // Populate the file
                        Ok(_) => json_file = StatsJson::new(),
                        Err(e) => panic!("{}", e),
                    }
                } else {
                    // If none of that occurs, there must be some other error.
                    panic!("{}", ex)
                }
            }
        }

        Stats { stats_file: json_file }
    }

    pub fn inc_wins(&mut self) -> u8 {
        self.stats_file.inc_wins();
        self.stats_file.get_wins().try_into().unwrap()
    }

    pub fn inc_losses(&mut self) -> u8 {
        self.stats_file.inc_losses();
        self.stats_file.get_losses().try_into().unwrap()
    }

    pub fn inc_draws(&mut self) -> u8 {
        self.stats_file.inc_draws();
        self.stats_file.get_draws().try_into().unwrap()
    }

    pub fn print(&self) {
        println!("Wins: {}\nLosses: {}\nDraws: {}\n", self.stats_file.get_wins(), self.stats_file.get_losses(), self.stats_file.get_draws())
    }
}