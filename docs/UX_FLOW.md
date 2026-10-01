# DeviceOS UX Flow

## Main navigation

Dashboard · Devices · Machines · Boards · Sensors · Projects · Firmware · Logs · Settings

## Primary wizard

```text
1. Connect
2. Board
3. Machine
4. Sensors
5. Pins
6. Wiring
7. Firmware
8. Upload
9. Test
10. Complete
```

## Connect

Show USB/serial devices, port, board identification and connection state. Provide manual board selection.

## Sensors

Select sensors and parameters. Show compatibility, required connections and dependencies.

## Pins

Show automatic assignments, conflicts, reserved pins and manual override.

## Wiring

Generate deterministic pin-to-pin instructions plus a visual wiring diagram.

## Firmware

Show configuration version, dependencies, compile output and build artifact.

## Upload

Show actual Arduino CLI/bootloader progress. Never fake progress.

## Test

Show serial handshake, connection state, live readings, units, raw values where relevant, and device-level diagnostics.

## Complete

Show configuration summary and save/export controls.
