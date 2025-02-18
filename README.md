# BulletForceHaxV3

BulletForceHaxv3 (BFHv3) is the third iteration of my attempt at writing tooling and cheats for a Unity WebAssembly game.

## Comparison to previous iterations

- [BulletForceHax](https://github.com/holly-hacker/bulletforcehax): "An in-browser cheating/sniffing tool for Bullet Force"
  - Originally written in Dart (compiled to JS), later partially rewritten in Rust (compiled to WASM/native)
  - Web app that used a modified Unity WebGL template and would hook WebSocket functions to call into some synchronous JavaScript or WASM code.
  - Contained an attempt at a UI in Angular, but was later abandoned
  - Contained a working headless client that was able of connecting to matches and was capable of sending arbitrary packets, but that code was never committed
- [BulletForceHaxv2](https://github.com/holly-hacker/bulletforcehaxv2): "Bullet Force launcher and MITM-based hax"
  - Written in Rust from the start
  - Worked by exposing HTTP and WebSocket endpoints that would proxy all game network traffic
  - Contained a native app that would host a WebView with the game and would automatically download the required game files, resulting in a fully self-contained experience
  - Contained a fairly solid networking library that could in theory be used by other projects
  - Used egui to provide a configuration UI
- BulletForceHaxv3: You are here
  - Written in Typescript and Rust
  - Packaged as a MV3 browser extension that can be loaded by any(?) modern browser
  - Forks BulletForceHaxV2's networking library, further improving it
  - Big plans!

## Future goals

These are some high-level goals that were conceived of near the start of development, take them with a grain of salt.

- [ ] Build out a high quality nwetworking library that could be published on its own
- [ ] Create a decent packet sniffer, likely as a chrome DevTools page
- [ ] Create a headless bot, possibly scriptable
- [ ] Create some basic cheats that could be controller from within the browser
