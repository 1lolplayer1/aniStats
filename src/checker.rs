use crate::config::SiteConfig;
use crate::types::{CheckResult, SiteStatus};
use chrono::Utc;
use reqwest::Client;
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use tokio::time::timeout;

pub async fn check_site(client: &Client, site: &SiteConfig, timeout_secs: u64) -> SiteStatus {
    let start = Instant::now();

    let outcome = timeout(
        Duration::from_secs(timeout_secs),
        client.head(&site.url).send(),
    )
    .await;

    let latency_ms = start.elapsed().as_millis() as u64;

    // Match on the two layers of Result
    match outcome {
        Err(_elapsed) => SiteStatus {
            name: site.name.clone(),
            url: site.url.clone(),
            status: CheckResult::Timeout,
            http_code: None,
            latency_ms: None,
            last_checked: Utc::now(),
            consecutive_failures: 0,
            history: VecDeque::new(),
        },

        Ok(Err(_req_err)) => SiteStatus {
            name: site.name.clone(),
            url: site.url.clone(),
            status: CheckResult::Down,
            http_code: None,
            latency_ms: None,
            last_checked: Utc::now(),
            consecutive_failures: 0,
            history: VecDeque::new(),
        },

        Ok(Ok(response)) => {
            let code = response.status().as_u16();
            // 200, 301, 302, 403 all mean the server responded = Up
            let status = match code {
                200..=399 | 403 => CheckResult::Up,
                _ => CheckResult::Down,
            };
            SiteStatus {
                name: site.name.clone(),
                url: site.url.clone(),
                status,
                http_code: Some(code),
                latency_ms: Some(latency_ms),
                last_checked: Utc::now(),
                consecutive_failures: 0,
                history: VecDeque::new(),
            }
        }
    }
}
