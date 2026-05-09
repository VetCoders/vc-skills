use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum IpcEvent {
    StateChange {
        service: String,
        from: String,
        to: String,
    },
    RouteUpdate {
        client: String,
        server: String,
        count: u64,
        p99_ms: u64,
    },
    ServerHealth {
        name: String,
        rss_mb: u64,
        restarts: u64,
        last_error: Option<String>,
    },
    ClientDrift {
        client: String,
        non_mux_paths: Vec<String>,
    },
}
