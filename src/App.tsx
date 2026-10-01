import { useState } from 'react';

const steps = ['Connect', 'Board', 'Machine', 'Sensors', 'Pins', 'Wiring', 'Firmware', 'Upload', 'Test'];

function App() {
  const [active, setActive] = useState('Dashboard');
  const [step, setStep] = useState(0);

  const next = () => setStep((current) => Math.min(current + 1, steps.length - 1));

  return (
    <div className="app-shell">
      <aside className="sidebar">
        <div className="brand">
          <div className="brand-mark">M</div>
          <div><strong>Maintain.ai</strong><span>DeviceOS</span></div>
        </div>
        <nav>
          {['Dashboard', 'Devices', 'Machines', 'Boards', 'Sensors', 'Projects', 'Firmware', 'Logs', 'Settings'].map((item) => (
            <button className={active === item ? 'nav-item active' : 'nav-item'} onClick={() => setActive(item)} key={item}>
              <span className="nav-dot" />{item}
            </button>
          ))}
        </nav>
        <div className="sidebar-footer">
          <span className="status-dot" /> Local-first mode
          <small>DeviceOS 0.1.0</small>
        </div>
      </aside>

      <main className="main">
        <header className="topbar">
          <div><p className="eyebrow">HARDWARE PROVISIONING</p><h1>{active}</h1></div>
          <div className="connection"><span className="status-dot" /> No device connected</div>
        </header>

        <section className="hero-card">
          <div>
            <p className="eyebrow">DEVICE SETUP</p>
            <h2>Connect a device and start configuring.</h2>
            <p className="muted">Provision boards, map sensors, generate tested firmware and verify live telemetry without leaving the desktop.</p>
            <button className="primary" onClick={() => setActive('Devices')}>Connect device</button>
          </div>
          <div className="hero-orbit"><div className="orbit-core">USB</div><span className="orbit-node node-a">BOARD</span><span className="orbit-node node-b">SENSOR</span><span className="orbit-node node-c">DATA</span></div>
        </section>

        <section className="grid three">
          <article className="metric"><span>Connected devices</span><strong>0</strong><small>Ready to provision</small></article>
          <article className="metric"><span>Saved projects</span><strong>0</strong><small>Stored locally</small></article>
          <article className="metric"><span>Firmware builds</span><strong>0</strong><small>No build history yet</small></article>
        </section>

        <section className="setup-card">
          <div className="section-heading"><div><p className="eyebrow">PROVISIONING WORKFLOW</p><h3>{steps[step]}</h3></div><span>{step + 1} / {steps.length}</span></div>
          <div className="steps">{steps.map((item, index) => <button key={item} onClick={() => setStep(index)} className={index === step ? 'step current' : index < step ? 'step done' : 'step'}><i>{index + 1}</i><span>{item}</span></button>)}</div>
          <div className="workflow-panel"><div><h4>{steps[step]}</h4><p className="muted">{step === 0 ? 'Connect an Arduino Uno or Nano over USB. DeviceOS will enumerate the serial port and identify the board when possible.' : 'This stage is ready for the next DeviceOS provisioning service.'}</p></div><button className="secondary" onClick={next}>{step === steps.length - 1 ? 'Complete' : 'Continue'}</button></div>
        </section>
      </main>
    </div>
  );
}

export default App;
