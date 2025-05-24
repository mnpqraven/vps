use thiserror::Error;

pub mod filename_resolve;
pub mod path;

#[derive(Error, Debug)]
pub enum EnvError {
    // TODO: more idiomatic
    #[error("failed to parse env at {0}")]
    Parse(#[from] toml::de::Error),

    #[error("config file does not exist at {0}")]
    FileNotFound(String),

    #[error("no suitable configuration env found, is this not running as an user ?")]
    NoSuitableConfigDir,
}

impl From<std::io::Error> for EnvError {
    fn from(value: std::io::Error) -> Self {
        Self::FileNotFound(value.to_string())
    }
}
