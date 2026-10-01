use crate::config_store::DeviceConfiguration;

pub use crate::runtime_firmware::RuntimeFirmware as GatewayFirmware;

pub fn generate(config: &DeviceConfiguration) -> Result<GatewayFirmware, String> {
    crate::runtime_firmware::generate(config)
}
