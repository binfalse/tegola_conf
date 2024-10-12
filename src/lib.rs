use std::error::Error;
use std::fs::{self, File};
use std::io::Write;

use models::config::Config;

pub mod models;

/// read a tegola config from a file
pub fn tegola_conf_read(path: String) -> Result<Config, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    Ok(toml::from_str(&contents)?)
}

/// write a tegola config to a file
pub fn tegola_conf_write(file: &mut File, config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = toml::to_string(config)?;
    write!(file, "{}", contents)?;
    Ok(())
}
