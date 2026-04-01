use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use tabled::Tabled;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    pub regrets: Vec<Regret>,
}

#[derive(Tabled, Serialize, Deserialize, Debug)]
pub struct Regret {
    pub command: String,
    pub reason: String,
    pub timestamp: Timestamp,
}

/* Since std::time::SystemTime does not implement std::fmt::Display, create a new struct that
 * uses SystemTime as a field and implement display for that struct instead.
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Timestamp(pub std::time::SystemTime);

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", humantime::format_rfc3339_seconds(self.0))
    }
}

fn config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "v81d", "nope") {
        return Ok(proj_dirs.config_dir().to_owned());
    }

    Err("Failed to get project directory.".into())
}

fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let config_path = Path::join(&config_dir().unwrap(), "config.toml");

    // Create all if the parent config directory is not present
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    Ok(config_path)
}

pub fn list_regrets() -> Result<Vec<Regret>, Box<dyn std::error::Error>> {
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

    Ok(config.regrets)
}

pub fn add_regret(regret: Regret) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(get_config_path().unwrap())
        .unwrap();

    let mut data = String::new();
    file.read_to_string(&mut data).unwrap(); // data as string

    let mut config: Config = if data.trim().is_empty() {
        Config {
            regrets: Vec::new(),
        }
    } else {
        toml::from_str(&data).unwrap() // config as Vec<Regret>
    };

    // If any regret in config has the same command as the one to be added, return an error
    if config.regrets.iter().any(|r| r.command == regret.command) {
        return Err("Regret already exists in configuration.".into());
    }

    config.regrets.push(regret);

    file.seek(SeekFrom::Start(0)).unwrap(); // cursor to start
    file.set_len(0).unwrap(); // truncate/clear
    file.write_all(toml::to_string(&config).unwrap().as_bytes())
        .unwrap();

    Ok(())
}

pub fn remove_regret(command: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(get_config_path().unwrap())
        .unwrap();

    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let mut config: Config = if data.trim().is_empty() {
        Config {
            regrets: Vec::new(),
        }
    } else {
        toml::from_str(&data).unwrap()
    };

    config.regrets.retain(|r| r.command != command); // retain non-matching commands

    file.seek(SeekFrom::Start(0)).unwrap();
    file.set_len(0).unwrap();
    file.write_all(toml::to_string(&config).unwrap().as_bytes())
        .unwrap();

    Ok(())
}
