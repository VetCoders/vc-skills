use std::sync::OnceLock;

use crossbeam_channel::{Receiver, Sender, unbounded};
use tracing::debug;

use crate::menu::update_status_label;
use crate::types::{TrayMenuEvent, TrayStatus};

pub static STATUS_CHANNEL: OnceLock<Sender<TrayStatus>> = OnceLock::new();
pub static MENU_EVENT_CHANNEL: OnceLock<Sender<TrayMenuEvent>> = OnceLock::new();

pub fn update_tray_status(status: TrayStatus) -> anyhow::Result<()> {
    if let Some(sender) = STATUS_CHANNEL.get() {
        sender
            .send(status)
            .map_err(|error| anyhow::anyhow!("failed to send tray status: {error}"))?;
        debug!("tray status update sent: {status:?}");
    }
    Ok(())
}

pub fn menu_event_receiver() -> anyhow::Result<Receiver<TrayMenuEvent>> {
    let (tx, rx) = unbounded();
    MENU_EVENT_CHANNEL
        .set(tx)
        .map_err(|_| anyhow::anyhow!("menu event channel already initialized"))?;
    Ok(rx)
}

pub fn send_menu_event(event: TrayMenuEvent) {
    if let Some(sender) = MENU_EVENT_CHANNEL.get() {
        let _ = sender.send(event);
    }
}

pub fn apply_status_update(status: TrayStatus) {
    update_status_label(&status.menu_label(crate::menu::current_service_count()));
}

pub fn init_channels() -> anyhow::Result<Receiver<TrayStatus>> {
    let (status_tx, status_rx) = unbounded();
    STATUS_CHANNEL
        .set(status_tx)
        .map_err(|_| anyhow::anyhow!("status channel already initialized"))?;
    Ok(status_rx)
}

