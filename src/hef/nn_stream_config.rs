use serde::{Serialize, Deserialize};

/// Represents a configuration for NN stream communication.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct NNStreamConfig {
    /// Number of peripheral bytes per buffer.
    pub periph_bytes_per_buffer: u32,
    /// Number of peripheral buffers per frame.
    pub periph_buffers_per_frame: u32,
}

/// Host buffer information for the control protocol.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct HostBufferInfo {
    /// Host buffer type.
    pub buffer_type: HostBufferType,
    /// Number of descriptors per buffer.
    pub descriptors_per_buffer: u32,
}

/// Represents different types of host buffers.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum HostBufferType {
    /// A buffer used for data transfer.
    Data,
    /// A buffer used for control communication.
    Control,
}

/// Represents a configuration for a channel.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ChannelConfig {
    /// Channel ID.
    pub channel_id: u8,
    /// Associated stream index.
    pub stream_index: u8,
    /// Stream configuration for this channel.
    pub nn_stream_config: Option<NNStreamConfig>,
    /// Host buffer information for the channel.
    pub host_buffer_info: Option<HostBufferInfo>,
    /// Indicates whether the channel has initial credits.
    pub initial_credits: Option<u32>,
}

impl ChannelConfig {
    /// Creates a new `ChannelConfig` with the specified parameters.
    pub fn new(
        channel_id: u8,
        stream_index: u8,
        nn_stream_config: Option<NNStreamConfig>,
        host_buffer_info: Option<HostBufferInfo>,
        initial_credits: Option<u32>,
    ) -> Self {
        Self {
            channel_id,
            stream_index,
            nn_stream_config,
            host_buffer_info,
            initial_credits,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_config_creation() {
        let nn_config = NNStreamConfig {
            periph_bytes_per_buffer: 512,
            periph_buffers_per_frame: 4,
        };

        let host_info = HostBufferInfo {
            buffer_type: HostBufferType::Data,
            descriptors_per_buffer: 256,
        };

        let config = ChannelConfig::new(1, 2, Some(nn_config.clone()), Some(host_info.clone()), Some(1024));

        assert_eq!(config.channel_id, 1);
        assert_eq!(config.stream_index, 2);
        assert_eq!(config.nn_stream_config, Some(nn_config));
        assert_eq!(config.host_buffer_info, Some(host_info));
        assert_eq!(config.initial_credits, Some(1024));
    }
}
