//! Audio stream model.

#[derive(Debug, Clone)]
pub struct StreamParams {
    pub channels: u32,
    pub rate: u32,
    pub format: String,
    pub buffer_size: u32,
}

#[derive(Debug, Clone)]
pub struct AudioStream {
    pub device: String,
    pub params: StreamParams,
    pub active: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_params() {
        let p = StreamParams { channels: 2, rate: 48000, format: "S16LE".into(), buffer_size: 1024 };
        assert_eq!(p.rate, 48000);
    }
}
