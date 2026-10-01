import type { MachineSignalDefinition } from './registry';

export type MachineComponentDefinition = {
  id: string;
  name: string;
  signal_ids: string[];
};

export type MachineTypeDefinition = {
  id: string;
  name: string;
  category: string;
  components: MachineComponentDefinition[];
  signal_ids: string[];
};

export const initialMachineTypes: MachineTypeDefinition[] = [
  { id: 'induction-motor', name: 'Induction Motor', category: 'motor', components: [{ id: 'motor-drive', name: 'Motor / Drive', signal_ids: ['temperature','vibration','current','voltage','rpm','load'] }, { id: 'motor-bearings', name: 'Bearings', signal_ids: ['temperature','vibration'] }], signal_ids: ['temperature','vibration','current','voltage','rpm','load'] },
  { id: 'pump', name: 'Pump', category: 'fluid', components: [{ id: 'pump-drive', name: 'Pump Drive', signal_ids: ['current','voltage','rpm'] }, { id: 'pump-process', name: 'Pump Process', signal_ids: ['pressure','flow'] }, { id: 'pump-bearings', name: 'Bearings', signal_ids: ['temperature','vibration'] }], signal_ids: ['temperature','vibration','current','pressure','flow','rpm'] },
  { id: 'conveyor', name: 'Conveyor', category: 'material-handling', components: [{ id: 'conveyor-drive', name: 'Drive', signal_ids: ['current','voltage','rpm','load'] }, { id: 'conveyor-belt', name: 'Belt / Motion', signal_ids: ['speed','vibration'] }], signal_ids: ['temperature','vibration','current','speed','load'] },
  { id: 'compressor', name: 'Compressor', category: 'fluid', components: [{ id: 'compressor-drive', name: 'Drive', signal_ids: ['current','voltage','rpm'] }, { id: 'compressor-process', name: 'Process', signal_ids: ['pressure','flow','temperature'] }, { id: 'compressor-bearings', name: 'Bearings', signal_ids: ['temperature','vibration'] }], signal_ids: ['temperature','vibration','current','pressure','flow','rpm'] },
  { id: 'cnc-machine', name: 'CNC Machine', category: 'machine-tool', components: [{ id: 'cnc-spindle', name: 'Spindle', signal_ids: ['rpm','load','temperature','vibration'] }, { id: 'cnc-drive', name: 'Drive', signal_ids: ['current','voltage'] }, { id: 'cnc-motion', name: 'Motion', signal_ids: ['speed'] }], signal_ids: ['temperature','vibration','current','voltage','rpm','load','speed'] },
  { id: 'generator', name: 'Generator', category: 'power', components: [{ id: 'generator-engine', name: 'Engine', signal_ids: ['temperature','vibration','rpm','load'] }, { id: 'generator-electrical', name: 'Electrical', signal_ids: ['current','voltage'] }], signal_ids: ['temperature','vibration','current','voltage','rpm','load'] },
  { id: 'transformer', name: 'Transformer', category: 'power', components: [{ id: 'transformer-core', name: 'Core', signal_ids: ['temperature','vibration'] }, { id: 'transformer-electrical', name: 'Electrical', signal_ids: ['current','voltage','load'] }], signal_ids: ['temperature','vibration','current','voltage','load'] },
  { id: 'fan-blower', name: 'Fan / Blower', category: 'air', components: [{ id: 'fan-drive', name: 'Drive', signal_ids: ['current','voltage','rpm'] }, { id: 'fan-rotor', name: 'Rotor', signal_ids: ['speed','vibration','temperature'] }], signal_ids: ['temperature','vibration','current','voltage','rpm','speed'] },
];

export type SignalMapping = {
  sensor_id: string;
  parameter_id: string;
  machine_signal_id: string;
  component_id?: string;
};

export function canMapSignal(parameterSignal: string, machineSignal: MachineSignalDefinition): boolean {
  return parameterSignal === machineSignal.id;
}
