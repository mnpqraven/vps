use crate::utils::error::CronDdnsError;
use apalis_cron::CronContext;
use chrono::Local;
use load_env::EnvSchema;
use tracing::{info, instrument};

mod crons;
pub mod utils;

#[derive(Debug, Default)]
pub struct Reminder;

#[allow(dead_code)]
enum IpKind {
    V4,
    V6,
}

impl IpKind {
    fn dig_args(&self) -> &str {
        match self {
            IpKind::V4 => "-4",
            IpKind::V6 => "-6",
        }
    }
}

/// CRON entry point to update clouflare ns with the current ip address
///
/// this acts as a DDNS as long as the local machine running this cronjob has
/// internet access.
///
/// Calling API documentation can be found here (ARecord schema):
/// https://developers.cloudflare.com/api/resources/dns/subresources/records/methods/update/
#[instrument(ret)]
pub async fn update_cf_conf(_job: Reminder, ctx: CronContext<Local>) -> Result<(), CronDdnsError> {
    let time = ctx.get_timestamp();
    info!("[CRON] timestamp: {}", time);
    let cf_env = EnvSchema::load()?.cloudflare;

    crons::cloudflare::cf_zone_api(cf_env).await
}
