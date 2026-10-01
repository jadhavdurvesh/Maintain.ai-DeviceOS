use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::machine_registry::machine_types;
use crate::registry::{boards, sensors};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorSelection {
    pub sensor_id: String,
    pub parameter_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationRequest {
    pub machine_type_id: String,
    pub board_id: String,
    pub sensors: Vec<SensorSelection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinAssignment {
    pub sensor_id: String,
    pub parameter_id: String,
    pub pin: u8,
    pub pin_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub assignments: Vec<PinAssignment>,
}

pub fn validate_configuration(request: &ConfigurationRequest) -> CompatibilityResult {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let mut assignments = Vec::new();

    let machine = machine_types().into_iter().find(|m| m.id == request.machine_type_id);
    let board = boards().into_iter().find(|b| b.id == request.board_id);

    let (Some(machine), Some(board)) = (machine, board) else {
        errors.push("Unknown machine type or board".into());
        return CompatibilityResult { valid: false, errors, warnings, assignments };
    };

    let machine_signals: HashSet<&str> = machine.signal_ids.iter().copied().collect();
    let mut used_digital = HashSet::new();
    let mut used_analog = HashSet::new();

    for selection in &request.sensors {
        let sensor = sensors().into_iter().find(|s| s.id == selection.sensor_id);
        let Some(sensor) = sensor else {
            errors.push(format!("Unknown sensor: {}", selection.sensor_id));
            continue;
        };

        if !sensor.buses.iter().any(|bus| board.buses.contains(bus)) {
            errors.push(format!("Sensor {} has no compatible bus on {}", sensor.name, board.name));
        }

        for parameter_id in &selection.parameter_ids {
            let parameter = sensor.parameters.iter().find(|p| p.id == parameter_id);
            let Some(parameter) = parameter else {
                errors.push(format!("Sensor {} does not expose parameter {}", sensor.name, parameter_id));
                continue;
            };

            if !machine_signals.contains(parameter.signal_type) {
                warnings.push(format!("{} is not a declared signal for machine type {}", parameter.name, machine.name));
            }

            if parameter.signal_type == "analog" {
                if let Some(pin) = board.analog_pins.iter().find(|p| !used_analog.contains(p)) {
                    used_analog.insert(*pin);
                    assignments.push(PinAssignment { sensor_id: sensor.id.to_string(), parameter_id: parameter.id.to_string(), pin: *pin, pin_type: "analog".into() });
                } else {
                    errors.push(format!("No free analog pin for {}", sensor.name));
                }
            } else {
                if let Some(pin) = board.digital_pins.iter().find(|p| !used_digital.contains(p)) {
                    used_digital.insert(*pin);
                    assignments.push(PinAssignment { sensor_id: sensor.id.to_string(), parameter_id: parameter.id.to_string(), pin: *pin, pin_type: "digital".into() });
                } else {
                    errors.push(format!("No free digital pin for {}", sensor.name));
                }
            }
        }
    }

    CompatibilityResult { valid: errors.is_empty(), errors, warnings, assignments }
}
