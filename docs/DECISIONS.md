# Architecture Decisions

## ADR-001 Desktop-first

Tauri 2 + React + TypeScript + Rust because DeviceOS needs local USB/serial, toolchain execution, bootloader/upload and filesystem access.

## ADR-002 Local-first

SQLite is the MVP store. Maintain.ai 3 is optional.

## ADR-003 Arduino CLI

Use the official Arduino CLI/toolchain rather than writing a compiler.

## ADR-004 Configuration-driven firmware

Use tested templates/modules rather than arbitrary C++ generation.

## ADR-005 Registry-driven hardware

Boards, sensors, capabilities, pins and compatibility rules live in versioned registries outside the UI.

## ADR-006 Adapter-based integration

HTTP, MQTT, WebSocket and Maintain.ai are adapters around normalized telemetry.

## ADR-007 Machine compatibility

DeviceOS aligns with Maintain.ai 3 machine and signal semantics when integrated, but does not own maintenance history or ML outcomes.
