use super::exists;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use std::fs;
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub key: String,
    pub path: String,
}
pub fn read_config() -> Config {
    if !exists("./config.json".to_string()) {
        fs::write(
            "./config.json",
            format!(
                "{{ \"key\": \"{}\",\n\"path\": \"./passwords.json\"}}",
                random()
            ),
        )
        .expect("error writing file");
    }
    let config: String = fs::read_to_string("./config.json").expect("Unable to read config file");
    let cfg: Config = serde_json::from_str(&config).unwrap();
    cfg
}
pub fn random() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 256)
}
