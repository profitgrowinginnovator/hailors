use serde::{Serialize, Deserialize};

/// Represents the features supported by a HEF file or a device.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SupportedFeatures {
    pub padded_ddr_buffers: bool,
    pub multi_network_support: bool,
    pub multi_context: bool,
    pub preliminary_run_asap: bool,
    pub hailo_net_flow: bool,
    pub dual_direction_stream_index: bool,
    pub nms_burst_mode: bool,
    pub output_scale_by_feature: bool,
    pub periph_calculation_in_hailort: bool,
    pub core_hw_padding_config_in_dfc: bool,
    pub batch_register_config: bool,
}

impl SupportedFeatures {
    /// Creates a new `SupportedFeatures` instance with all features set to their default (false).
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks if a specific feature is supported.
    pub fn is_supported(&self, feature_name: &str) -> Option<bool> {
        match feature_name {
            "padded_ddr_buffers" => Some(self.padded_ddr_buffers),
            "multi_network_support" => Some(self.multi_network_support),
            "multi_context" => Some(self.multi_context),
            "preliminary_run_asap" => Some(self.preliminary_run_asap),
            "hailo_net_flow" => Some(self.hailo_net_flow),
            "dual_direction_stream_index" => Some(self.dual_direction_stream_index),
            "nms_burst_mode" => Some(self.nms_burst_mode),
            "output_scale_by_feature" => Some(self.output_scale_by_feature),
            "periph_calculation_in_hailort" => Some(self.periph_calculation_in_hailort),
            "core_hw_padding_config_in_dfc" => Some(self.core_hw_padding_config_in_dfc),
            "batch_register_config" => Some(self.batch_register_config),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_supported_features() {
        let features = SupportedFeatures::new();
        assert!(!features.padded_ddr_buffers);
        assert!(!features.multi_network_support);
        assert!(!features.multi_context);
        assert!(!features.preliminary_run_asap);
        assert!(!features.hailo_net_flow);
        assert!(!features.dual_direction_stream_index);
        assert!(!features.nms_burst_mode);
        assert!(!features.output_scale_by_feature);
        assert!(!features.periph_calculation_in_hailort);
        assert!(!features.core_hw_padding_config_in_dfc);
        assert!(!features.batch_register_config);
    }

    #[test]
    fn test_feature_check() {
        let mut features = SupportedFeatures::new();
        features.multi_context = true;

        assert_eq!(features.is_supported("multi_context"), Some(true));
        assert_eq!(features.is_supported("hailo_net_flow"), Some(false));
        assert_eq!(features.is_supported("non_existent_feature"), None);
    }
}
