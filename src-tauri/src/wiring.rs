use serde::{Deserialize, Serialize};

use crate::config_store::DeviceConfiguration;
use crate::registry::{boards, sensors};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WiringConnection {
    pub sensor_id: String,
    pub parameter_id: String,
    pub signal_type: String,
    pub pin: u8,
    pub pin_type: String,
    pub notes: String,
}

fn pin_label(board_id: &str, pin: u8, pin_type: &str) -> String { if pin_type == "analog" { if board_id == "esp32-devkit-v1" { format!("GPIO {}", pin) } else { format!("A{}", pin.saturating_sub(14)) } } else { format!("D{}", pin) } }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WiringSpecification {
    pub configuration_id: String,
    pub board_id: String,
    pub board_name: String,
    pub connections: Vec<WiringConnection>,
    pub warnings: Vec<String>,
}

pub fn generate_wiring(config: &DeviceConfiguration) -> Result<WiringSpecification, String> {
    let board = boards().into_iter().find(|b| b.id == config.board_id)
        .ok_or_else(|| format!("Unknown board: {}", config.board_id))?;
    let mut connections = Vec::new();
    let mut warnings = Vec::new();

    for assignment in &config.assignments {
        let sensor = sensors().into_iter().find(|s| s.id == assignment.sensor_id)
            .ok_or_else(|| format!("Unknown sensor: {}", assignment.sensor_id))?;
        let parameter = sensor.parameters.iter().find(|p| p.id == assignment.parameter_id)
            .ok_or_else(|| format!("Parameter {} not found on {}", assignment.parameter_id, sensor.name))?;
        let notes = match assignment.pin_type.as_str() {
            "analog" => format!("Connect {} output to {}; verify sensor reference voltage and conditioning.", sensor.name, pin_label(board.id, assignment.pin, "analog")),
            "digital" => format!("Connect {} signal to {}; configure pull-up/pull-down as required by the sensor.", sensor.name, pin_label(board.id, assignment.pin, "digital")),
            other => format!("Pin type {} requires a board-specific wiring profile.", other),
        };
        connections.push(WiringConnection {
            sensor_id: sensor.id.to_string(),
            parameter_id: parameter.id.to_string(),
            signal_type: parameter.signal_type.to_string(),
            pin: assignment.pin,
            pin_type: assignment.pin_type.to_string(),
            notes,
        });
    }

    if connections.is_empty() {
        warnings.push("Configuration has no pin assignments.".into());
    }

    Ok(WiringSpecification { configuration_id: config.id.clone(), board_id: board.id.to_string(), board_name: board.name.to_string(), connections, warnings })
}
