# Maintain.ai DeviceOS Architecture

## 1. Position

DeviceOS is a standalone desktop application and the hardware/edge provisioning layer of the Maintain.ai ecosystem.

## 2. Logical architecture

```text
┌─────────────────────────────────────────────────────────────┐
│                 Maintain.ai DeviceOS                        │
│                                                             │
│  React + TypeScript UI                                     │
│      │                                                      │
│      ▼                                                      │
│  Application Services                                       │
│      ├── Project Service                                    │
│      ├── Machine/Device Service                             │
│      ├── Registry Service                                   │
│      ├── Pin Assignment Engine                              │
│      ├── Validation Engine                                  │
│      ├── Firmware Engine                                    │
│      ├── Telemetry Service                                  │
│      └── Upload/Safety Guard                                │
│      │                                                      │
│      ▼                                                      │
│  Tauri / Rust Native Layer                                  │
│      ├── USB + serial discovery                             │
│      ├── serial transport                                   │
│      ├── Arduino CLI execution                              │
│      ├── upload/bootloader                                  │
│      ├── filesystem                                         │
│      └── OS integration                                     │
│      │                                                      │
│      ├──────────────┐                                       │
│      ▼              ▼                                       │
│   SQLite       Firmware Workspace                           │
└──────┬──────────────────────────────────────────────────────┘
       │ explicit adapter / normalized telemetry
       ▼
┌─────────────────────────────────────────────────────────────┐
│ Maintain.ai 3                                               │
│ Machines · telemetry · anomaly/degradation · alerts         │
│ maintenance · work orders · history · AI/ML                 │
└─────────────────────────────────────────────────────────────┘
```

## 3. Separation of concerns

### UI
Presentation, wizard state, forms, dashboards and visualization.

### Application services
Domain rules, project lifecycle, registries, validation, pin allocation, build orchestration and telemetry normalization.

### Rust native layer
Local hardware access, serial operations, Arduino CLI process control, upload/bootloader operations and approved filesystem access.

### Firmware layer
Versioned tested modules and templates. Configuration fills validated template parameters.

### Persistence
SQLite contains local domain state, builds, uploads and logs. Generated source/binaries live in an exportable workspace.

## 4. Failure boundaries

Distinguish:
- discovery failure
- unsupported board
- serial failure
- invalid configuration
- pin conflict
- missing dependency
- compile failure
- upload failure
- handshake failure
- sensor verification failure

Compilation success does not imply upload success; upload success does not imply sensor verification success.
