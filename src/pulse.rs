//! PulseAudio-style server model.

#[derive(Debug, Clone)]
pub struct PulseServer {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct PulseSink {
    pub name: String,
    pub index: u32,
    pub volume: u32,
}

#[derive(Debug, Clone)]
pub struct PulseSource {
    pub name: String,
    pub index: u32,
    pub volume: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pulse_sink() {
        let s = PulseSink { name: "default".into(), index: 0, volume: 65536 };
        assert_eq!(s.index, 0);
    }
}
