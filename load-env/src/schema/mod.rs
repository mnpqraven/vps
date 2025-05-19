use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::fs::read_to_string;
use std::path::Path;
use std::path::PathBuf;
use tracing::info;

use crate::utils::EnvError;

#[derive(Serialize, Deserialize, Default)]
pub struct EnvSchema {
    pub database: EnvSchemaDatabase,
}

#[derive(Serialize, Deserialize)]
pub struct EnvSchemaDatabase {
    pub url: String,
    pub auth_token: String,
    /// see https://docs.turso.tech/libsql#encryption-at-rest
    pub enc_key: String,
    pub local_database_path: String,
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
}

impl Default for EnvSchemaDatabase {
    fn default() -> Self {
        Self {
            url: "http://127.0.0.1:4010".into(),
            auth_token: String::new(),
            enc_key: "your_encryption_key".into(),
            // TODO:
            local_database_path: "/tmp/vps_database.db".into(),
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
    use crate::schema::legit_names;

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
        let good_pass = good_names.iter().all(legit_names);
        assert!(good_pass);
        let bad_pass = bad_names.iter().all(|filename| !legit_names(filename));
        assert!(bad_pass);
    }
}
