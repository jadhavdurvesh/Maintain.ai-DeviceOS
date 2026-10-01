# Project Scope

## Product identity

**Product:** Maintain.ai DeviceOS

**Purpose:** Desktop hardware/device provisioning and edge-configuration software for industrial maintenance instrumentation.

## In scope

- physical board discovery
- supported board identification and manual selection
- machine/device association
- sensor and parameter selection
- pin assignment and conflict validation
- wiring guidance and deterministic diagrams
- firmware generation from tested templates
- board package/library management
- Arduino CLI compile/upload
- serial monitor and handshake
- live sensor verification
- local persistence
- firmware versioning and rollback
- project export/import
- telemetry normalization
- Maintain.ai integration adapter

## Out of MVP scope

- cloud fleet management
- remote/OTA deployment
- arbitrary AI-generated firmware
- custom compiler
- mobile app
- Kubernetes/microservices
- broad permissions system
- every board/sensor
- speculative infrastructure

## MVP acceptance criterion

A technician can plug in an Arduino Uno/Nano, select a machine, configure supported sensors, accept or override validated pins, follow wiring guidance, generate firmware, compile, upload, complete a real handshake, observe live readings, and save/export the project.

## Scope guardrail

New work must directly improve device provisioning, physical instrumentation, firmware lifecycle, verification, telemetry, safety, or Maintain.ai integration.
