use std::collections::HashMap;
use std::sync::Arc;

use super::layer_info::LayerInfo;
use super::context_switch_actions::ContextSwitchConfigAction;

/// Represents metadata for a context within a CoreOp.
#[derive(Debug)]
pub struct ContextMetadata {
    actions: Vec<Arc<ContextSwitchConfigAction>>,
    config_buffers_info: HashMap<u8, ConfigBufferInfo>,
    const_input_layer_found: bool,

    boundary_input_layers: Vec<LayerInfo>,
    boundary_output_layers: Vec<LayerInfo>,
    inter_context_input_layers: Vec<LayerInfo>,
    inter_context_output_layers: Vec<LayerInfo>,
    ddr_input_layers: Vec<LayerInfo>,
    ddr_output_layers: Vec<LayerInfo>,
    cache_input_layers: Vec<LayerInfo>,
    cache_output_layers: Vec<LayerInfo>,
}

/// Information about the configuration buffer for a given context.
#[derive(Debug)]
pub struct ConfigBufferInfo {
    /// Sizes of all successive CCW bursts.
    pub bursts_sizes: Vec<u32>,
    /// Offset from the beginning of the HEF user address.
    pub offset_from_hef_base: u64,
}

impl ContextMetadata {
    pub fn new(
        actions: Vec<Arc<ContextSwitchConfigAction>>,
        config_buffers_info: HashMap<u8, ConfigBufferInfo>,
        const_input_layer_found: bool,
    ) -> Self {
        Self {
            actions,
            config_buffers_info,
            const_input_layer_found,
            boundary_input_layers: Vec::new(),
            boundary_output_layers: Vec::new(),
            inter_context_input_layers: Vec::new(),
            inter_context_output_layers: Vec::new(),
            ddr_input_layers: Vec::new(),
            ddr_output_layers: Vec::new(),
            cache_input_layers: Vec::new(),
            cache_output_layers: Vec::new(),
        }
    }

    pub fn get_actions(&self) -> &Vec<Arc<ContextSwitchConfigAction>> {
        &self.actions
    }

    pub fn get_actions_of_type(&self, action_types: &[ContextSwitchConfigAction]) -> Vec<Arc<ContextSwitchConfigAction>> {
        self.actions
            .iter()
            .filter(|action| action_types.contains(&**action))
            .cloned()
            .collect()
    }

    pub fn get_config_buffers_info(&self) -> &HashMap<u8, ConfigBufferInfo> {
        &self.config_buffers_info
    }

    pub fn add_boundary_layer(&mut self, layer_info: LayerInfo) {
        match layer_info.direction {
            Direction::Input => self.boundary_input_layers.push(layer_info),
            Direction::Output => self.boundary_output_layers.push(layer_info),
        }
    }

    pub fn add_inter_context_layer(&mut self, layer_info: LayerInfo) {
        match layer_info.direction {
            Direction::Input => self.inter_context_input_layers.push(layer_info),
            Direction::Output => self.inter_context_output_layers.push(layer_info),
        }
    }

    pub fn add_ddr_layer(&mut self, layer_info: LayerInfo) {
        match layer_info.direction {
            Direction::Input => self.ddr_input_layers.push(layer_info),
            Direction::Output => self.ddr_output_layers.push(layer_info),
        }
    }

    pub fn add_cache_layer(&mut self, layer_info: LayerInfo) {
        match layer_info.direction {
            Direction::Input => self.cache_input_layers.push(layer_info),
            Direction::Output => self.cache_output_layers.push(layer_info),
        }
    }

    pub fn get_boundary_input_layers(&self) -> &Vec<LayerInfo> {
        &self.boundary_input_layers
    }

    pub fn get_boundary_output_layers(&self) -> &Vec<LayerInfo> {
        &self.boundary_output_layers
    }

    pub fn get_inter_context_input_layers(&self) -> &Vec<LayerInfo> {
        &self.inter_context_input_layers
    }

    pub fn get_inter_context_output_layers(&self) -> &Vec<LayerInfo> {
        &self.inter_context_output_layers
    }

    pub fn get_ddr_input_layers(&self) -> &Vec<LayerInfo> {
        &self.ddr_input_layers
    }

    pub fn get_ddr_output_layers(&self) -> &Vec<LayerInfo> {
        &self.ddr_output_layers
    }

    pub fn get_cache_input_layers(&self) -> &Vec<LayerInfo> {
        &self.cache_input_layers
    }

    pub fn get_cache_output_layers(&self) -> &Vec<LayerInfo> {
        &self.cache_output_layers
    }

    pub fn get_context_transfer_size(&self) -> usize {
        self.boundary_input_layers
            .iter()
            .chain(self.inter_context_input_layers.iter())
            .chain(self.ddr_input_layers.iter())
            .chain(self.cache_input_layers.iter())
            .map(|layer| layer.get_transfer_size().unwrap_or(0))
            .sum()
    }

    pub fn const_input_layer_found(&self) -> bool {
        self.const_input_layer_found
    }
}
