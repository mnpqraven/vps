use apalis::prelude::*;
use apalis_cron::{CronStream, Schedule};
use chrono::Local;
use cron_ddns::{update_cf_conf, utils::error::CronDdnsError};
use std::str::FromStr;
use tower::load_shed::LoadShedLayer;

#[tokio::main]
async fn main() -> Result<(), CronDdnsError> {
    tracing_subscriber::fmt::init();

    // https://www.freetool.dev/crontab-generator/
    let schedule = "0 0/30 * * * *";

    let schedule = Schedule::from_str(schedule)
        .map_err(|_e| CronDdnsError::Unknown(format!("Bad cron string {schedule}")))?;

    let worker = WorkerBuilder::new("cron-ddns")
        .enable_tracing()
        // Important when you have layers that block the service
        .layer(LoadShedLayer::new())
        .backend(CronStream::new_with_timezone(schedule, Local))
        .build_fn(update_cf_conf);
    worker.run().await;

    Ok(())
}
