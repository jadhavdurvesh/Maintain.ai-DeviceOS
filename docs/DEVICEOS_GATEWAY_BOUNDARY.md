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

ESP32 direct mode is the intentional exception: ESP32 uses its built-in Wi-Fi and sends authenticated HTTPS telemetry directly to the same backend using a machine-specific device key provisioned by DeviceOS.
          ↓
   Maintain.ai 3
```

## Wire contract

The Gateway currently expects a generic serial-device interface and one JSON object per line. The current reference Arduino firmware emits a line-oriented reading object such as:

```json
{"reading_type":"temperature","value":28.4,"unit":"C"}
```

The Gateway adds device identity/authentication and forwards accepted readings to the backend.

DeviceOS firmware generation has a dedicated **Gateway-compatible output mode** for Arduino-class controllers. ESP32 has a separate **Direct Wi-Fi mode** that embeds its network provisioning and machine-specific device key so it can operate without a gateway or host computer.

## Important rule

DeviceOS must not add Maintain.ai user authentication or worker JWT handling to Arduino firmware.

The physical Arduino should emit sensor readings only; the Gateway handles device-key authentication and the backend bridge.

ESP32 is different by design: it is a networked controller, so DeviceOS provisions Wi-Fi and a machine-specific device key into its direct-cloud firmware. It still never receives a worker/user credential.

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
Gateway for Arduino-class controllers; DeviceOS may verify ESP32 over USB during provisioning

### Buffer/retry
Gateway for Arduino-class controllers; ESP32 uses its network transport

### Authenticate ingestion
Gateway for Arduino-class controllers; ESP32 uses its provisioned machine device key

### Upload to Maintain.ai
Gateway for Arduino-class controllers; ESP32 direct HTTPS

### Maintenance intelligence
Maintain.ai 3
