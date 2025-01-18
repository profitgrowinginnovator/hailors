// layer_info.rs

use super::{HailoFormat, NNStreamConfig, QuantInfo};


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LayerType {
    NotSet,
    Boundary,
    InterContext,
    Ddr,
    Cfg,
    Cache,
}

#[derive(Debug)]
pub struct BufferIndices {
    pub index: u32,
    pub cluster_index: u32,
}

#[derive(Debug)]
pub struct ConnectedContextInfo {
    pub context_index: u16,
    pub dma_engine_index: u8,
    pub stream_index: u8,
}

#[derive(Debug)]
pub struct DdrInfo {
    pub total_buffers_per_frame: u16,
    pub min_buffered_rows: u16,
}

#[derive(Debug)]
pub struct LayerInfo {
    pub layer_type: LayerType,
    pub direction: HailoStreamDirection, // Enum for hailo_stream_direction_t
    pub stream_index: u8,
    pub dma_engine_index: u8,
    pub name: String,
    pub network_name: String,
    pub network_index: u8,
    pub nn_stream_config: NNStreamConfig, // Struct for CONTROL_PROTOCOL__nn_stream_config_t
    pub max_shmifo_size: u32,
    pub context_index: u16,
    pub pad_index: Option<u32>, // Replaces INVALID_PAD_INDEX
    pub shape: Hailo3DImageShape, // Struct for hailo_3d_image_shape_t
    pub hw_shape: Hailo3DImageShape,
    pub hw_data_bytes: u32,
    pub format: HailoFormat, // Struct for hailo_format_t
    pub quant_info: QuantInfo, // Struct for hailo_quant_info_t
    pub quant_infos: Vec<QuantInfo>,
    pub nms_info: HailoNmsInfo, // Struct for hailo_nms_info_t
    pub is_mux: bool,
    pub predecessor: Vec<LayerInfo>,
    pub height_gcd: u32,
    pub height_ratios: Vec<u32>,
    pub is_multi_planar: bool,
    pub planes: Vec<LayerInfo>,
    pub plane_index: u8,
    pub is_defused_nms: bool,
    pub fused_nms_layer: Vec<LayerInfo>,
    pub buffer_indices: BufferIndices,
    pub connected_context_info: Option<ConnectedContextInfo>,
    pub ddr_info: DdrInfo,
    pub cache_id: u32,
}

impl LayerInfo {
    pub fn get_transfer_size(&self) -> Result<u32, String> {
        match self.layer_type {
            LayerType::Boundary | LayerType::InterContext | LayerType::Ddr => {
                Ok(self.nn_stream_config.periph_bytes_per_buffer * self.nn_stream_config.periph_buffers_per_frame)
            }
            _ => Err("Unsupported layer type for transfer size".to_string()),
        }
    }
}
