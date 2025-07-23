use crate::utils::errors::EnvError;
use crate::utils::filename_resolve::first_legit_file;
use crate::utils::path::get_first_valid_dir;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::fs::read_to_string;
use std::path::PathBuf;
use tracing::instrument;

pub const NAME_REGEX: &str = r"\.?[cC]onfig\.?(dev|production)?\.toml";

/// config file spec
/// filename: `config.toml` (capital `C` or `.config` at the front are acceptable)
/// dir priority:
/// cargo dir -> `~/.config` -> `usr` (TODO)
#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct EnvSchema {
    pub database: EnvSchemaDatabase,
    pub rpc: EnvSchemaRpc,
    pub cloudflare: EnvCloudflare,
    pub frontend: HashMap<String, EnvFrontend>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct EnvSchemaRpc {
    port: i32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct EnvSchemaDatabase {
    user: String,
    password: String,
    database_entrypoint: String,
    blob_storage_path: String,
}

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct EnvCloudflare {
    record_id: String,
    zone_id: String,
    api_token: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct EnvFrontend {
    pub client_id: String,
    pub client_secret: String,
    pub callback_url: String,
}

impl EnvSchemaDatabase {
    pub fn blob_storage(&self) -> Result<PathBuf, EnvError> {
        let maybe_path = self.blob_storage_path.clone();

        match fs::create_dir_all(&maybe_path).is_err() {
            true => Err(EnvError::FileNotFound(format!(
                "blob storage folder: {maybe_path}"
            ))),
            false => Ok(maybe_path.into()),
        }
    }
}

impl EnvSchema {
    #[instrument(ret, level = "debug")]
    pub fn load() -> Result<Self, EnvError> {
        let crate_path = get_first_valid_dir().ok_or(EnvError::NoSuitableConfigDir)?;
        let first_legit_file = first_legit_file(crate_path, true)?;
        let conf_str = read_to_string(first_legit_file.clone()).map_err(|source| EnvError::Io {
            file_name: Some(first_legit_file),
            source,
        })?;

        let env = toml::from_str::<EnvSchema>(&conf_str)?;

        Ok(env)
    }

    pub fn db_url(&self) -> String {
        let EnvSchemaDatabase {
            user,
            password,
            database_entrypoint,
            ..
        } = &self.database;
        format!("postgres://{user}:{password}@localhost/{database_entrypoint}")
    }
}

impl Default for EnvSchemaRpc {
    fn default() -> Self {
        Self { port: 5005 }
    }
}

impl Default for EnvSchemaDatabase {
    fn default() -> Self {
        Self {
            user: "postgres".into(),
            password: "postgres".into(),
            database_entrypoint: "mydatabase".into(),
            blob_storage_path: "/home/othi/.vps/data".into(),
        }
    }
}

impl EnvSchemaRpc {
    pub fn client_url(&self) -> String {
        format!("grpc://127.0.0.1:{}", self.port)
    }
}

#[cfg(test)]
mod tests {
    use super::EnvSchema;
    use crate::utils::{filename_resolve::is_legit_filename, path::get_first_valid_dir};
    use std::fs::read_to_string;

    #[test]
    fn correct_filename_sourcing() {
        let good_names = [
            "config.toml",
            "Config.toml",
            "config.dev.toml",
            "Config.production.toml",
            ".config.toml",
        ];
        let bad_names = [
            "config.example.toml",
            "Config.example.toml",
            "Caroftuynrf.arostuf.toml",
            "arofuytnrft.toml",
            "Config.notarealplatform.toml",
        ];
        for name in good_names.iter() {
            let is_legit = is_legit_filename(name);
            println!("matching {name} with regex gives back {is_legit}");
            assert!(is_legit);
        }
        for name in bad_names.iter() {
            let is_legit = is_legit_filename(name);
            println!("matching {name} with regex gives back {is_legit}");
            assert!(!is_legit);
        }
    }

    #[test]
    fn example_equals_default() {
        // path fn to owned
        let dir = get_first_valid_dir().unwrap();
        let example_path = dir.join("Config.example.toml");
        let example = read_to_string(example_path).unwrap();

        let parsed = toml::from_str::<EnvSchema>(&example).unwrap();
        let default = EnvSchema::default();
        assert_eq!(default, parsed);
    }
}
