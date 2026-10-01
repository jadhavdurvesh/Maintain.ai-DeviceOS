mod config_engine;
mod config_store;
mod db;
mod firmware;
mod firmware_generator;
mod machine_registry;
mod registry;
mod wiring;

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
    let timestamp_ms = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis()).unwrap_or_default();
    HealthStatus { native_layer: "ready", serial_support: true, sqlite_support: true, timestamp_ms }
}

#[tauri::command]
pub fn list_serial_ports() -> Result<Vec<String>, String> {
    serialport::available_ports().map(|ports| ports.into_iter().map(|port| port.port_name).collect()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_boards() -> Vec<registry::BoardDefinition> { registry::boards() }
#[tauri::command]
pub fn list_sensors() -> Vec<registry::SensorDefinition> { registry::sensors() }
#[tauri::command]
pub fn list_machine_signals() -> Vec<registry::MachineSignalDefinition> { registry::machine_signals() }
#[tauri::command]
pub fn list_machine_types() -> Vec<machine_registry::MachineTypeDefinition> { machine_registry::machine_types() }

#[tauri::command]
pub fn validate_configuration(request: config_engine::ConfigurationRequest) -> config_engine::CompatibilityResult {
    config_engine::validate_configuration(&request)
}

#[tauri::command]
pub fn save_configuration(path: String, config: config_store::DeviceConfiguration) -> Result<String, String> {
    let connection = db::open_database(&path).map_err(|e| e.to_string())?;
    config_store::save_configuration(&connection, &config).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn load_configuration(path: String, id: String) -> Result<config_store::DeviceConfiguration, String> {
    let connection = db::open_database(&path).map_err(|e| e.to_string())?;
    config_store::load_configuration(&connection, &id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_configuration(path: String, id: String) -> Result<(), String> {
    let connection = db::open_database(&path).map_err(|e| e.to_string())?;
    config_store::delete_configuration(&connection, &id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn generate_wiring(config: config_store::DeviceConfiguration) -> Result<wiring::WiringSpecification, String> {
    wiring::generate_wiring(&config)
}

#[tauri::command]
pub fn build_firmware_spec(config: config_store::DeviceConfiguration) -> Result<firmware::FirmwareBuildSpecification, String> {
    firmware::build_specification(&config)
}

#[tauri::command]
pub fn generate_firmware(config: config_store::DeviceConfiguration) -> Result<firmware_generator::GeneratedFirmware, String> {
    firmware_generator::generate(&config)
}

#[tauri::command]
pub fn initialize_database(path: String) -> Result<(), String> {
    db::open_database(&path).map(|_| ()).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            health,
            list_serial_ports,
            list_boards,
            list_sensors,
            list_machine_signals,
            list_machine_types,
            validate_configuration,
            save_configuration,
            load_configuration,
            delete_configuration,
            generate_wiring,
            build_firmware_spec,
            generate_firmware,
            initialize_database
        ])
        .run(tauri::generate_context!())
        .expect("error while running Maintain.ai DeviceOS");
}
