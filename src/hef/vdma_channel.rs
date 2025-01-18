use serde::{Serialize, Deserialize};

/// Represents a VDMA channel's identifier and properties.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct VdmaChannel {
    /// The index of the channel.
    pub channel_id: u8,
    /// The stream index associated with the channel.
    pub stream_index: u8,
    /// Indicates if this channel is a dummy stream.
    pub is_dummy_stream: bool,
}

impl VdmaChannel {
    /// Creates a new `VdmaChannel` with the specified parameters.
    pub fn new(channel_id: u8, stream_index: u8, is_dummy_stream: bool) -> Self {
        Self {
            channel_id,
            stream_index,
            is_dummy_stream,
        }
    }

    /// Checks if the channel is associated with a dummy stream.
    pub fn is_dummy(&self) -> bool {
        self.is_dummy_stream
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vdma_channel() {
        let channel = VdmaChannel::new(1, 2, false);
        assert_eq!(channel.channel_id, 1);
        assert_eq!(channel.stream_index, 2);
        assert!(!channel.is_dummy());
    }

    #[test]
    fn test_dummy_channel() {
        let channel = VdmaChannel::new(3, 4, true);
        assert!(channel.is_dummy());
    }
}
