mod arduino_toolchain;
mod arduino_upload;
mod commands;
mod config_engine;
mod config_store;
mod db;
mod device_verification;
mod firmware;
mod firmware_generator;
mod gemini;
mod gateway_firmware;
mod machine_registry;
mod registry;
mod runtime;
mod runtime_firmware;
mod runtime_protocol;
mod sensor_drivers;
mod serial_runtime;
mod telemetry;
mod wiring;

#[cfg_attr(mobile,tauri::mobile_entry_point)]
pub fn run(){
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::health,
            commands::list_serial_ports,
            commands::list_boards,
            commands::list_sensors,
            commands::list_machine_signals,
            commands::list_machine_types,
            commands::validate_configuration,
            commands::save_configuration,
            commands::save_active_configuration,
            commands::load_configuration,
            commands::delete_configuration,
            commands::generate_wiring,
            commands::build_firmware_spec,
            commands::generate_firmware,
            commands::generate_gateway_firmware,
            commands::generate_runtime_firmware,
            commands::plan_sensor_drivers,
            commands::encode_telemetry,
            commands::open_serial,
            commands::validate_runtime_frame,
            commands::bridge_runtime_frame,
            commands::initialize_database,
            commands::arduino_cli_status,
            commands::compile_gateway_firmware,
            commands::detect_arduino_boards,
            commands::upload_gateway_firmware,
            commands::verify_uploaded_device,
            commands::load_gemini_settings,
            commands::save_gemini_settings,
            commands::verify_firmware_with_gemini,
            commands::compile_firmware_source
        ])
        .run(tauri::generate_context!())
        .expect("error while running Maintain.ai DeviceOS");
}
