use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::machine_registry::machine_types;
use crate::registry::{boards, sensors};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorSelection { pub sensor_id: String, pub parameter_ids: Vec<String> }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationRequest { pub machine_type_id: String, pub board_id: String, pub sensors: Vec<SensorSelection> }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinAssignment { pub sensor_id: String, pub parameter_id: String, pub pin: u8, pub pin_type: String }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityResult { pub valid: bool, pub errors: Vec<String>, pub warnings: Vec<String>, pub assignments: Vec<PinAssignment> }

pub fn validate_configuration(request: &ConfigurationRequest) -> CompatibilityResult {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let mut assignments = Vec::new();
    let machine = machine_types().into_iter().find(|m| m.id == request.machine_type_id);
    let board = boards().into_iter().find(|b| b.id == request.board_id);
    let (Some(machine), Some(board)) = (machine, board) else { errors.push("Unknown machine type or board".into()); return CompatibilityResult { valid:false, errors, warnings, assignments }; };
    let machine_signals: HashSet<&str> = machine.signal_ids.iter().copied().collect();
    let mut used_digital: HashSet<u8> = HashSet::new();
    let mut used_analog: HashSet<u8> = HashSet::new();

    for selection in &request.sensors {
        let sensor = sensors().into_iter().find(|s| s.id == selection.sensor_id);
        let Some(sensor) = sensor else { errors.push(format!("Unknown sensor: {}", selection.sensor_id)); continue; };
        if !sensor.buses.iter().any(|bus| board.buses.contains(bus)) { errors.push(format!("Sensor {} has no compatible bus on {}", sensor.name, board.name)); }
        if selection.parameter_ids.is_empty() { warnings.push(format!("No parameters selected for {}", sensor.name)); continue; }
        let selected_parameters = selection.parameter_ids.iter().filter_map(|id| sensor.parameters.iter().find(|p| p.id == id)).collect::<Vec<_>>();
        for parameter in &selected_parameters { if !machine_signals.contains(parameter.signal_type) { warnings.push(format!("{} is not a declared signal for machine type {}", parameter.name, machine.name)); } }
        let digital_needed = sensor.required_digital_pins as usize;
        let analog_needed = sensor.required_analog_pins as usize;
        if board.digital_pins.iter().filter(|p| !used_digital.contains(p)).count() < digital_needed { errors.push(format!("Not enough free digital pins for {}", sensor.name)); }
        if board.analog_pins.iter().filter(|p| !used_analog.contains(p)).count() < analog_needed { errors.push(format!("Not enough free analog pins for {}", sensor.name)); }

        for parameter in selected_parameters {
            if parameter.signal_type == "analog" {
                let pin = board.analog_pins.iter().copied().find(|pin| !used_analog.contains(pin));
                if let Some(pin) = pin {
                    used_analog.insert(pin);
                    assignments.push(PinAssignment { sensor_id:sensor.id.to_string(), parameter_id:parameter.id.to_string(), pin, pin_type:"analog".into() });
                }
            } else {
                let pin = board.digital_pins.iter().copied().find(|pin| !used_digital.contains(pin));
                if let Some(pin) = pin {
                    used_digital.insert(pin);
                    assignments.push(PinAssignment { sensor_id:sensor.id.to_string(), parameter_id:parameter.id.to_string(), pin, pin_type:"digital".into() });
                }
            }
        }
    }
    CompatibilityResult { valid:errors.is_empty(), errors, warnings, assignments }
}
