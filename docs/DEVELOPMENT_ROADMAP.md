# Development Roadmap

## Phase 0 — Foundation
- Tauri 2
- React + TypeScript
- Rust command layer
- SQLite
- typed schemas
- CI
- logging
- test harness

## Phase 1 — Hardware discovery
- serial-port enumeration
- USB metadata
- Uno/Nano definitions
- manual fallback
- connection state

## Phase 2 — Sensor and machine model
- registries
- machine/device/project entities
- DHT11
- SW-420
- generic analog/digital
- parameter model

## Phase 3 — Configuration engine
- capability matching
- deterministic assignment
- conflict detection
- manual override
- wiring renderer

## Phase 4 — Firmware engine
- tested templates
- dependency manifest
- Arduino CLI
- compile logs
- artifact checksums

## Phase 5 — Upload and verification
- upload
- actual progress parsing
- reset/reconnect
- handshake
- serial monitor
- live readings

## Phase 6 — Persistence/export
- firmware history
- upload history
- logs
- import/export
- rollback

## Phase 7 — Maintain.ai adapter
- machine identity mapping
- device identity mapping
- telemetry envelope
- authenticated ingestion
- transport adapters

## Phase 8 — Expansion
- ESP32
- more sensors
- calibration
- industrial protocols
- cloud/fleet features only after local workflow is reliable
