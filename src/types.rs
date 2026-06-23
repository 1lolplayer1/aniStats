use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize)]
pub enum CheckResult {
    Up,
    Down,
    Timeout,
}

#[derive(Debug, Clone, Serialize)]
pub struct CheckPoint {
    pub timestamp: DateTime<Utc>,
    pub status: CheckResult,
    pub latency_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SiteStatus {
    pub name: String,
    pub url: String,
    pub status: CheckResult,
    pub http_code: Option<u16>,
    pub latency_ms: Option<u64>,
    pub last_checked: DateTime<Utc>,
    pub consecutive_failures: u32,
    pub history: VecDeque<CheckPoint>,
}
