use load_env::schema::EnvSchema;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::{error::Error, fs::File, io::Read};
use tonic::{Code, Status};

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

pub async fn get_db() -> Result<sqlx::Pool<sqlx::Postgres>, Status> {
    // let db_url = EnvSchema::new().unwrap().db_url();
    // TODO: omit
    // tracing::info!("Connecting to database @ {}", &db_url);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost/mydatabase")
        .await
        .map_err(|_| Status::new(Code::Unavailable, "couldn't connect to database"))?;
    Ok(pool)
}
