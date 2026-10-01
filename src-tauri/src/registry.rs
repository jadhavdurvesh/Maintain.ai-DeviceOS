use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub fqbn: &'static str,
    pub digital_pins: &'static [u8],
    pub analog_pins: &'static [u8],
    pub buses: &'static [&'static str],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub signal_type: &'static str,
    pub unit: &'static str,
    pub data_type: &'static str,
    pub min: Option<f32>,
    pub max: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub category: &'static str,
    pub protocol: &'static str,
    pub parameters: &'static [ParameterDefinition],
    pub required_digital_pins: u8,
    pub required_analog_pins: u8,
    pub buses: &'static [&'static str],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineSignalDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub category: &'static str,
    pub units: &'static [&'static str],
}

const UNO_DIGITAL: &[u8] = &[0,1,2,3,4,5,6,7,8,9,10,11,12,13];
const UNO_ANALOG: &[u8] = &[14,15,16,17,18,19];
const NANO_DIGITAL: &[u8] = &[0,1,2,3,4,5,6,7,8,9,10,11,12,13];
const NANO_ANALOG: &[u8] = &[14,15,16,17,18,19];
const UNO_BUSES: &[&str] = &["uart", "i2c", "spi", "gpio", "analog"];
const NANO_BUSES: &[&str] = &["uart", "i2c", "spi", "gpio", "analog"];

const TEMP: ParameterDefinition = ParameterDefinition { id: "temperature", name: "Temperature", signal_type: "temperature", unit: "°C", data_type: "float", min: None, max: None };
const HUMIDITY: ParameterDefinition = ParameterDefinition { id: "humidity", name: "Humidity", signal_type: "humidity", unit: "%", data_type: "float", min: Some(0.0), max: Some(100.0) };
const VIBRATION: ParameterDefinition = ParameterDefinition { id: "vibration", name: "Vibration", signal_type: "vibration", unit: "state", data_type: "boolean", min: None, max: None };
const CURRENT: ParameterDefinition = ParameterDefinition { id: "current", name: "Current", signal_type: "current", unit: "A", data_type: "float", min: Some(0.0), max: None };
const VOLTAGE: ParameterDefinition = ParameterDefinition { id: "voltage", name: "Voltage", signal_type: "voltage", unit: "V", data_type: "float", min: Some(0.0), max: None };
const PRESSURE: ParameterDefinition = ParameterDefinition { id: "pressure", name: "Pressure", signal_type: "pressure", unit: "bar", data_type: "float", min: Some(0.0), max: None };
const FLOW: ParameterDefinition = ParameterDefinition { id: "flow", name: "Flow", signal_type: "flow", unit: "L/min", data_type: "float", min: Some(0.0), max: None };
const SPEED: ParameterDefinition = ParameterDefinition { id: "speed", name: "Speed", signal_type: "speed", unit: "m/s", data_type: "float", min: Some(0.0), max: None };
const LOAD: ParameterDefinition = ParameterDefinition { id: "load", name: "Load", signal_type: "load", unit: "%", data_type: "float", min: Some(0.0), max: Some(100.0) };
const RPM: ParameterDefinition = ParameterDefinition { id: "rpm", name: "RPM", signal_type: "rpm", unit: "rpm", data_type: "float", min: Some(0.0), max: None };
const DISTANCE: ParameterDefinition = ParameterDefinition { id: "distance", name: "Distance", signal_type: "distance", unit: "mm", data_type: "float", min: Some(0.0), max: None };
const ANALOG: ParameterDefinition = ParameterDefinition { id: "analog", name: "Analog", signal_type: "analog", unit: "raw", data_type: "integer", min: Some(0.0), max: Some(1023.0) };
const DIGITAL: ParameterDefinition = ParameterDefinition { id: "digital", name: "Digital", signal_type: "digital", unit: "state", data_type: "boolean", min: None, max: None };

pub fn boards() -> Vec<BoardDefinition> {
    vec![
        BoardDefinition { id: "arduino-uno-r3", name: "Arduino Uno R3", fqbn: "arduino:avr:uno", digital_pins: UNO_DIGITAL, analog_pins: UNO_ANALOG, buses: UNO_BUSES },
        BoardDefinition { id: "arduino-nano", name: "Arduino Nano", fqbn: "arduino:avr:nano", digital_pins: NANO_DIGITAL, analog_pins: NANO_ANALOG, buses: NANO_BUSES },
    ]
}

