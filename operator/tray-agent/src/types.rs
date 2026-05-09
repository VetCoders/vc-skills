use crate::ipc_client::ClientKind;
use anyhow::Result;
use muda::MenuId;
use tracing::debug;
use tray_icon::Icon;

use crate::icons::{create_fallback_icon, load_custom_icon};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayStatus {
    Idle,
    Routing,
    Saturated,
    Restarting,
    Failed,
}

impl TrayStatus {
    pub fn tooltip(&self) -> String {
        format!("Vibecrafted mux - {}", self.label())
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Idle => "Idle",
            Self::Routing => "Routing",
            Self::Saturated => "Saturated",
            Self::Restarting => "Restarting",
            Self::Failed => "Failed",
        }
    }

    pub fn menu_label(&self, service_count: usize) -> String {
        format!("Status: {} ({} services)", self.label(), service_count)
    }

    pub fn to_icon(self) -> Result<Icon> {
        load_custom_icon(self).or_else(|error| {
            debug!("custom tray icon failed, using fallback: {error}");
            create_fallback_icon(self)
        })
    }
}

pub fn silver_label_for_status(status: TrayStatus) -> &'static str {
    status.label()
}

#[derive(Debug, Clone)]
pub enum TrayMenuEvent {
    ShowMuxDashboard,
    OpenMuxLogs,
    CopyRoutingTable,
    CopyDiagnostics,
    RestartService(String),
    VerifyClient(ClientKind),
    ContinueOnboarding,
    OpenSettings,
    OpenHelp,
    ShowAbout,
    Quit,
}

pub struct MenuIds {
    pub show_dashboard: MenuId,
    pub open_logs: MenuId,
    pub copy_routing_table: MenuId,
    pub copy_diagnostics: MenuId,
    pub continue_onboarding: Option<MenuId>,
    pub open_settings: MenuId,
    pub help: MenuId,
    pub about: MenuId,
    pub quit: MenuId,
    pub restart_services: Vec<(String, MenuId)>,
    pub verify_clients: Vec<(ClientKind, MenuId)>,
}

impl MenuIds {
    pub fn resolve_restart_service(&self, id: &MenuId) -> Option<String> {
        self.restart_services
            .iter()
            .find_map(|(name, item_id)| (item_id == id).then(|| name.clone()))
    }

    pub fn resolve_verify_client(&self, id: &MenuId) -> Option<ClientKind> {
        self.verify_clients
            .iter()
            .find_map(|(kind, item_id): &(ClientKind, MenuId)| {
                (item_id == id).then(|| kind.clone())
            })
    }
}
