export type ParameterDefinition = {
  id: string;
  name: string;
  signal_type: string;
  unit: string;
  data_type: 'float' | 'integer' | 'boolean';
  min?: number;
  max?: number;
};

export type PhysicalPinDefinition = {
  id: string;
  label: string;
  role: 'power' | 'ground' | 'signal';
  signal_type?: string;
};

export type BoardDefinition = {
  id: string;
  name: string;
  fqbn: string;
  digital_pins: number[];
  analog_pins: number[];
  buses: string[];
};

export type SensorDefinition = {
  id: string;
  name: string;
  category: string;
  protocol: string;
  parameters: ParameterDefinition[];
  required_digital_pins: number;
  required_analog_pins: number;
  buses: string[];
  physical_pins: PhysicalPinDefinition[];
};

export type MachineSignalDefinition = {
  id: string;
  name: string;
  category: string;
  units: string[];
};

const parameter = (id: string, name: string, signal_type: string, unit: string, data_type: ParameterDefinition['data_type'], min?: number, max?: number): ParameterDefinition => ({ id, name, signal_type, unit, data_type, min, max });
const power = (id='vcc', label='VCC'): PhysicalPinDefinition => ({ id, label, role: 'power' });
const ground = (id='gnd', label='GND'): PhysicalPinDefinition => ({ id, label, role: 'ground' });
const signal = (id: string, label: string, signal_type='digital'): PhysicalPinDefinition => ({ id, label, role: 'signal', signal_type });

export const initialBoards: BoardDefinition[] = [
  { id: 'arduino-uno-r3', name: 'Arduino Uno R3', fqbn: 'arduino:avr:uno', digital_pins: Array.from({ length: 14 }, (_, i) => i), analog_pins: [14,15,16,17,18,19], buses: ['uart','i2c','spi','gpio','analog'] },
  { id: 'arduino-nano', name: 'Arduino Nano', fqbn: 'arduino:avr:nano', digital_pins: Array.from({ length: 14 }, (_, i) => i), analog_pins: [14,15,16,17,18,19], buses: ['uart','i2c','spi','gpio','analog'] },
];

export const initialSensors: SensorDefinition[] = [
  { id: 'dht11', name: 'DHT11', category: 'environmental', protocol: 'single-wire', parameters: [parameter('temperature','Temperature','temperature','°C','float'), parameter('humidity','Humidity','humidity','%','float',0,100)], required_digital_pins: 1, required_analog_pins: 0, buses: ['gpio'], physical_pins: [power(), ground(), signal('data','DATA','digital')] },
  { id: 'sw420', name: 'SW-420 Vibration', category: 'vibration', protocol: 'digital', parameters: [parameter('vibration','Vibration','vibration','state','boolean')], required_digital_pins: 1, required_analog_pins: 0, buses: ['gpio'], physical_pins: [power(), ground(), signal('do','DO','digital')] },
  { id: 'current-sensor', name: 'Current Sensor', category: 'electrical', protocol: 'analog', parameters: [parameter('current','Current','current','A','float',0)], required_digital_pins: 0, required_analog_pins: 1, buses: ['analog'], physical_pins: [power(), ground(), signal('out','OUT','analog')] },
  { id: 'voltage-sensor', name: 'Voltage Sensor', category: 'electrical', protocol: 'analog', parameters: [parameter('voltage','Voltage','voltage','V','float',0)], required_digital_pins: 0, required_analog_pins: 1, buses: ['analog'], physical_pins: [power(), ground(), signal('out','OUT','analog')] },
  { id: 'pressure-sensor', name: 'Pressure Sensor', category: 'pressure', protocol: 'analog', parameters: [parameter('pressure','Pressure','pressure','bar','float',0)], required_digital_pins: 0, required_analog_pins: 1, buses: ['analog'], physical_pins: [power(), ground(), signal('out','OUT','analog')] },
  { id: 'flow-sensor', name: 'Flow Sensor', category: 'flow', protocol: 'pulse', parameters: [parameter('flow','Flow','flow','L/min','float',0)], required_digital_pins: 1, required_analog_pins: 0, buses: ['gpio'], physical_pins: [power(), ground(), signal('pulse','PULSE','digital')] },
  { id: 'speed-sensor', name: 'Speed Sensor', category: 'motion', protocol: 'pulse', parameters: [parameter('speed','Speed','speed','m/s','float',0)], required_digital_pins: 1, required_analog_pins: 0, buses: ['gpio'], physical_pins: [power(), ground(), signal('pulse','PULSE','digital')] },
  { id: 'rpm-sensor', name: 'RPM Sensor', category: 'motion', protocol: 'pulse', parameters: [parameter('rpm','RPM','rpm','rpm','float',0)], required_digital_pins: 1, required_analog_pins: 0, buses: ['gpio'], physical_pins: [power(), ground(), signal('pulse','PULSE','digital')] },
  { id: 'load-sensor', name: 'Load Sensor', category: 'mechanical', protocol: 'analog', parameters: [parameter('load','Load','load','%','float',0,100)], required_digital_pins: 0, required_analog_pins: 1, buses: ['analog'], physical_pins: [power(), ground(), signal('out','OUT','analog')] },
  { id: 'distance-sensor', name: 'Distance Sensor', category: 'position', protocol: 'digital', parameters: [parameter('distance','Distance','distance','mm','float',0)], required_digital_pins: 2, required_analog_pins: 0, buses: ['gpio'], physical_pins: [power(), ground(), signal('trig','TRIG','digital'), signal('echo','ECHO','digital')] },
  { id: 'generic-analog-input', name: 'Generic Analog Input', category: 'generic', protocol: 'analog', parameters: [parameter('analog','Analog','analog','raw','integer',0,1023)], required_digital_pins: 0, required_analog_pins: 1, buses: ['analog'], physical_pins: [power(), ground(), signal('in','IN','analog')] },
  { id: 'generic-digital-input', name: 'Generic Digital Input', category: 'generic', protocol: 'digital', parameters: [parameter('digital','Digital','digital','state','boolean')], required_digital_pins: 1, required_analog_pins: 0, buses: ['gpio'], physical_pins: [power(), ground(), signal('in','IN','digital')] },
];

export const machineSignals: MachineSignalDefinition[] = [
  { id: 'temperature', name: 'Temperature', category: 'thermal', units: ['°C','°F','K'] },
  { id: 'humidity', name: 'Humidity', category: 'environmental', units: ['%'] },
  { id: 'vibration', name: 'Vibration', category: 'condition', units: ['state','mm/s','g'] },
  { id: 'current', name: 'Current', category: 'electrical', units: ['A'] },
  { id: 'voltage', name: 'Voltage', category: 'electrical', units: ['V'] },
  { id: 'pressure', name: 'Pressure', category: 'process', units: ['bar','psi','kPa'] },
  { id: 'flow', name: 'Flow', category: 'process', units: ['L/min','m³/h'] },
  { id: 'speed', name: 'Speed', category: 'motion', units: ['m/s'] },
  { id: 'load', name: 'Load', category: 'mechanical', units: ['%','kg','N'] },
  { id: 'rpm', name: 'RPM', category: 'motion', units: ['rpm'] },
  { id: 'distance', name: 'Distance', category: 'position', units: ['mm','cm','m'] },
];
