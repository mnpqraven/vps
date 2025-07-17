use apalis::prelude::*;
use apalis_cron::{CronContext, CronStream, Schedule};
use chrono::Local;
use std::str::FromStr;
use tower::load_shed::LoadShedLayer;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // https://www.freetool.dev/crontab-generator/
    let schedule = Schedule::from_str("* * * * * *")?;

    let worker = WorkerBuilder::new("cron-ddns")
        .enable_tracing()
        .layer(LoadShedLayer::new()) // Important when you have layers that block the service
        .backend(CronStream::new_with_timezone(schedule, Local))
        .build_fn(send_reminder);
    worker.run().await;

    Ok(())
}

#[derive(Debug, Default)]
struct Reminder;

async fn send_reminder(_job: Reminder, ctx: CronContext<Local>) {
    let time = ctx.get_timestamp();
    info!("[CRON] timestamp: {}", time)
}
