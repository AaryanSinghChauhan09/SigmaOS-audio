//! SigmaOS-audio — audio subsystem model.
//!
//! Models ALSA, PipeWire, and PulseAudio-style audio graphs:
//! devices, streams, nodes, graphs, and audio processing.
//! Deterministic and std-only (no actual hardware access).

pub mod alsa;
pub mod pipewire;
pub mod pulse;
pub mod device;
pub mod stream;

pub use alsa::{AlsaDevice, AlsaPcm, AlsaMixer};
pub use pipewire::{AudioNode, AudioGraph, Link};
pub use pulse::{PulseSink, PulseSource, PulseServer};
pub use device::{AudioDevice, DeviceState};
pub use stream::{AudioStream, StreamParams};
