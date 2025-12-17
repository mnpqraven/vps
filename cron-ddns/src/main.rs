use apalis::prelude::*;
use apalis_cron::{CronStream, Schedule};
use chrono::Local;
use cron_ddns::{
    update_cf_conf,
    utils::{error::CronDdnsError, health::CronHealthRpc},
};
use proto_types::service::health_service_server::HealthServiceServer;
use std::str::FromStr;
use tonic::transport::Server;
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

    // TODO: persisting cron jobs for cached results
    // @see https://docs.rs/apalis-cron/latest/apalis_cron/#persisting-cron-jobs
    // apalis impl
    // https://github.com/apalis-dev/apalis-board/blob/main/examples/axum-email-service/src/main.rs
    let cron = async move {
        tracing::info!("[BOOT] cron worker");
        worker.run().await;
        Ok::<(), CronDdnsError>(())
    };

    let cron_rpc = async move {
        tracing::info!("[BOOT] cron rpc");
        let env = load_env::EnvSchema::load().expect("loading env");
        let url = env.rpc.addr(&load_env::schema::RpcTarget::Cron);
        tracing::info!("{url}");
        Server::builder()
            .trace_fn(|_| tracing::debug_span!("cron"))
            .add_service(HealthServiceServer::new(CronHealthRpc))
            .serve(url.parse().expect("parsing url from env"))
            .await
            .expect("starting cron rpc");
        Ok(())
    };

    // run both futures in parallel
    futures::future::try_join(cron, cron_rpc).await?;

    tracing::debug!("[SHUTDOWN] cron");

    Ok(())
}
