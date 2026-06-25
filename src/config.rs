use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub enabled: bool,
    pub warning_threshold: f64,
    pub regrets: Vec<Regret>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled: true,
            warning_threshold: 0.75,
            regrets: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Regret {
    pub command: String,
    #[serde(default)]
    pub reason: Reason,
    pub timestamp: Timestamp,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Reason(pub Option<String>);

impl Reason {
    pub fn get(&self) -> String {
        self.0.clone().unwrap_or("None".to_string())
    }
}

impl std::fmt::Display for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.get())
    }
}

/* Since std::time::SystemTime does not implement std::fmt::Display, create a new struct that
 * uses SystemTime as a field and implement display for that struct instead.
 */
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Timestamp(pub std::time::SystemTime);

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", humantime::format_rfc3339_seconds(self.0))
    }
}

pub fn config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "v81d", "nope") {
        return Ok(proj_dirs.config_dir().to_owned());
    }

    Err("Failed to get project directory.".into())
}

pub fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let config_path = Path::join(&config_dir().unwrap(), "config.toml");

    // Create all if the parent config directory is not present
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    Ok(config_path)
}

pub fn get_config_file() -> Result<File, Box<dyn std::error::Error>> {
    Ok(OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(get_config_path().unwrap())
        .unwrap())
}

pub fn read_file(file: &mut File) -> Result<String, Box<dyn std::error::Error>> {
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    Ok(data)
}

pub fn get_config(data: &str) -> Result<Config, Box<dyn std::error::Error>> {
    Ok(if data.trim().is_empty() {
        Config::default()
    } else {
        toml::from_str(data).unwrap()
    })
}

pub fn set_enabled(enabled: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = get_config_file().unwrap();
    let data = read_file(&mut file).unwrap();
    let mut config = get_config(&data).unwrap();

    config.enabled = enabled;

    file.seek(SeekFrom::Start(0)).unwrap();
    file.set_len(0).unwrap();
    file.write_all(toml::to_string(&config).unwrap().as_bytes())
        .unwrap();

    Ok(())
}

pub fn set_warning_threshold(threshold: f64) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = get_config_file().unwrap();
    let data = read_file(&mut file).unwrap();
    let mut config = get_config(&data).unwrap();

    config.warning_threshold = threshold;

    file.seek(SeekFrom::Start(0)).unwrap();
    file.set_len(0).unwrap();
    file.write_all(toml::to_string(&config).unwrap().as_bytes())
        .unwrap();

    Ok(())
}

pub fn list_regrets() -> Result<Vec<Regret>, Box<dyn std::error::Error>> {
    let mut file = get_config_file().unwrap();
    let data = read_file(&mut file).unwrap();
    let config = get_config(&data).unwrap();
    Ok(config.regrets)
}

pub fn add_regret(regret: Regret) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = get_config_file().unwrap();
    let data = read_file(&mut file).unwrap();
    let mut config = get_config(&data).unwrap();

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

pub fn remove_regret(id: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = get_config_file().unwrap();
    let data = read_file(&mut file).unwrap();
    let mut config = get_config(&data).unwrap();

    config.regrets.remove(id); // remove at index

    file.seek(SeekFrom::Start(0)).unwrap();
    file.set_len(0).unwrap();
    file.write_all(toml::to_string(&config).unwrap().as_bytes())
        .unwrap();

    Ok(())
}

pub fn clear_regrets() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = get_config_file().unwrap();
    let data = read_file(&mut file).unwrap();

    if !data.trim().is_empty() {
        let mut config: Config = toml::from_str(&data).unwrap();
        config.regrets = Vec::new(); // reset regrets vector

        file.seek(SeekFrom::Start(0)).unwrap();
        file.set_len(0).unwrap();
        file.write_all(toml::to_string(&config).unwrap().as_bytes())
            .unwrap();
    };

    Ok(())
}

pub fn get_regret(id: usize) -> Result<Regret, Box<dyn std::error::Error>> {
    let mut file = get_config_file().unwrap();
    let data = read_file(&mut file).unwrap();
    let config = get_config(&data).unwrap();
    Ok(config.regrets[id].clone())
}
