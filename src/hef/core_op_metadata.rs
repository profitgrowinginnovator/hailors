use std::collections::HashMap;
use std::sync::Arc;

use super::context_metadata::ContextMetadata;
use super::layer_info::LayerInfo;
use super::supported_features::SupportedFeatures;
use super::stream_info::{StreamInfo, VStreamInfo};

#[derive(Debug, Clone)]
pub struct ConfigChannelInfo {
    pub engine_index: u8,
}

#[derive(Debug, Clone)]
pub struct CoreOpMetadata {
    pub core_op_name: String,
    pub preliminary_context: ContextMetadata,
    pub dynamic_contexts: Vec<ContextMetadata>,
    pub config_channels_info: Vec<ConfigChannelInfo>,
    pub supported_features: SupportedFeatures,
    pub sorted_network_names: Vec<String>,
    pub can_fast_batch_switch: bool,
}

impl CoreOpMetadata {
    pub fn new(
        core_op_name: String,
        preliminary_context: ContextMetadata,
        dynamic_contexts: Vec<ContextMetadata>,
        config_channels_info: Vec<ConfigChannelInfo>,
        supported_features: SupportedFeatures,
        sorted_network_names: Vec<String>,
        can_fast_batch_switch: bool,
    ) -> Self {
        Self {
            core_op_name,
            preliminary_context,
            dynamic_contexts,
            config_channels_info,
            supported_features,
            sorted_network_names,
            can_fast_batch_switch,
        }
    }

    pub fn get_input_layer_infos(&self) -> Vec<LayerInfo> {
        self.preliminary_context.get_boundary_input_layers()
    }

    pub fn get_output_layer_infos(&self) -> Vec<LayerInfo> {
        self.preliminary_context.get_boundary_output_layers()
    }

    pub fn get_all_layer_infos(&self) -> Vec<LayerInfo> {
        let mut all_layers = self.get_input_layer_infos();
        all_layers.extend(self.get_output_layer_infos());
        all_layers
    }

    pub fn get_stream_infos(&self) -> Vec<StreamInfo> {
        self.get_all_layer_infos()
            .iter()
            .flat_map(LayerInfo::to_stream_infos)
            .collect()
    }

    pub fn get_vstream_infos(&self) -> Vec<VStreamInfo> {
        self.get_all_layer_infos()
            .iter()
            .flat_map(LayerInfo::to_vstream_infos)
            .collect()
    }

    pub fn get_contexts_count(&self) -> usize {
        self.dynamic_contexts.len()
    }

    pub fn get_total_transfer_size(&self) -> usize {
        self.get_all_layer_infos()
            .iter()
            .filter_map(|layer| layer.get_transfer_size().ok())
            .sum()
    }
}