pub fn sensors() -> Vec<SensorDefinition> {
    vec![
        SensorDefinition { id: "dht11", name: "DHT11", category: "environmental", protocol: "single-wire", parameters: &[TEMP, HUMIDITY], required_digital_pins: 1, required_analog_pins: 0, buses: &["gpio"] },
        SensorDefinition { id: "sw420", name: "SW-420 Vibration", category: "vibration", protocol: "digital", parameters: &[VIBRATION], required_digital_pins: 1, required_analog_pins: 0, buses: &["gpio"] },
        SensorDefinition { id: "current-sensor", name: "Current Sensor", category: "electrical", protocol: "analog", parameters: &[CURRENT], required_digital_pins: 0, required_analog_pins: 1, buses: &["analog"] },
        SensorDefinition { id: "voltage-sensor", name: "Voltage Sensor", category: "electrical", protocol: "analog", parameters: &[VOLTAGE], required_digital_pins: 0, required_analog_pins: 1, buses: &["analog"] },
        SensorDefinition { id: "pressure-sensor", name: "Pressure Sensor", category: "pressure", protocol: "analog", parameters: &[PRESSURE], required_digital_pins: 0, required_analog_pins: 1, buses: &["analog"] },
        SensorDefinition { id: "flow-sensor", name: "Flow Sensor", category: "flow", protocol: "pulse", parameters: &[FLOW], required_digital_pins: 1, required_analog_pins: 0, buses: &["gpio"] },
        SensorDefinition { id: "speed-sensor", name: "Speed Sensor", category: "motion", protocol: "pulse", parameters: &[SPEED], required_digital_pins: 1, required_analog_pins: 0, buses: &["gpio"] },
        SensorDefinition { id: "rpm-sensor", name: "RPM Sensor", category: "motion", protocol: "pulse", parameters: &[RPM], required_digital_pins: 1, required_analog_pins: 0, buses: &["gpio"] },
        SensorDefinition { id: "load-sensor", name: "Load Sensor", category: "mechanical", protocol: "analog", parameters: &[LOAD], required_digital_pins: 0, required_analog_pins: 1, buses: &["analog"] },
        SensorDefinition { id: "distance-sensor", name: "Distance Sensor", category: "position", protocol: "digital", parameters: &[DISTANCE], required_digital_pins: 2, required_analog_pins: 0, buses: &["gpio"] },
        SensorDefinition { id: "generic-analog-input", name: "Generic Analog Input", category: "generic", protocol: "analog", parameters: &[ANALOG], required_digital_pins: 0, required_analog_pins: 1, buses: &["analog"] },
        SensorDefinition { id: "generic-digital-input", name: "Generic Digital Input", category: "generic", protocol: "digital", parameters: &[DIGITAL], required_digital_pins: 1, required_analog_pins: 0, buses: &["gpio"] },
    ]
}

pub fn machine_signals() -> Vec<MachineSignalDefinition> {
    vec![
        MachineSignalDefinition { id: "temperature", name: "Temperature", category: "thermal", units: &["°C", "°F", "K"] },
        MachineSignalDefinition { id: "humidity", name: "Humidity", category: "environmental", units: &["%"] },
        MachineSignalDefinition { id: "vibration", name: "Vibration", category: "condition", units: &["state", "mm/s", "g"] },
        MachineSignalDefinition { id: "current", name: "Current", category: "electrical", units: &["A"] },
        MachineSignalDefinition { id: "voltage", name: "Voltage", category: "electrical", units: &["V"] },
        MachineSignalDefinition { id: "pressure", name: "Pressure", category: "process", units: &["bar", "psi", "kPa"] },
        MachineSignalDefinition { id: "flow", name: "Flow", category: "process", units: &["L/min", "m³/h"] },
        MachineSignalDefinition { id: "speed", name: "Speed", category: "motion", units: &["m/s"] },
        MachineSignalDefinition { id: "load", name: "Load", category: "mechanical", units: &["%", "kg", "N"] },
        MachineSignalDefinition { id: "rpm", name: "RPM", category: "motion", units: &["rpm"] },
        MachineSignalDefinition { id: "distance", name: "Distance", category: "position", units: &["mm", "cm", "m"] },
    ]
}
