# SR71-Kurumi 🛫

Your personal cockpit companion.

A small system daemon, animation engine & compositor-side renderer that displays Kurumi floating above your workspace and reacting to:

- coding state
- time of day
- git status
- system activity

## Workspace Layout

sr71-kurumi/
├── sr71-core/ # domain logic, FSM, commands
├── kurumi-deamon/ # runtime daemon
├── sr71-ctl/ # CLI controls
└── docs/ # architecture, protocol, design docs



### Goals
- Non-intrusive animated UI layer
- State-driven visual behavior
- Extendable hook system
- Zero-latency compositor integration


# 🔥next sprint goals become:

### **Sprint A: daemon maturity**
- add Unix socket for IPC
- define message format
- log state transitions

### **Sprint B: CLI maturity**
- `sr71-ctl happy`
- `sr71-ctl coding`
- `sr71-ctl hide`

