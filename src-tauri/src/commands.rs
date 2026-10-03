use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::Manager;

use crate::{arduino_toolchain, arduino_upload, config_engine, config_store, db, device_verification, firmware, firmware_generator, gateway_firmware, machine_registry, registry, runtime_firmware, runtime_protocol, sensor_drivers, serial_runtime, telemetry, wiring};

#[derive(Debug, Serialize)]
pub struct HealthStatus { pub native_layer: &'static str, pub serial_support: bool, pub sqlite_support: bool, pub timestamp_ms: u128 }
#[tauri::command]
pub fn health() -> HealthStatus { let timestamp_ms=SystemTime::now().duration_since(UNIX_EPOCH).map(|d|d.as_millis()).unwrap_or_default(); HealthStatus{native_layer:"ready",serial_support:true,sqlite_support:true,timestamp_ms} }
#[tauri::command]
pub fn list_serial_ports() -> Result<Vec<String>, String> { serial_runtime::list_ports().map(|p|p.into_iter().map(|x|x.port_name).collect()) }
#[tauri::command]
pub fn list_boards() -> Vec<registry::BoardDefinition> { registry::boards() }
#[tauri::command]
pub fn list_sensors() -> Vec<registry::SensorDefinition> { registry::sensors() }
#[tauri::command]
pub fn list_machine_signals() -> Vec<registry::MachineSignalDefinition> { registry::machine_signals() }
#[tauri::command]
pub fn list_machine_types() -> Vec<machine_registry::MachineTypeDefinition> { machine_registry::machine_types() }
#[tauri::command]
pub fn validate_configuration(request: config_engine::ConfigurationRequest) -> config_engine::CompatibilityResult { config_engine::validate_configuration(&request) }
#[tauri::command]
pub fn save_configuration(path:String,config:config_store::DeviceConfiguration)->Result<String,String>{let c=db::open_database(&path).map_err(|e|e.to_string())?;config_store::save_configuration(&c,&config).map_err(|e|e.to_string())}
#[tauri::command]
pub fn save_active_configuration(app: tauri::AppHandle, config: config_store::DeviceConfiguration)->Result<String,String>{let dir=app.path().app_data_dir().map_err(|e|e.to_string())?;std::fs::create_dir_all(&dir).map_err(|e|e.to_string())?;let path=dir.join("deviceos.db");let c=db::open_database(path.to_str().ok_or_else(||"Invalid database path".to_string())?).map_err(|e|e.to_string())?;config_store::save_configuration(&c,&config).map_err(|e|e.to_string())}
#[tauri::command]
pub fn load_configuration(path:String,id:String)->Result<config_store::DeviceConfiguration,String>{let c=db::open_database(&path).map_err(|e|e.to_string())?;config_store::load_configuration(&c,&id).map_err(|e|e.to_string())}
#[tauri::command]
pub fn delete_configuration(path:String,id:String)->Result<(),String>{let c=db::open_database(&path).map_err(|e|e.to_string())?;config_store::delete_configuration(&c,&id).map_err(|e|e.to_string())}
#[tauri::command]
pub fn generate_wiring(config:config_store::DeviceConfiguration)->Result<wiring::WiringSpecification,String>{wiring::generate_wiring(&config)}
#[tauri::command]
pub fn build_firmware_spec(config:config_store::DeviceConfiguration)->Result<firmware::FirmwareBuildSpecification,String>{firmware::build_specification(&config)}
#[tauri::command]
pub fn generate_firmware(config:config_store::DeviceConfiguration)->Result<firmware_generator::GeneratedFirmware,String>{firmware_generator::generate(&config)}
#[tauri::command]
pub fn generate_gateway_firmware(config:config_store::DeviceConfiguration)->Result<gateway_firmware::GatewayFirmware,String>{gateway_firmware::generate(&config)}
#[tauri::command]
pub fn generate_runtime_firmware(config:config_store::DeviceConfiguration)->Result<runtime_firmware::RuntimeFirmware,String>{runtime_firmware::generate(&config)}
#[tauri::command]
pub fn plan_sensor_drivers(config:config_store::DeviceConfiguration)->Result<sensor_drivers::DriverPlan,String>{sensor_drivers::plan(&config)}
#[tauri::command]
pub fn encode_telemetry(envelope:telemetry::TelemetryEnvelope)->Result<String,String>{telemetry::encode_json(&envelope)}
#[tauri::command]
pub fn open_serial(config:serial_runtime::SerialRuntimeConfig)->Result<String,String>{let _=serial_runtime::open(&config)?;Ok("connected".into())}
#[tauri::command]
pub fn validate_runtime_frame(config:config_store::DeviceConfiguration,frame:runtime_protocol::RuntimeFrame)->runtime_protocol::ValidationResult{runtime_protocol::validate_frame(&config,&frame)}
#[tauri::command]
pub fn bridge_runtime_frame(config:config_store::DeviceConfiguration,frame:runtime_protocol::RuntimeFrame)->Result<telemetry::TelemetryEnvelope,String>{runtime_protocol::to_telemetry(&config,&frame)}
#[tauri::command]
pub fn initialize_database(path:String)->Result<(),String>{db::open_database(&path).map(|_|()).map_err(|e|e.to_string())}
#[tauri::command]
pub fn arduino_cli_status()->arduino_toolchain::ToolchainStatus{arduino_toolchain::status()}
#[tauri::command]
pub async fn compile_gateway_firmware(config:config_store::DeviceConfiguration)->Result<arduino_toolchain::BuildResult,String>{tauri::async_runtime::spawn_blocking(move || arduino_toolchain::compile(&config)).await.map_err(|e|format!("Firmware build task failed: {e}"))?}
#[tauri::command]
pub async fn detect_arduino_boards()->Result<Vec<arduino_upload::DetectedBoard>,String>{tauri::async_runtime::spawn_blocking(arduino_upload::detect).await.map_err(|e|format!("Arduino detection task failed: {e}"))?}
#[tauri::command]
pub async fn upload_gateway_firmware(port:String,fqbn:String,build_dir:String)->Result<arduino_upload::UploadResult,String>{tauri::async_runtime::spawn_blocking(move || arduino_upload::upload(port,fqbn,build_dir)).await.map_err(|e|format!("Firmware upload task failed: {e}"))?}
#[tauri::command]
pub fn verify_uploaded_device(config:config_store::DeviceConfiguration,port:String,baud_rate:u32,timeout_ms:u64)->Result<device_verification::VerificationResult,String>{device_verification::verify(&config,port,baud_rate,timeout_ms)}
