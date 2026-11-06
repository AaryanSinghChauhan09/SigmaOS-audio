//! ALSA-style device model.

#[derive(Debug, Clone)]
pub struct AlsaDevice {
    pub name: String,
    pub card: u32,
    pub device: u32,
}

#[derive(Debug, Clone)]
pub struct AlsaPcm {
    pub device: AlsaDevice,
    pub channels: u32,
    pub rate: u32,
    pub format: String,
}

#[derive(Debug, Clone)]
pub struct AlsaMixer {
    pub device: AlsaDevice,
    pub elements: Vec<MixerElement>,
}

#[derive(Debug, Clone)]
pub struct MixerElement {
    pub name: String,
    pub volume: u32,
    pub mute: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alsa_device() {
        let d = AlsaDevice { name: "hw:0".into(), card: 0, device: 0 };
        assert_eq!(d.card, 0);
    }
}
