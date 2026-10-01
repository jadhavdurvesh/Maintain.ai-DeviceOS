# Maintain.ai DeviceOS

Maintain.ai DeviceOS is the hardware provisioning, firmware, device verification, and machine-edge configuration layer for the Maintain.ai ecosystem.

It is a standalone desktop application. Core operation must not depend on Maintain.ai 3, cloud infrastructure, or an internet connection.

## Product workflow

**Connect → Identify → Configure → Validate → Wire → Generate → Compile → Upload → Verify → Save/Export**

## Relationship to Maintain.ai 3

DeviceOS owns the physical-device provisioning lifecycle.

Maintain.ai 3 owns the industrial maintenance-intelligence/application lifecycle.

```text
Sensors → Board/Edge Device → DeviceOS → normalized telemetry
                                           ↓
                                  Maintain.ai 3 adapter
                                           ↓
                         monitoring → anomaly/fault evidence
                                           ↓
                          alerts → maintenance → work orders
                                           ↓
                                   technician outcome
                                           ↓
                                  ML learning evidence
```

DeviceOS must remain useful without Maintain.ai 3.

## MVP

Windows-first desktop application using Tauri 2, React, TypeScript, Rust and SQLite.

Initial hardware:
- Arduino Uno
- Arduino Nano
- DHT11
- SW-420
- generic analog input
- generic digital input

Initial capabilities:
- USB/serial discovery
- manual board selection
- machine/device association
- automatic/manual pin assignment
- wiring instructions
- deterministic wiring diagram
- firmware generation from tested templates
- Arduino CLI compilation and upload
- serial monitor
- live verification
- project save/export

See `docs/` for the full architecture and implementation contract.
