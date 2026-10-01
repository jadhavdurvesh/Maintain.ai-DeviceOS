use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize)]
pub struct HealthStatus {
    pub native_layer: &'static str,
    pub serial_support: bool,
    pub sqlite_support: bool,
    pub timestamp_ms: u128,
}

#[tauri::command]
pub fn health() -> HealthStatus {
    let timestamp_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default();

    HealthStatus {
        native_layer: "ready",
        serial_support: true,
        sqlite_support: true,
        timestamp_ms,
    }
}

#[tauri::command]
pub fn list_serial_ports() -> Result<Vec<String>, String> {
    serialport::available_ports()
        .map(|ports| ports.into_iter().map(|port| port.port_name).collect())
        .map_err(|error| error.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![health, list_serial_ports])
        .run(tauri::generate_context!())
        .expect("error while running Maintain.ai DeviceOS");
}
