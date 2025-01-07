use serde_json;
use std::env::current_exe;
use std::fs;

#[derive(Default)]
pub struct ShutdownCommands {
    pub shutdown: String,
    pub sleep: String,
    pub lock: String,
    pub logoff: String,
    pub restart: String,
    pub config_success: bool,
}

impl ShutdownCommands {
    pub fn new() -> ShutdownCommands {
        // Parse json here
        let file = fs::read_to_string(format!(
            "{}{}config.json",
            current_exe().unwrap().parent().unwrap().display(),
            if cfg!(target_os = "windows") {"\\"} else {"/"}
        ));

        if file.is_err() {
            Self::default()
        } else {
            let file = file.unwrap();
            let serialization: Result<serde_json::Value, serde_json::Error> =
                serde_json::from_str(file.as_str());

            if serialization.is_err() {
                return Self::default()
            }
            let serialization = serialization.unwrap();

            Self {
                shutdown: serialization["shutdown"].as_str().unwrap().to_string(),
                sleep: serialization["sleep"].clone().as_str().unwrap().to_string(),
                lock: serialization["lock"].clone().as_str().unwrap().to_string(),
                logoff: serialization["logoff"].clone().as_str().unwrap().to_string(),
                restart: serialization["restart"].clone().as_str().unwrap().to_string(),
                config_success: true,
            }
        }
    }
}
