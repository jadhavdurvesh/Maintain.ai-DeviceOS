import React, { Component, type ReactNode, useEffect, useMemo, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './styles.css';

type Board={id:string;name:string;fqbn:string;digital_pins:number[];analog_pins:number[];buses:string[]};
type PhysicalPin={id:string;label:string;role:'power'|'ground'|'signal';signal_type?:string};
type Parameter={id:string;name:string;signal_type:string;unit:string;data_type:string;min?:number;max?:number};
type Sensor={id:string;name:string;category:string;protocol:string;parameters:Parameter[];required_digital_pins:number;required_analog_pins:number;buses:string[];physical_pins:PhysicalPin[]};
type Machine={id:string;name:string;category:string;signal_ids:string[];components:{id:string;name:string;signal_ids:string[]}[]};
type Selection={sensor_id:string;parameter_ids:string[]};
type Wiring={sensor_id:string;parameter_id:string;signal_type:string;pin:number;pin_type:string;notes?:string};
type DetectedBoard={address:string;port_type:string;protocol:string;board_name?:string;fqbn?:string};

const signalIcon:Record<string,string>={temperature:'°',humidity:'%',vibration:'≈',current:'A',voltage:'V',pressure:'P',flow:'↝',speed:'S',load:'L',rpm:'R',distance:'↔',digital:'D',analog:'∿'};

class ErrorBoundary extends Component<{children:ReactNode},{error:string|null}>{
 state={error:null as string|null};
 static getDerivedStateFromError(error:unknown){return {error:error instanceof Error?error.message:String(error)}}
 render(){if(this.state.error)return <div className="fatal"><div className="fatal-card"><div className="fatal-mark">!</div><p className="eyebrow">DEVICEOS RECOVERY</p><h1>The workspace could not be rendered.</h1><p>{this.state.error}</p><button className="button primary" onClick={()=>location.reload()}>Reload DeviceOS</button></div></div>;return this.props.children}
}

function App(){
 const[machines,setMachines]=useState<Machine[]>([]); const[boards,setBoards]=useState<Board[]>([]); const[sensors,setSensors]=useState<Sensor[]>([]);
 const[machine,setMachine]=useState('induction-motor'); const[board,setBoard]=useState('arduino-uno-r3'); const[selected,setSelected]=useState<Selection[]>([]);
 const[wiring,setWiring]=useState<Wiring[]>([]); const[detected,setDetected]=useState<DetectedBoard[]>([]); const[buildResult,setBuildResult]=useState<any>(null);
 const[activeView,setActiveView]=useState<'overview'|'hardware'|'wiring'|'build'>('overview'); const[showMachinePicker,setShowMachinePicker]=useState(false);
 const[notice,setNotice]=useState(''); const[loading,setLoading]=useState(true); const[busy,setBusy]=useState(false); const[uploading,setUploading]=useState(false);

 useEffect(()=>{let alive=true;Promise.all([invoke<Board[]>('list_boards'),invoke<Sensor[]>('list_sensors'),invoke<Machine[]>('list_machine_types')]).then(([b,s,m])=>{if(!alive)return;setBoards(b);setSensors(s);setMachines(m);if(m.length&&!m.some(x=>x.id==='induction-motor'))setMachine(m[0].id);setLoading(false)}).catch(e=>{if(!alive)return;setNotice(`DeviceOS services unavailable: ${String(e)}`);setLoading(false)});return()=>{alive=false}},[]);

 const currentMachine=machines.find(m=>m.id===machine); const currentBoard=boards.find(b=>b.id===board);
 const chosenSensors=selected.map(x=>({...x,sensor:sensors.find(s=>s.id===x.sensor_id)})).filter(x=>x.sensor) as Array<Selection&{sensor:Sensor}>;
 const measurements=useMemo(()=>selected.reduce((n,x)=>n+x.parameter_ids.length,0),[selected]);
 const usedPins=new Set(wiring.map(x=>`${x.pin_type}:${x.pin}`));
 const request=()=>({machine_type_id:machine,board_id:board,sensors:selected});
 const config=(assignments=wiring)=>({id:`device-${machine}`,device_id:null,machine_type_id:machine,board_id:board,sensors:selected,assignments,schema_version:'1.0',definition_version:'1.0'});

 const chooseMachine=(id:string)=>{setMachine(id);setSelected([]);setWiring([]);setBuildResult(null);setNotice('');setShowMachinePicker(false)};
 const toggleSensor=(s:Sensor)=>setSelected(p=>p.some(x=>x.sensor_id===s.id)?p.filter(x=>x.sensor_id!==s.id):[...p,{sensor_id:s.id,parameter_ids:s.parameters.map(x=>x.id)}]);
 const toggleParam=(sid:string,pid:string)=>setSelected(p=>p.map(x=>x.sensor_id===sid?{...x,parameter_ids:x.parameter_ids.includes(pid)?x.parameter_ids.filter(id=>id!==pid):[...x, pid]}:x));
 const validate=async()=>{try{const r:any=await invoke('validate_configuration',{request:request()});if(!r.valid){setNotice(r.errors?.join(' ')||'The configuration needs attention.');return false}setWiring(r.assignments||[]);setNotice(r.warnings?.join(' ')||'Configuration validated.');return true}catch(e){setNotice(`Compatibility check failed: ${String(e)}`);return false}};
 const detect=async()=>{try{const r=await invoke<DetectedBoard[]>('detect_arduino_boards');setDetected(r);setNotice(r.length?`Arduino detected on ${r[0].address}.`:'No compatible Arduino detected. Connect it by USB and scan again.')}catch(e){setNotice(`Detection failed: ${String(e)}`)}};
 const build=async()=>{setBusy(true);try{const ok=await validate();if(!ok)return;try{await invoke('save_active_configuration',{config:config(wiring)})}catch{}const r:any=await invoke('compile_gateway_firmware',{config:config(wiring)});setBuildResult(r);setNotice(r.success?'Firmware compiled successfully.':'Firmware compilation failed. Review the build output below.');setActiveView('build')}catch(e){setNotice(`Build failed: ${String(e)}`)}finally{setBusy(false)}};
 const upload=async()=>{setUploading(true);try{const r=await invoke<DetectedBoard[]>('detect_arduino_boards');setDetected(r);if(!r.length){setNotice('Connect the Arduino before uploading.');return}if(!buildResult?.success){setNotice('Build the firmware successfully before uploading.');return}const target=r.find(x=>x.fqbn===currentBoard?.fqbn)||r[0];const result:any=await invoke('upload_gateway_firmware',{port:target.address,fqbn:target.fqbn||currentBoard?.fqbn,build_dir:buildResult.output_dir});setNotice(result.success?'Firmware uploaded successfully.':result.message||'Upload failed.')}catch(e){setNotice(`Upload failed: ${String(e)}`)}finally{setUploading(false)}};
 const pinLabel=(w:Wiring)=>w.pin_type==='analog'?`A${Math.max(0,w.pin-14)}`:`D${w.pin}`;
 const wiringFor=(s:Sensor)=>{const rows=wiring.filter(w=>w.sensor_id===s.id);const assignment=(p:PhysicalPin)=>p.role==='signal'?rows.find(w=>w.parameter_id===s.parameters.find(x=>x.signal_type===p.signal_type)?.id):undefined;return {rows,assignment}};

 if(loading)return <div className="loading-screen"><div className="loading-logo">M</div><div><b>Maintain.ai DeviceOS</b><span>Loading device workspace…</span></div></div>;

 return <div className="app-shell">
  <aside className="rail">
   <div className="brand"><div className="brand-mark">M</div><div><b>Maintain.ai</b><span>DeviceOS</span></div></div>
   <div className="rail-label">WORKSPACE</div>
   <nav className="nav">
    <button className={activeView==='overview'?'active':''} onClick={()=>setActiveView('overview')}><span>⌂</span>Overview</button>
    <button className={activeView==='hardware'?'active':''} onClick={()=>setActiveView('hardware')}><span>◈</span>Hardware</button>
    <button className={activeView==='wiring'?'active':''} onClick={()=>setActiveView('wiring')}><span>⌁</span>Wiring</button>
    <button className={activeView==='build'?'active':''} onClick={()=>setActiveView('build')}><span>▣</span>Device build</button>
   </nav>
   <div className="rail-spacer"/>
   <div className="rail-status"><i className={detected.length?'live':''}/><div><b>{detected.length?'Arduino connected':'No device connected'}</b><span>{detected.length?detected[0].address:'USB device can be detected from Device build'}</span></div></div>
   <div className="version">DeviceOS 0.1.0</div>
  </aside>

  <main className="main">
   <header className="topbar">
    <div><p className="eyebrow">DEVICE WORKSPACE</p><h1>{currentMachine?.name||'Machine workspace'}</h1><div className="context-line"><span>{currentMachine?.category||'Machine'}</span><span>•</span><span>{currentBoard?.name||'Select controller'}</span></div></div>
    <div className="top-actions"><button className="machine-switch" onClick={()=>setShowMachinePicker(true)}><span className="machine-mini">⚙</span><span><b>Machine</b><small>Change machine</small></span><strong>⌄</strong></button><button className="scan-button" onClick={detect}>Scan USB</button></div>
   </header>

   <div className="workspace-bar"><div className="status-strip"><span className="status-pill"><i className={detected.length?'live':''}/>{detected.length?'Device connected':'Ready for configuration'}</span><span>{selected.length} sensors</span><span>{measurements} measurements</span><span>{new Set(wiring.map(w=>`${w.pin_type}:${w.pin}`)).size} signal pins</span></div><div className="workspace-actions">{notice&&<span className="inline-notice">{notice}</span>}</div></div>

   {showMachinePicker&&<div className="modal-backdrop" onMouseDown={()=>setShowMachinePicker(false)}><div className="machine-modal" onMouseDown={e=>e.stopPropagation()}><div className="modal-head"><div><p className="eyebrow">MACHINE PROFILE</p><h2>Select the asset you are configuring</h2><p>Changing the machine resets its sensor selection and wiring plan.</p></div><button className="close" onClick={()=>setShowMachinePicker(false)}>×</button></div><div className="machine-picker">{machines.map(m=><button key={m.id} className={m.id===machine?'selected':''} onClick={()=>chooseMachine(m.id)}><div className="machine-symbol">⚙</div><div><b>{m.name}</b><span>{m.category}</span><small>{m.signal_ids.length} machine signals · {m.components.length} components</small></div><i>{m.id===machine?'✓':'→'}</i></button>)}</div></div></div>}

   {activeView==='overview'&&<section className="content-grid">
    <div className="hero-panel"><div><p className="eyebrow">ACTIVE ASSET</p><h2>{currentMachine?.name}</h2><p>Configure the monitoring hardware for this machine. DeviceOS keeps the physical sensor pins, controller allocation and firmware configuration synchronized.</p><div className="hero-actions"><button className="button primary" onClick={()=>setActiveView('hardware')}>Configure hardware <span>→</span></button><button className="button secondary" onClick={()=>setActiveView('wiring')}>View wiring</button></div></div><div className="machine-visual"><div className="machine-core">⚙</div>{currentMachine?.signal_ids.slice(0,6).map((id,i)=><span key={id} className={`signal-orbit o${i}`}>{signalIcon[id]||'•'} {id}</span>)}</div></div>
    <div className="section-heading"><div><p className="eyebrow">CONFIGURATION</p><h3>Current device plan</h3></div><span className="quiet">Live configuration</span></div>
    <div className="summary-grid"><div className="summary-card"><span className="summary-icon">◈</span><div><small>Sensors</small><b>{selected.length}</b><span>{selected.length?'Hardware selected':'None selected yet'}</span></div></div><div className="summary-card"><span className="summary-icon">∿</span><div><small>Measurements</small><b>{measurements}</b><span>{currentMachine?.signal_ids.length||0} signals available</span></div></div><div className="summary-card"><span className="summary-icon">▦</span><div><small>Controller</small><b>{currentBoard?.name||'—'}</b><span>{currentBoard?`${currentBoard.digital_pins.length} digital · ${currentBoard.analog_pins.length} analog`:'Choose hardware'}</span></div></div><div className="summary-card"><span className="summary-icon">✓</span><div><small>Firmware</small><b>{buildResult?.success?'Ready':'Not built'}</b><span>{buildResult?.success?'Build artifact available':'Build when wiring is ready'}</span></div></div></div>
    <div className="section-heading"><div><p className="eyebrow">MACHINE SIGNALS</p><h3>What this machine can monitor</h3></div></div><div className="signal-grid">{currentMachine?.signal_ids.map(id=><div className="signal-card" key={id}><span>{signalIcon[id]||'•'}</span><div><b>{id[0].toUpperCase()+id.slice(1)}</b><small>Machine signal</small></div></div>)}</div>
   </section>}

   {activeView==='hardware'&&<section className="content-grid"><div className="section-heading"><div><p className="eyebrow">HARDWARE</p><h2>Build your sensing stack</h2><p>Select the controller and physical sensors. There is no artificial step-by-step flow — changes apply directly to this device workspace.</p></div><div className="hardware-actions"><button className="button secondary" onClick={detect}>Detect Arduino</button><button className="button primary" onClick={()=>setActiveView('wiring')}>Open wiring <span>→</span></button></div></div>
    <div className="hardware-layout"><div><div className="panel-title"><b>Controller</b><span>{currentBoard?.name||'Select one'}</span></div><div className="board-grid">{boards.map(b=><button key={b.id} className={`board-card ${b.id===board?'selected':''}`} onClick={()=>{setBoard(b.id);setWiring([]);setBuildResult(null)}}><div className="board-art"><div className="usb-label">USB</div><div className="chip">MCU</div><div className="board-pins left">{b.analog_pins.slice(0,4).map(p=><i key={p}>A{p-14}</i>)}</div><div className="board-pins right">{b.digital_pins.slice(0,8).map(p=><i key={p}>D{p}</i>)}</div></div><div className="card-copy"><b>{b.name}</b><span>{b.digital_pins.length} digital · {b.analog_pins.length} analog</span><small>{b.buses.join(' · ')}</small></div><i className="card-check">{b.id===board?'✓':''}</i></button>)}</div></div>
    <div><div className="panel-title"><b>Sensors</b><span>{selected.length} selected</span></div><div className="sensor-list">{sensors.map(s=>{const active=selected.some(x=>x.sensor_id===s.id);const compatible=currentMachine?.signal_ids.some(id=>s.parameters.some(p=>p.signal_type===id));return <div key={s.id} className={`sensor-row ${active?'selected':''}`}><button onClick={()=>toggleSensor(s)} className="sensor-select"><span className="sensor-symbol">◈</span><span><b>{s.name}</b><small>{s.protocol} · {s.physical_pins.length} pins</small></span><i>{active?'✓':'+'}</i></button>{active&&<div className="parameter-row">{s.parameters.map(p=><button key={p.id} className={selected.find(x=>x.sensor_id===s.id)?.parameter_ids.includes(p.id)?'on':''} onClick={()=>toggleParam(s.id,p.id)}><span>{signalIcon[p.signal_type]||'•'}</span>{p.name}<small>{p.unit}</small></button>)}</div>}{!active&&compatible&&<span className="compat">Matches machine signal</span>}</div>})}</div></div></div>
   </section>}

   {activeView==='wiring'&&<section className="content-grid"><div className="section-heading"><div><p className="eyebrow">PHYSICAL WIRING</p><h2>Connect the device correctly</h2><p>Power and ground are physical connections and do not consume GPIO. Only signal wires are assigned to Arduino pins.</p></div><button className="button primary" onClick={async()=>{if(!selected.length){setNotice('Select at least one sensor first.');return}await validate()}}>Validate & map pins</button></div>
    <div className="wiring-layout"><div className="pinmap-panel"><div className="panel-title"><b>{currentBoard?.name}</b><span>Controller pin map</span></div><div className="arduino-large"><div className="arduino-usb">USB</div><div className="arduino-mcu">ATmega<br/>MCU</div><div className="arduino-name">{currentBoard?.name}</div><div className="pin-column analog">{currentBoard?.analog_pins.map(p=><span key={p} className={usedPins.has(`analog:${p}`)?'used':''}>A{p-14}</span>)}</div><div className="pin-column digital">{currentBoard?.digital_pins.map(p=><span key={p} className={usedPins.has(`digital:${p}`)?'used':''}>D{p}</span>)}</div></div><div className="pin-legend"><span><i className="legend-signal"/>Signal</span><span><i className="legend-power"/>5V / VCC</span><span><i className="legend-ground"/>GND</span></div></div>
     <div className="connection-stack">{chosenSensors.length===0?<div className="empty-panel"><div>⌁</div><b>No sensors selected</b><span>Choose sensors in Hardware to generate their exact physical connections.</span><button className="button primary" onClick={()=>setActiveView('hardware')}>Choose sensors</button></div>:chosenSensors.map(({sensor})=>{const d=wiringFor(sensor);return <div className="connection-card" key={sensor.id}><div className="connection-top"><div className="sensor-symbol">◈</div><div><b>{sensor.name}</b><span>{sensor.protocol}</span></div><strong>{d.rows.length} signal assignment{d.rows.length===1?'':'s'}</strong></div><div className="physical-rows">{sensor.physical_pins.map(p=>{const a=d.assignment(p);return <div key={p.id} className={`physical-row ${p.role}`}><span className="role-tag">{p.role==='signal'?'SIGNAL':p.role.toUpperCase()}</span><b>{p.label}</b><span className="arrow">→</span><strong>{p.role==='signal'?(a?pinLabel(a):'Run validation'):(p.role==='power'?'Arduino 5V / VCC':'Arduino GND')}</strong></div>})}</div>{sensor.id==='dht11'&&<div className="important-note"><b>DHT11 wiring</b><span>This sensor has 3 physical pins: VCC, DATA and GND. Temperature and humidity are two measurements carried over the single DATA wire, so both measurements correctly share one Arduino GPIO pin.</span></div>}</div>})}</div>
    </div></section>}

   {activeView==='build'&&<section className="content-grid"><div className="section-heading"><div><p className="eyebrow">DEVICE BUILD</p><h2>Deploy the configured machine</h2><p>Generate Gateway-compatible firmware from the exact hardware plan, then detect and upload it to the Arduino.</p></div><div className="build-top-actions"><button className="button secondary" onClick={detect}>Scan USB</button><button className="button primary" disabled={busy} onClick={build}>{busy?'Building…':'Build firmware'} <span>→</span></button></div></div>
    <div className="build-layout"><div className="build-status-card"><div className={`build-status-icon ${buildResult?.success?'ready':''}`}>{buildResult?.success?'✓':'▣'}</div><p className="eyebrow">DEPLOYMENT STATUS</p><h3>{buildResult?.success?'Firmware ready':'Configuration ready'}</h3><p>{currentMachine?.name} · {selected.length} sensors · {measurements} measurements</p><div className="deployment-list"><div><span>Machine</span><b>{currentMachine?.name}</b></div><div><span>Controller</span><b>{currentBoard?.name}</b></div><div><span>Signal pins</span><b>{new Set(wiring.map(w=>`${w.pin_type}:${w.pin}`)).size}</b></div><div><span>USB</span><b>{detected.length?detected[0].address:'Not detected'}</b></div></div>{detected.length>0&&buildResult?.success&&<button className="button primary wide" disabled={uploading} onClick={upload}>{uploading?'Uploading…':'Upload firmware to Arduino'} <span>↑</span></button>}{detected.length===0&&<button className="button secondary wide" onClick={detect}>Detect Arduino</button>}</div><div className="build-output-panel"><div className="panel-title"><b>Build output</b><span>{buildResult?.success?'Successful':'Awaiting build'}</span></div>{buildResult?<pre>{buildResult.stdout||''}{buildResult.stderr||''}</pre>:<div className="empty-output"><span>▣</span><b>No build has been run yet</b><small>Validate your wiring, then build the firmware from this workspace.</small></div>}</div></div>{notice&&<div className="notice-panel">{notice}</div>}</section>}
  </main>
 </div>;
}

export default function Root(){return <ErrorBoundary><App/></ErrorBoundary>}
