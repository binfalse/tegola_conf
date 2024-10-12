use std::error::Error;
use std::fs::{read_to_string, File};
use std::io::Write;

use models::config::Config;
use toml::{from_str, to_string};

pub mod models;

/// read a tegola config from a file
pub fn tegola_conf_read(path: &str) -> Result<Config, Box<dyn Error>> {
    let contents = read_to_string(path)?;
    Ok(from_str(&contents)?)
}

/// write a tegola config to a file
pub fn tegola_conf_write(file: &mut File, config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = to_string(config)?;
    write!(file, "{}", contents)?;
    Ok(())
}
