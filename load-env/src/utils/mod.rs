pub mod path;

#[derive(Debug)]
pub enum EnvError {
    // TODO: more idiomatic
    Parse(String),
    FileNotFound(String),
}

impl From<std::io::Error> for EnvError {
    fn from(value: std::io::Error) -> Self {
        Self::FileNotFound(value.to_string())
    }
}

impl From<toml::de::Error> for EnvError {
    fn from(value: toml::de::Error) -> Self {
        Self::Parse(value.to_string())
    }
}
