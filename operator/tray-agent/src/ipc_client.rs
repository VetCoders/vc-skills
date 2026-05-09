use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;
use tracing::{debug, warn};

pub use mux_agent::ipc::command::{ClientKind, MuxControlCommand, MuxControlResponse};
pub use mux_agent::ipc::event::IpcEvent;

use crate::state::update_tray_status;
use crate::types::TrayStatus;

pub fn default_socket_path() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".rust-mux/ipc/control.sock")
}

pub async fn subscribe_loop(socket_path: PathBuf) {
    let mut attempt = 0u32;
    let mut backoff = Duration::from_secs(1);
    loop {
        match subscribe_once(&socket_path).await {
            Ok(()) => {
                attempt = 0;
                backoff = Duration::from_secs(1);
            }
            Err(error) => {
                attempt += 1;
                warn!("mux tray IPC subscribe failed attempt {attempt}/10: {error:#}");
                if attempt >= 10 {
                    let _ = update_tray_status(TrayStatus::Failed);
                    return;
                }
                tokio::time::sleep(backoff).await;
                backoff = (backoff * 2).min(Duration::from_secs(30));
            }
        }
    }
}

async fn subscribe_once(socket_path: &Path) -> Result<()> {
    let stream = UnixStream::connect(socket_path)
        .await
        .with_context(|| format!("connect {}", socket_path.display()))?;
    let (reader, mut writer) = stream.into_split();
    write_request(&mut writer, &MuxControlCommand::Subscribe).await?;
    let mut lines = BufReader::new(reader).lines();
    while let Some(line) = lines.next_line().await? {
        let response: MuxControlResponse = serde_json::from_str(&line)?;
        if let MuxControlResponse::Event(event) = response {
            handle_event(event)?;
        } else {
            debug!("mux subscribe response: {response:?}");
        }
    }
    anyhow::bail!("mux IPC stream closed")
}

fn handle_event(event: IpcEvent) -> Result<()> {
    match event {
        IpcEvent::StateChange { service: _, from: _, to } => {
            let status = match to.as_str() {
                "failed" => TrayStatus::Failed,
                "restarting" => TrayStatus::Restarting,
                "routing" => TrayStatus::Routing,
                "saturated" => TrayStatus::Saturated,
                _ => TrayStatus::Idle,
            };
            update_tray_status(status);
            Ok(())
        }
        IpcEvent::ServerHealth { name: _, rss_mb: _, restarts, last_error } => {
            let status = if restarts > 0 && last_error.is_some() {
                TrayStatus::Failed
            } else if restarts > 0 {
                TrayStatus::Restarting
            } else {
                TrayStatus::Idle
            };
            update_tray_status(status);
            Ok(())
        }
        IpcEvent::RouteUpdate { .. } => {
            Ok(())
        }
        IpcEvent::ClientDrift { .. } => {
            Ok(())
        }
    }
}

pub async fn send_command(
    socket_path: impl AsRef<Path>,
    command: &MuxControlCommand,
) -> Result<MuxControlResponse> {
    let mut stream = UnixStream::connect(socket_path.as_ref()).await?;
    write_request(&mut stream, command).await?;
    let mut lines = BufReader::new(stream).lines();
    let Some(line) = lines.next_line().await? else {
        anyhow::bail!("mux IPC returned no response");
    };
    serde_json::from_str(&line).context("decode mux response")
}

async fn write_request<W>(writer: &mut W, command: &MuxControlCommand) -> Result<()> 
where
    W: AsyncWriteExt + Unpin,
{
    let encoded = serde_json::to_string(command)?;
    writer.write_all(encoded.as_bytes()).await?;
    writer.write_all(b"\n").await?;
    writer.flush().await?;
    Ok(())
}

pub async fn stop_mux_daemon(socket_path: &Path) -> Result<MuxControlResponse> {
    send_command(socket_path, &MuxControlCommand::Shutdown { graceful: true }).await
}

pub async fn restart_service(socket_path: &Path, name: &str) -> Result<MuxControlResponse> {
    send_command(
        socket_path,
        &MuxControlCommand::RestartService {
            name: name.to_string(),
        },
    )
    .await
}

pub async fn verify_client(socket_path: &Path, client_kind: ClientKind) -> Result<String> {
    let kind_str = match &client_kind {
        ClientKind::Claude => "Claude",
        ClientKind::Codex => "Codex",
        ClientKind::Gemini => "Gemini",
        ClientKind::Junie => "Junie",
        ClientKind::Generic { name } => name.as_str(),
    };

    match send_command(socket_path, &MuxControlCommand::Verify { client_kind }).await? {
        MuxControlResponse::VerifyResult(result) => {
            let detail = if result.ok {
                "ok".to_string()
            } else {
                format!("drift, {} non-mux endpoints", result.non_mux_servers.len())
            };
            Ok(format!("{kind_str}: {detail}"))
        }
        MuxControlResponse::Error(message) => anyhow::bail!(message),
        other => Ok(format!("{other:?}")),
    }
}

pub async fn route_snapshot(socket_path: PathBuf) -> Result<String> {
    match send_command(&socket_path, &MuxControlCommand::RouteSnapshot).await? {
        MuxControlResponse::Routes(routes) => {
            Ok(serde_json::to_string_pretty(&routes)?)
        }
        MuxControlResponse::Error(message) => anyhow::bail!(message),
        other => Ok(format!("{other:?}")),
    }
}

// Diagnostics is no longer part of MuxControlCommand. So we will fail if called.
pub async fn diagnostics(_socket_path: PathBuf) -> Result<String> {
    anyhow::bail!("Diagnostics command is not supported by the canonical schema");
}
