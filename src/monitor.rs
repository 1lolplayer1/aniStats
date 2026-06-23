use crate::checker::check_site;
use crate::config::SiteConfig;
use crate::types::{CheckPoint, CheckResult, SiteStatus};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

pub type AppState = Arc<RwLock<HashMap<String, SiteStatus>>>;

pub fn spawn_monitor_task(
    site: SiteConfig,
    state: AppState,
    client: reqwest::Client,
    timeout_secs: u64,
    interval_secs: u64,
    history_limit: usize,
) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));

        loop {
            interval.tick().await;

            let result = check_site(&client, &site, timeout_secs).await;

            let mut map = state.write().await;
            let entry = map
                .entry(site.name.clone())
                .or_insert_with(|| result.clone());

            // update consecutive_failures based on new status
            match result.status {
                CheckResult::Up => entry.consecutive_failures = 0,
                _ => entry.consecutive_failures += 1,
            }

            // push this check into the history ring buffer
            entry.history.push_back(CheckPoint {
                timestamp: Utc::now(),
                status: result.status.clone(),
                latency_ms: result.latency_ms,
            });

            // keep history bounded
            if entry.history.len() > history_limit {
                entry.history.pop_front();
            }

            // update the live fields
            entry.status = result.status;
            entry.http_code = result.http_code;
            entry.latency_ms = result.latency_ms;
            entry.last_checked = result.last_checked;

            // lock is released here automatically when `map` drops
        }
    });
}
