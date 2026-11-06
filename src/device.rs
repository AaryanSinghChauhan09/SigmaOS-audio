//! Generic audio device model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceState { Active, Idle, Suspended }

#[derive(Debug, Clone)]
pub struct AudioDevice {
    pub name: String,
    pub state: DeviceState,
    pub channels: u32,
    pub rate: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn device_state() {
        let mut d = AudioDevice { name: "default".into(), state: DeviceState::Idle, channels: 2, rate: 48000 };
        d.state = DeviceState::Active;
        assert_eq!(d.state, DeviceState::Active);
    }
}
