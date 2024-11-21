use std::{error::Error, fs::File, io::Read};

use serde::{Deserialize, Serialize};

/// current Config.toml shape
/// [general]
/// home = "./"

#[derive(Debug, Serialize, Deserialize)]
pub struct VpsConfig {
    pub general: VpsConfigGenral,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VpsConfigGenral {
    pub home: String,
}

pub fn read_config_toml() -> Result<VpsConfig, Box<dyn Error>> {
    let path = "./Config.toml";

    let mut f = File::open(path)?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    let config: VpsConfig = toml::from_str(&buffer)?;

    Ok(config)
}
