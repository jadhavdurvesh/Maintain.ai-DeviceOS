use serde::{Deserialize, Serialize};

use crate::config_store::DeviceConfiguration;
use crate::registry::sensors;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverDefinition {
    pub sensor_id: String,
    pub driver_id: String,
    pub protocol: String,
    pub library: Option<String>,
    pub parameters: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryChannel {
    pub sensor_id: String,
    pub parameter_id: String,
    pub signal_type: String,
    pub unit: String,
    pub sample_period_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverPlan {
    pub configuration_id: String,
    pub drivers: Vec<DriverDefinition>,
    pub telemetry_channels: Vec<TelemetryChannel>,
    pub warnings: Vec<String>,
}

fn driver_for(protocol: &str, sensor_id: &str) -> (&'static str, Option<&'static str>) {
    match protocol {
        "I2C" => ("i2c_generic", Some("Wire")),
        "UART" => ("uart_generic", None),
        "SPI" => ("spi_generic", Some("SPI")),
        "analog" => ("analog_generic", None),
        "digital" => ("digital_generic", None),
        _ => ("unsupported", None),
    }
}

pub fn plan(config: &DeviceConfiguration) -> Result<DriverPlan, String> {
    let mut drivers = Vec::new();
    let mut channels = Vec::new();
    let mut warnings = Vec::new();

    for selection in &config.sensors {
        let sensor = sensors().into_iter().find(|s| s.id == selection.sensor_id)
            .ok_or_else(|| format!("Unknown sensor: {}", selection.sensor_id))?;
        let (driver_id, library) = driver_for(sensor.protocol, sensor.id);
        if driver_id == "unsupported" {
            warnings.push(format!("No concrete runtime driver is registered for {} (protocol {}).", sensor.name, sensor.protocol));
        }
        drivers.push(DriverDefinition {
            sensor_id: sensor.id.to_string(),
            driver_id: driver_id.to_string(),
            protocol: sensor.protocol.to_string(),
            library: library.map(str::to_string),
            parameters: selection.parameter_ids.clone(),
        });

        for parameter_id in &selection.parameter_ids {
            let parameter = sensor.parameters.iter().find(|p| p.id == parameter_id)
                .ok_or_else(|| format!("Unknown parameter {} on {}", parameter_id, sensor.name))?;
            channels.push(TelemetryChannel {
                sensor_id: sensor.id.to_string(),
                parameter_id: parameter.id.to_string(),
                signal_type: parameter.signal_type.to_string(),
                unit: parameter.unit.to_string(),
                sample_period_ms: 1000,
            });
        }
    }

    Ok(DriverPlan { configuration_id: config.id.clone(), drivers, telemetry_channels: channels, warnings })
}
