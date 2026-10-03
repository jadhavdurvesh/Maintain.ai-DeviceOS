use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config_engine::{ConfigurationRequest, PinAssignment};

pub const CONFIG_SCHEMA_VERSION: &str = "1.0";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConfiguration {
    pub id: String,
    pub device_id: Option<String>,
    #[serde(default)] pub wifi_ssid: Option<String>,
    #[serde(default)] pub wifi_password: Option<String>,
    #[serde(default)] pub api_base_url: Option<String>,
    #[serde(default)] pub device_key: Option<String>,
    pub machine_type_id: String,
    pub board_id: String,
    pub sensors: Vec<crate::config_engine::SensorSelection>,
    pub assignments: Vec<PinAssignment>,
    pub schema_version: String,
    pub definition_version: String,
}

fn now_ms() -> String {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis().to_string()).unwrap_or_else(|_| "0".into())
}

fn hash_config(json: &str) -> String {
    let mut hasher = DefaultHasher::new();
    json.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

pub fn save_configuration(connection: &Connection, config: &DeviceConfiguration) -> Result<String> {
    let id = if config.id.is_empty() { format!("cfg-{}", now_ms()) } else { config.id.clone() };
    let json = serde_json::to_string(config).map_err(|_| rusqlite::Error::InvalidQuery)?;
    let hash = hash_config(&json);
    connection.execute(
        "INSERT OR REPLACE INTO configurations (id, device_id, schema_version, definition_version, config_json, config_hash, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![id, config.device_id, config.schema_version, config.definition_version, json, hash, now_ms()],
    )?;
    Ok(id)
}

pub fn load_configuration(connection: &Connection, id: &str) -> Result<DeviceConfiguration> {
    let json: String = connection.query_row("SELECT config_json FROM configurations WHERE id = ?1", params![id], |row| row.get(0))?;
    serde_json::from_str(&json).map_err(|_| rusqlite::Error::InvalidQuery)
}

pub fn delete_configuration(connection: &Connection, id: &str) -> Result<()> {
    connection.execute("DELETE FROM configurations WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn request_to_configuration(request: ConfigurationRequest, assignments: Vec<PinAssignment>, definition_version: &str) -> DeviceConfiguration {
    DeviceConfiguration {
        id: String::new(),
        device_id: None,
        wifi_ssid: None,
        wifi_password: None,
        api_base_url: None,
        device_key: None,
        machine_type_id: request.machine_type_id,
        board_id: request.board_id,
        sensors: request.sensors,
        assignments,
        schema_version: CONFIG_SCHEMA_VERSION.into(),
        definition_version: definition_version.into(),
    }
}
