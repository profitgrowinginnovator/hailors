// ProtoHef.rs

use std::collections::HashMap;
use std::sync::Arc;

// Placeholder for supported features in the ProtoHefHeader.
#[derive(Debug, Clone)]
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

impl Default for SupportedFeatures {
    fn default() -> Self {
        SupportedFeatures {
            padded_ddr_buffers: false,
            multi_network_support: false,
            multi_context: false,
            preliminary_run_asap: false,
            hailo_net_flow: false,
            dual_direction_stream_index: false,
            nms_burst_mode: false,
            output_scale_by_feature: false,
            periph_calculation_in_hailort: false,
            core_hw_padding_config_in_dfc: false,
            batch_register_config: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProtoHefHeader {
    pub version: u32,
    pub proto_size: u32,
    pub checksum: u64, // Placeholder for the actual checksum type (e.g., MD5, CRC, etc.)
    pub supported_features: SupportedFeatures,
}

#[derive(Debug, Clone)]
pub struct ProtoHefExtension {
    pub name: String,
    pub version: u32,
}

#[derive(Debug, Clone)]
pub struct ProtoHefOptionalExtension {
    pub name: String,
    pub is_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct ProtoHefNetworkGroup {
    pub name: String,
    pub core_ops: HashMap<String, Arc<CoreOpMetadata>>,
    pub extensions: Vec<ProtoHefExtension>,
    pub optional_extensions: Vec<ProtoHefOptionalExtension>,
}

#[derive(Debug, Clone)]
pub struct ProtoHef {
    pub header: ProtoHefHeader,
    pub network_groups: Vec<ProtoHefNetworkGroup>,
}

impl ProtoHef {
    pub fn new(header: ProtoHefHeader, network_groups: Vec<ProtoHefNetworkGroup>) -> Self {
        ProtoHef {
            header,
            network_groups,
        }
    }

    pub fn get_network_group(&self, name: &str) -> Option<&ProtoHefNetworkGroup> {
        self.network_groups.iter().find(|ng| ng.name == name)
    }

    pub fn list_network_groups(&self) -> Vec<String> {
        self.network_groups.iter().map(|ng| ng.name.clone()).collect()
    }
}

// Placeholder for the CoreOpMetadata struct
#[derive(Debug, Clone)]
pub struct CoreOpMetadata {
    pub name: String,
    // Add additional fields related to CoreOpMetadata as required.
}

impl CoreOpMetadata {
    pub fn new(name: &str) -> Self {
        CoreOpMetadata {
            name: name.to_string(),
        }
    }
}
