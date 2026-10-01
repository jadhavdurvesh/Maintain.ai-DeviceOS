use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub fqbn: &'static str,
    pub digital_pins: &'static [u8],
    pub analog_pins: &'static [u8],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub protocol: &'static str,
    pub parameters: &'static [&'static str],
    pub required_digital_pins: u8,
    pub required_analog_pins: u8,
}

const UNO_DIGITAL: &[u8] = &[0,1,2,3,4,5,6,7,8,9,10,11,12,13];
const UNO_ANALOG: &[u8] = &[14,15,16,17,18,19];
const NANO_DIGITAL: &[u8] = &[0,1,2,3,4,5,6,7,8,9,10,11,12,13];
const NANO_ANALOG: &[u8] = &[14,15,16,17,18,19];

pub fn boards() -> Vec<BoardDefinition> {
    vec![
        BoardDefinition { id: "arduino-uno-r3", name: "Arduino Uno R3", fqbn: "arduino:avr:uno", digital_pins: UNO_DIGITAL, analog_pins: UNO_ANALOG },
        BoardDefinition { id: "arduino-nano", name: "Arduino Nano", fqbn: "arduino:avr:nano", digital_pins: NANO_DIGITAL, analog_pins: NANO_ANALOG },
    ]
}

pub fn sensors() -> Vec<SensorDefinition> {
    vec![
        SensorDefinition { id: "dht11", name: "DHT11", protocol: "single-wire", parameters: &["temperature", "humidity"], required_digital_pins: 1, required_analog_pins: 0 },
        SensorDefinition { id: "sw420", name: "SW-420 Vibration", protocol: "digital", parameters: &["vibration"], required_digital_pins: 1, required_analog_pins: 0 },
        SensorDefinition { id: "generic-analog-input", name: "Generic Analog Input", protocol: "analog", parameters: &["analog"], required_digital_pins: 0, required_analog_pins: 1 },
        SensorDefinition { id: "generic-digital-input", name: "Generic Digital Input", protocol: "digital", parameters: &["digital"], required_digital_pins: 1, required_analog_pins: 0 },
    ]
}
