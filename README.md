# SigmaOS Audio

An evolving Rust audio library exposing `AudioManager` and modules for audio graphs, codec and driver abstractions, ALSA/PipeWire related types, podcast data, editing effects, and audio routing.

The module names and data models do not establish working access to sound hardware, a running PipeWire/ALSA service, real-time guarantees, or production codec support. Check each backend and its SigmaOS integration before relying on it.

## Build

The manifest references sibling [`SigmaOS-kernel`](../SigmaOS-kernel). From the SigmaOS checkout:

```sh
cargo check --manifest-path SigmaOS-audio/Cargo.toml --lib
```

## License

GPL-3.0-or-later. See [`Cargo.toml`](Cargo.toml).
