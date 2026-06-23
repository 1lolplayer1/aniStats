mod api;
mod checker;
mod config;
mod monitor;
mod types;

use api::create_router;
use config::Config;
use monitor::{AppState, spawn_monitor_task};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. load config
    let config = Config::load()?;

    // 2. build shared HTTP client
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (compatible)")
        .build()?;

    // 3. initialize empty shared state
    let state: AppState = Arc::new(RwLock::new(HashMap::new()));

    // 4. spawn one monitor task per site
    for site in &config.sites {
        let interval = site
            .interval_secs
            .unwrap_or(config.monitor.default_interval_secs);

        spawn_monitor_task(
            site.clone(),
            Arc::clone(&state),
            client.clone(),
            config.monitor.request_timeout_secs,
            interval,
            config.monitor.history_limit,
        );
    }

    // 5. start API server
    let router = create_router(Arc::clone(&state));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    println!("Monitor running — API at http://localhost:3000");
    axum::serve(listener, router).await?;

    Ok(())
}
