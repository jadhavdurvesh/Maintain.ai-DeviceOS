# Maintain.ai 3 Machine Compatibility

## Purpose

Maintain.ai 3 already has a machine taxonomy and machine-specific engineering metadata. DeviceOS should align with it so a configured physical device can feed the same semantic machine model.

## Base machine types

The Machines area includes:
- induction motor
- pump
- conveyor
- compressor
- other

It also exposes specialized machine types.

## Specialized machine profiles

The current machine-profile model includes examples such as:
- CNC machine
- lathe
- milling machine
- drilling machine
- grinding machine
- hydraulic press
- injection molding machine
- packaging machine
- generator
- transformer
- boiler
- furnace/oven
- HVAC
- fan/blower
- gearbox
- turbine
- crane/hoist

Robot architectures include:
- articulated
- SCARA
- Cartesian/Gantry
- Delta/Parallel
- cylindrical
- spherical/polar
- collaborative robot
- humanoid

## Typical signal vocabulary

Examples from the machine profiles:

CNC:
- spindle RPM/load
- vibration
- temperature
- tool wear
- coolant state

Induction motor:
- current
- voltage
- RPM
- winding/bearing temperature
- vibration
- power factor

Pump:
- pressure
- flow
- motor current
- vibration
- bearing temperature

Robots:
- joint position
- joint torque/current
- controller temperature
- force/speed depending on architecture

## Safety vocabulary

Maintain.ai 3 models signal-level warning/shutdown policy metadata and machine-specific hazards. DeviceOS can carry relevant signal and wiring metadata, but a general-purpose Arduino/ESP board must not be represented as a certified safety controller.

## Semantic mapping

Where possible, use shared signal names:
- temperature
- vibration
- current
- voltage
- pressure
- flow
- RPM
- load
- position
- force

When a machine-specific name differs, store an explicit mapping instead of silently changing meaning.

## Ownership

DeviceOS must not become the competing source of truth for:
- faults
- alerts
- work orders
- maintenance history
- technician outcomes
- health scores
- ML outcomes
