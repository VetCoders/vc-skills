uniffi::setup_scaffolding!();

use std::path::PathBuf;
use std::sync::OnceLock;
use tray_agent::ipc_client::{send_command, ClientKind, MuxControlCommand, MuxControlResponse};

static SOCKET_PATH: OnceLock<PathBuf> = OnceLock::new();

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum MuxError {
    #[error("{msg}")]
    Core { msg: String },
}

impl From<anyhow::Error> for MuxError {
    fn from(e: anyhow::Error) -> Self {
        MuxError::Core {
            msg: format!("{e:#}"),
        }
    }
}

#[derive(uniffi::Enum)]
pub enum FfiClientKind {
    Claude,
    Codex,
    Gemini,
    Junie,
    Generic { name: String },
}

impl From<ClientKind> for FfiClientKind {
    fn from(k: ClientKind) -> Self {
        match k {
            ClientKind::Claude => FfiClientKind::Claude,
            ClientKind::Codex => FfiClientKind::Codex,
            ClientKind::Gemini => FfiClientKind::Gemini,
            ClientKind::Junie => FfiClientKind::Junie,
            ClientKind::Generic { name } => FfiClientKind::Generic { name },
        }
    }
}

impl From<FfiClientKind> for ClientKind {
    fn from(k: FfiClientKind) -> Self {
        match k {
            FfiClientKind::Claude => ClientKind::Claude,
            FfiClientKind::Codex => ClientKind::Codex,
            FfiClientKind::Gemini => ClientKind::Gemini,
            FfiClientKind::Junie => ClientKind::Junie,
            FfiClientKind::Generic { name } => ClientKind::Generic { name },
        }
    }
}

#[derive(uniffi::Record)]
pub struct FfiServerStatus {
    pub name: String,
    pub status: String,
    pub queue_depth: u32,
    pub queue_capacity: u32,
    pub restart_count: u64,
}

#[derive(uniffi::Record)]
pub struct FfiRoute {
    pub client: FfiClientKind,
    pub service: String,
    pub state: String,
}

#[derive(uniffi::Record)]
pub struct FfiClientConfig {
    pub kind: FfiClientKind,
    pub config: String,
}

#[derive(uniffi::Record)]
pub struct FfiVerifyResult {
    pub kind: FfiClientKind,
    pub ok: bool,
    pub detail: String,
}

#[derive(uniffi::Record)]
pub struct FfiNonMuxEntry {
    pub name: String,
}

#[derive(uniffi::Enum)]
pub enum FfiSubscriberState {
    Connected,
    Disconnected,
}

// ═══════════════════════════════════════════════════════════
// Engine
// ═══════════════════════════════════════════════════════════

#[uniffi::export]
pub fn init_runtime(socket_path: String) -> Result<(), MuxError> {
    SOCKET_PATH
        .set(PathBuf::from(socket_path))
        .map_err(|_| MuxError::Core {
            msg: "Already initialized".to_string(),
        })?;
    Ok(())
}

fn get_socket_path() -> Result<PathBuf, MuxError> {
    SOCKET_PATH.get().cloned().ok_or_else(|| MuxError::Core {
        msg: "Runtime not initialized".to_string(),
    })
}

#[uniffi::export(callback_interface)]
pub trait EventCallback: Send + Sync {
    fn on_event(&self, event_json: String);
    fn on_error(&self, err: String);
}

#[uniffi::export]
pub async fn subscribe_events(callback: Box<dyn EventCallback>) -> Result<(), MuxError> {
    let socket = get_socket_path()?;
    // We launch a background task to stream events to avoid blocking or issues with returning streams.
    tokio::spawn(async move {
        use tokio::io::AsyncBufReadExt;
        let stream_res = tokio::net::UnixStream::connect(&socket).await;
        if let Err(e) = stream_res {
            callback.on_error(e.to_string());
            return;
        }
        let stream = stream_res.unwrap();
        let (reader, mut writer) = stream.into_split();
        let command = MuxControlCommand::Subscribe;
        let encoded = serde_json::to_string(&command).unwrap() + "\n";

        use tokio::io::AsyncWriteExt;
        if let Err(e) = writer.write_all(encoded.as_bytes()).await {
            callback.on_error(e.to_string());
            return;
        }

        let mut lines = tokio::io::BufReader::new(reader).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            callback.on_event(line);
        }
        callback.on_error("Stream closed".to_string());
    });
    Ok(())
}

#[uniffi::export]
pub async fn get_server_status() -> Result<Vec<FfiServerStatus>, MuxError> {
    let socket = get_socket_path()?;
    let res = send_command(&socket, &MuxControlCommand::GetStatus).await?;
    if let MuxControlResponse::Status(snapshot) = res {
        Ok(vec![FfiServerStatus {
            name: snapshot.service_name.clone(),
            status: format!("{:?}", snapshot.server_status),
            queue_depth: snapshot.pending_requests as u32,
            queue_capacity: snapshot.max_active_clients as u32,
            restart_count: snapshot.restarts,
        }])
    } else {
        Err(MuxError::Core {
            msg: "Unexpected response".into(),
        })
    }
}

#[uniffi::export]
pub async fn get_routes() -> Result<Vec<FfiRoute>, MuxError> {
    let socket = get_socket_path()?;
    let res = send_command(&socket, &MuxControlCommand::RouteSnapshot).await?;
    if let MuxControlResponse::Routes(routes) = res {
        Ok(routes
            .into_iter()
            .map(|r| FfiRoute {
                client: FfiClientKind::Generic { name: r.client }, // Or map based on name
                service: r.server,
                state: r.status,
            })
            .collect())
    } else {
        Err(MuxError::Core {
            msg: "Unexpected response".into(),
        })
    }
}

#[uniffi::export]
pub async fn verify_client(kind: FfiClientKind) -> Result<FfiVerifyResult, MuxError> {
    let socket = get_socket_path()?;
    let ckind: ClientKind = kind.into();
    let res = send_command(
        &socket,
        &MuxControlCommand::Verify {
            client_kind: ckind.clone(),
        },
    )
    .await?;
    if let MuxControlResponse::VerifyResult(result) = res {
        Ok(FfiVerifyResult {
            kind: ckind.into(),
            ok: result.ok,
            detail: format!("{} non-mux", result.non_mux_servers.len()),
        })
    } else {
        Err(MuxError::Core {
            msg: "Unexpected response".into(),
        })
    }
}

#[uniffi::export]
pub async fn restart_service(name: String) -> Result<(), MuxError> {
    let socket = get_socket_path()?;
    let _res = send_command(&socket, &MuxControlCommand::RestartService { name }).await?;
    Ok(())
}

#[uniffi::export]
pub async fn get_recent_logs(service: String, _lines: u32) -> Result<Vec<String>, MuxError> {
    Ok(vec![format!(
        "Logs for {} not implemented in backend",
        service
    )])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_kind_roundtrip() {
        let original = ClientKind::Claude;
        let ffi: FfiClientKind = original.clone().into();
        let back: ClientKind = ffi.into();
        assert_eq!(original, back);

        let original = ClientKind::Generic {
            name: "test".to_string(),
        };
        let ffi: FfiClientKind = original.clone().into();
        let back: ClientKind = ffi.into();
        assert_eq!(original, back);
    }
}
