use serde::{Deserialize, Serialize};

use crate::config_store::DeviceConfiguration;
use crate::registry::boards;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirmwareInput {
    pub configuration_id: String,
    pub board_id: String,
    pub fqbn: String,
    pub schema_version: String,
    pub definition_version: String,
    pub pin_assignments: Vec<crate::config_engine::PinAssignment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirmwareBuildSpecification {
    pub input: FirmwareInput,
    pub source_template: String,
    pub compile_command: String,
    pub upload_command: String,
}

pub fn build_specification(config: &DeviceConfiguration) -> Result<FirmwareBuildSpecification, String> {
    let board = boards().into_iter().find(|b| b.id == config.board_id)
        .ok_or_else(|| format!("Unknown board: {}", config.board_id))?;

    let input = FirmwareInput {
        configuration_id: config.id.clone(),
        board_id: board.id.to_string(),
        fqbn: board.fqbn.to_string(),
        schema_version: config.schema_version.clone(),
        definition_version: config.definition_version.clone(),
        pin_assignments: config.assignments.clone(),
    };

    Ok(FirmwareBuildSpecification {
        input,
        source_template: "maintain-ai-device-runtime".into(),
        compile_command: format!("arduino-cli compile --fqbn {} <generated-sketch>", board.fqbn),
        upload_command: format!("arduino-cli upload -p <serial-port> --fqbn {} <generated-sketch>", board.fqbn),
    })
}
