use std::collections::HashMap;

/// Represents information about configuration buffers.
#[derive(Debug, Clone)]
pub struct ConfigBufferInfo {
    /// Sizes of all successive CCW bursts.
    pub bursts_sizes: Vec<u32>,
    /// Offset from the beginning of the HEF user address in case of a continuous pre-allocated buffer.
    pub offset_from_hef_base: u64,
}

impl ConfigBufferInfo {
    /// Creates a new instance of `ConfigBufferInfo`.
    pub fn new(bursts_sizes: Vec<u32>, offset_from_hef_base: u64) -> Self {
        ConfigBufferInfo {
            bursts_sizes,
            offset_from_hef_base,
        }
    }
}

/// A map of configuration buffer information, keyed by configuration stream index.
pub type ConfigBufferInfoMap = HashMap<u8, ConfigBufferInfo>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_buffer_info_creation() {
        let bursts_sizes = vec![64, 128, 256];
        let offset = 1024;

        let config_buffer_info = ConfigBufferInfo::new(bursts_sizes.clone(), offset);

        assert_eq!(config_buffer_info.bursts_sizes, bursts_sizes);
        assert_eq!(config_buffer_info.offset_from_hef_base, offset);
    }

    #[test]
    fn test_config_buffer_info_map() {
        let mut config_map: ConfigBufferInfoMap = HashMap::new();

        config_map.insert(0, ConfigBufferInfo::new(vec![64, 128], 1024));
        config_map.insert(1, ConfigBufferInfo::new(vec![256, 512], 2048));

        assert_eq!(config_map.len(), 2);
        assert!(config_map.contains_key(&0));
        assert!(config_map.contains_key(&1));
    }
}
