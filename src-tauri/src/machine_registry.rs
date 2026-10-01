use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct MachineComponentDefinition { pub id: &'static str, pub name: &'static str, pub signal_ids: &'static [&'static str] }
#[derive(Debug, Clone, Serialize)]
pub struct MachineTypeDefinition { pub id: &'static str, pub name: &'static str, pub category: &'static str, pub components: &'static [MachineComponentDefinition], pub signal_ids: &'static [&'static str] }

const MOTOR_SIGNALS:&[&str]=&["temperature","vibration","current","voltage","rpm","load"];
const PUMP_SIGNALS:&[&str]=&["temperature","vibration","current","pressure","flow","rpm"];
const CONVEYOR_SIGNALS:&[&str]=&["temperature","vibration","current","speed","load"];
const COMPRESSOR_SIGNALS:&[&str]=&["temperature","vibration","current","pressure","flow","rpm"];
const CNC_SIGNALS:&[&str]=&["temperature","vibration","current","voltage","rpm","load","speed"];
const GENERATOR_SIGNALS:&[&str]=&["temperature","vibration","current","voltage","rpm","load"];
const TRANSFORMER_SIGNALS:&[&str]=&["temperature","vibration","current","voltage","load"];
const FAN_SIGNALS:&[&str]=&["temperature","vibration","current","voltage","rpm","speed"];
const MOTOR_COMPONENTS:&[MachineComponentDefinition]=&[MachineComponentDefinition{id:"motor-drive",name:"Motor / Drive",signal_ids:MOTOR_SIGNALS},MachineComponentDefinition{id:"motor-bearings",name:"Bearings",signal_ids:&["temperature","vibration"]}];
const PUMP_COMPONENTS:&[MachineComponentDefinition]=&[MachineComponentDefinition{id:"pump-drive",name:"Pump Drive",signal_ids:&["current","voltage","rpm"]},MachineComponentDefinition{id:"pump-process",name:"Pump Process",signal_ids:&["pressure","flow"]},MachineComponentDefinition{id:"pump-bearings",name:"Bearings",signal_ids:&["temperature","vibration"]}];
const CONVEYOR_COMPONENTS:&[MachineComponentDefinition]=&[MachineComponentDefinition{id:"conveyor-drive",name:"Drive",signal_ids:&["current","voltage","rpm","load"]},MachineComponentDefinition{id:"conveyor-belt",name:"Belt / Motion",signal_ids:&["speed","vibration"]}];
const COMPRESSOR_COMPONENTS:&[MachineComponentDefinition]=&[MachineComponentDefinition{id:"compressor-drive",name:"Drive",signal_ids:&["current","voltage","rpm"]},MachineComponentDefinition{id:"compressor-process",name:"Process",signal_ids:&["pressure","flow","temperature"]},MachineComponentDefinition{id:"compressor-bearings",name:"Bearings",signal_ids:&["temperature","vibration"]}];
const CNC_COMPONENTS:&[MachineComponentDefinition]=&[MachineComponentDefinition{id:"cnc-spindle",name:"Spindle",signal_ids:&["rpm","load","temperature","vibration"]},MachineComponentDefinition{id:"cnc-drive",name:"Drive",signal_ids:&["current","voltage"]},MachineComponentDefinition{id:"cnc-motion",name:"Motion",signal_ids:&["speed"]}];
const GENERATOR_COMPONENTS:&[MachineComponentDefinition]=&[MachineComponentDefinition{id:"generator-engine",name:"Engine",signal_ids:&["temperature","vibration","rpm","load"]},MachineComponentDefinition{id:"generator-electrical",name:"Electrical",signal_ids:&["current","voltage"]}];
const TRANSFORMER_COMPONENTS:&[MachineComponentDefinition]=&[MachineComponentDefinition{id:"transformer-core",name:"Core",signal_ids:&["temperature","vibration"]},MachineComponentDefinition{id:"transformer-electrical",name:"Electrical",signal_ids:&["current","voltage","load"]}];
const FAN_COMPONENTS:&[MachineComponentDefinition]=&[MachineComponentDefinition{id:"fan-drive",name:"Drive",signal_ids:&["current","voltage","rpm"]},MachineComponentDefinition{id:"fan-rotor",name:"Rotor",signal_ids:&["speed","vibration","temperature"]}];

pub fn machine_types()->Vec<MachineTypeDefinition>{vec![MachineTypeDefinition{id:"induction-motor",name:"Induction Motor",category:"motor",components:MOTOR_COMPONENTS,signal_ids:MOTOR_SIGNALS},MachineTypeDefinition{id:"pump",name:"Pump",category:"fluid",components:PUMP_COMPONENTS,signal_ids:PUMP_SIGNALS},MachineTypeDefinition{id:"conveyor",name:"Conveyor",category:"material-handling",components:CONVEYOR_COMPONENTS,signal_ids:CONVEYOR_SIGNALS},MachineTypeDefinition{id:"compressor",name:"Compressor",category:"fluid",components:COMPRESSOR_COMPONENTS,signal_ids:COMPRESSOR_SIGNALS},MachineTypeDefinition{id:"cnc-machine",name:"CNC Machine",category:"machine-tool",components:CNC_COMPONENTS,signal_ids:CNC_SIGNALS},MachineTypeDefinition{id:"generator",name:"Generator",category:"power",components:GENERATOR_COMPONENTS,signal_ids:GENERATOR_SIGNALS},MachineTypeDefinition{id:"transformer",name:"Transformer",category:"power",components:TRANSFORMER_COMPONENTS,signal_ids:TRANSFORMER_SIGNALS},MachineTypeDefinition{id:"fan-blower",name:"Fan / Blower",category:"air",components:FAN_COMPONENTS,signal_ids:FAN_SIGNALS}]}
