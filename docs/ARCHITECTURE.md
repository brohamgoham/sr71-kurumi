# sr71-kurumi Architecture

The system is composed of 3 components:

## sr71-core (Rust)
- Defines domain model
- Emote types
- Animation state machine
- IPC command types

## kurumi-daemon (Rust)
- Runs event loop
- Produces commands based on external events
- Sends commands to renderer/plugin

## sr71-ctl (Rust)
- CLI tool for setting states manually

## Rendering Layer
TBD - Hyprland plugin (C++), Android later

