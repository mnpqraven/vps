use apalis::prelude::*;
use apalis_cron::{CronContext, CronStream, Schedule};
use chrono::Local;
use cron_ddns::{update_cf_conf, utils::error::CronDdnsError};
use std::str::FromStr;
use tower::load_shed::LoadShedLayer;
use tracing::{info, instrument};

#[tokio::main]
async fn main() -> Result<(), CronDdnsError> {
    tracing_subscriber::fmt::init();

    // https://www.freetool.dev/crontab-generator/
    let schedule = "* * * * * *";

    let schedule = Schedule::from_str(schedule)
        .map_err(|_e| CronDdnsError::Unknown("Bad cron string {schedule}".into()))?;

    let worker = WorkerBuilder::new("cron-ddns")
        .enable_tracing()
        // Important when you have layers that block the service
        .layer(LoadShedLayer::new())
        .backend(CronStream::new_with_timezone(schedule, Local))
        .build_fn(send_reminder);
    worker.run().await;

    Ok(())
}

#[derive(Debug, Default)]
struct Reminder;

#[instrument(ret)]
async fn send_reminder(_job: Reminder, ctx: CronContext<Local>) -> Result<(), CronDdnsError> {
    let time = ctx.get_timestamp();
    info!("[CRON] timestamp: {}", time);
    update_cf_conf().await?;
    Err(CronDdnsError::Unknown("WIP".into()))
}
