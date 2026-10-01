# Telemetry & Maintain.ai 3 Integration

## Boundary

DeviceOS owns physical acquisition, device configuration and normalization.

Maintain.ai 3 owns machine intelligence and operational maintenance records.

## Standard telemetry envelope

```json
{
  "schema_version": "1.0",
  "device_id": "GW-00042",
  "machine_id": "CNC-004",
  "timestamp": 1727790000,
  "sequence": 42,
  "measurements": {
    "temperature": 28.4,
    "humidity": 63.0,
    "vibration": 0
  },
  "source": "deviceos",
  "firmware_version": "1.0.0"
}
```

## Adapter targets

- local serial/logging
- HTTP
- MQTT
- WebSocket
- Maintain.ai

MVP transport is serial-first.

## Maintain.ai 3 path

```text
DeviceOS device
   ↓
authenticated ingestion
   ↓
Maintain.ai sensor reading
   ↓
machine behavioural state
   ↓
anomaly/degradation evidence
   ↓
fault/alert
   ↓
maintenance/work order
   ↓
technician outcome
```

DeviceOS should feed this path without duplicating operational records.

## Device credentials

Maintain.ai 3 has per-machine live sensor integration using a device key and authenticated ingestion. DeviceOS should support provisioning such credentials while minimizing exposure in logs and exports.
