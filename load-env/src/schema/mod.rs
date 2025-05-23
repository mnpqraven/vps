use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::fs::read_to_string;
use std::path::Path;
use std::path::PathBuf;
use tracing::info;

use crate::utils::EnvError;

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct EnvSchema {
    pub database: EnvSchemaDatabase,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct EnvSchemaDatabase {
    user: String,
    password: String,
    database_entrypoint: String,
}

impl EnvSchema {
    pub fn new() -> Result<Self, EnvError> {
        let cargo_dir = env::var("CARGO_MANIFEST_DIR");
        if cargo_dir.is_err() {
            // FIXME: graceful handling
            return Err(EnvError::FileNotFound("manifest dir not found".to_string()));
        }
        let cargo_dir = cargo_dir.unwrap();

        let crate_path = PathBuf::from(cargo_dir);
        // parent cause we are in /load-env rn
        let crate_path = crate_path.parent().unwrap();

        let conf_str = source_filename(crate_path)?;
        info!("[CONFIG] using config from {}", &conf_str);
        let env = toml::from_str::<EnvSchema>(&conf_str)?;

        Ok(env)
    }

    pub fn db_url(self) -> String {
        let EnvSchemaDatabase {
            user,
            password,
            database_entrypoint,
        } = self.database;
        format!("postgres://{user}:{password}@localhost/{database_entrypoint}")
    }
}

impl Default for EnvSchemaDatabase {
    fn default() -> Self {
        Self {
            user: "postgres".into(),
            password: "postgres".into(),
            database_entrypoint: "mydatabase".into(),
        }
    }
}

fn source_filename<T: AsRef<Path>>(dir: T) -> Result<String, EnvError> {
    // TODO: search by regex
    let path = dir.as_ref().join("config.toml");

    info!("{:?}", path);
    let conf = read_to_string(path)?;
    Ok(conf)
}

fn legit_names<T: AsRef<Path>>(conf_path: T) -> bool {
    let conf_path = conf_path.as_ref();
    let ext = conf_path.extension();
    match ext {
        None => false,
        Some(ext) if ext.ne("toml") => false,
        Some(_) => {
            let filename = conf_path
                .file_stem()
                .expect("valid extension = valid stem")
                .to_string_lossy();

            // FIXME: regex
            filename.starts_with("Config") || filename.starts_with("config")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EnvSchema;
    use crate::schema::legit_names;
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
            assert!(legit_names(name))
        }
        for name in bad_names.iter() {
            assert!(legit_names(name));
        }
    }

    #[test]
    fn example_equals_default() {
        // path fn to owned
        let example = read_to_string("../../../Config.example.toml").unwrap();
        let parsed = toml::from_str::<EnvSchema>(&example).unwrap();
        let default = EnvSchema::default();
        assert_eq!(default, parsed);
    }
}
