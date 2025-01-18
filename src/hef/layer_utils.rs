use super::LayerType;
use super::StreamInfo;
use super::LayerInfo;


/// Utility functions for working with LayerInfo and StreamInfo.
pub struct LayerUtils;

impl LayerUtils {
    /// Converts a `LayerInfo` object to a list of `StreamInfo` objects.
    pub fn to_stream_infos(layer_info: &LayerInfo) -> Vec<StreamInfo> {
        let mut stream_infos = Vec::new();

        if layer_info.is_multi_planar {
            for plane in &layer_info.planes {
                stream_infos.push(Self::create_stream_info(plane));
            }
        } else {
            stream_infos.push(Self::create_stream_info(layer_info));
        }

        stream_infos
    }

    /// Checks if a `StreamInfo` object already exists in a vector of `StreamInfo` objects.
    pub fn stream_info_exists(vec: &[StreamInfo], name: &str) -> bool {
        vec.iter().any(|info| info.name == name)
    }

    /// Computes the transfer size for a given layer.
    pub fn calculate_transfer_size(layer_info: &LayerInfo) -> Option<u32> {
        match layer_info.layer_type {
            LayerType::Boundary | LayerType::Ddr | LayerType::InterContext => {
                Some(layer_info.nn_stream_config.periph_bytes_per_buffer * layer_info.nn_stream_config.periph_buffers_per_frame)
            }
            _ => None,
        }
    }

    fn create_stream_info(layer_info: &LayerInfo) -> StreamInfo {
        let mut stream_info = StreamInfo {
            name: layer_info.name.clone(),
            hw_data_bytes: layer_info.hw_data_bytes,
            format: layer_info.format.clone(),
            ..Default::default()
        };

        if let Some(nms_info) = &layer_info.nms_info {
            stream_info.nms_info = Some(nms_info.clone());
        }

        stream_info
    }
}
