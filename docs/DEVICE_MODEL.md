# DeviceOS Domain Model

## Hierarchy

```text
Maintain.ai organization (optional integration context)
└── Machine
    └── Device
        ├── Board
        ├── Sensors
        │   └── Parameters
        ├── Pin Assignments
        ├── Configuration Versions
        ├── Firmware Builds
        ├── Upload History
        └── Telemetry
```

## Machine

When integrated, the machine identity should align with Maintain.ai 3.

Fields:
- machine_code
- name
- category
- manufacturer
- model_number
- location
- department
- criticality
- operating_hours
- optional Maintain.ai machine ID

DeviceOS must not become a competing owner of maintenance history.

## Device

A physical controller installed for a machine.

Suggested fields:
- device_id
- machine_id
- board_id
- serial_number
- port
- connection_state
- firmware_version
- configuration_version
- credential/key status
- created_at
- last_seen_at

## Sensor

Physical instrumentation definition with protocol, parameters, pin/capability requirements, power requirements, libraries, compatible boards and wiring rules.

## Parameter

A measurable sensor output.

Examples:
- DHT11 → temperature, humidity
- MPU6050 → acceleration X/Y/Z, gyroscope X/Y/Z
- SW-420 → vibration detected

## Configuration

Versioned immutable representation used for a firmware build.

It contains board, machine/device reference, sensors, pins, sampling, communications, relevant safety metadata and schema versions.

## Firmware build

Contains build ID, semantic version, config hash, registry/template versions, board/FQBN, dependencies, artifacts, checksums, status and timestamps.

## Upload history

Contains target device, selected build, port, detected board, result, tool output summary and verification result.
