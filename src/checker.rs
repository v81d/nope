use crate::config::{Config, Regret, get_config_path};
use std::fs::OpenOptions;
use std::io::Read;

pub fn check_command(command: &str) -> Option<Regret> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(get_config_path().unwrap())
        .unwrap();

    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let config: Config = if data.trim().is_empty() {
        Config {
            regrets: Vec::new(),
        }
    } else {
        toml::from_str(&data).unwrap()
    };

    if let Some(regret) = config.regrets.iter().find(|r| r.command == command) {
        return Some(regret.clone());
    }

    None
}
