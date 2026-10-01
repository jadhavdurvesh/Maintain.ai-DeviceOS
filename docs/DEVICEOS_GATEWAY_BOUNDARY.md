# DeviceOS ↔ MAINTAIN-AI-IoT-Gateway Boundary

## Purpose

Maintain.ai DeviceOS and MAINTAIN-AI-IoT-Gateway are separate products with a clean handoff.

### DeviceOS

Owns:
- machine-aware configuration
- physical sensor selection
- parameter selection
- pin assignment
- wiring instructions
- firmware code generation
- dependency selection
- compile
- board detection
- firmware upload
- local device verification
- firmware/configuration history

### MAINTAIN-AI-IoT-Gateway

Owns:
- serial runtime connection
- multiple connected devices
- device pairing
- device-key authentication
- backend connectivity
- HTTPS ingestion
- retry logic
- offline queue/store-and-forward
- backend status
- telemetry forwarding to Maintain.ai

## Golden workflow

```
Technician
   ↓
Maintain.ai DeviceOS
   ├── Select machine
   ├── Select controller
   ├── Select sensors
   ├── Select parameters
   ├── Automatic pin assignment
   ├── Visual wiring
   ├── Generate firmware
   ├── Compile
   ├── Upload
   └── Verify
          ↓
   Arduino / ESP32
          ↓ USB / Serial
   MAINTAIN-AI-IoT-Gateway
          ↓ HTTPS
   Maintain.ai backend
          ↓
   Maintain.ai 3
```

## Wire contract

The Gateway currently expects a generic serial-device interface and one JSON object per line. The current reference Arduino firmware emits a line-oriented reading object such as:

```json
{"reading_type":"temperature","value":28.4,"unit":"C"}
```

The Gateway adds device identity/authentication and forwards accepted readings to the backend.

DeviceOS firmware generation must therefore have a dedicated **Gateway-compatible output mode**.

## Important rule

DeviceOS must not add Maintain.ai user authentication, worker JWT handling, or backend device-key authentication to Arduino firmware.

The physical Arduino should emit sensor readings only. Gateway handles the authenticated bridge.

## Responsibilities by lifecycle

### Configure
DeviceOS

### Generate
DeviceOS

### Compile
DeviceOS

### Flash
DeviceOS

### Read serial data
Gateway

### Buffer/retry
Gateway

### Authenticate ingestion
Gateway

### Upload to Maintain.ai
Gateway

### Maintenance intelligence
Maintain.ai 3
