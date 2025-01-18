pub mod layer_info;
pub mod context_switch_actions;
pub mod context_metadata;
pub mod core_op_metadata;
pub mod config_buffer;
pub mod proto_hef;
pub mod hailo_format;
pub mod stream_info;
pub mod supported_features;
pub mod layer_utils;
pub mod vdma_channel;
pub mod nn_stream_config;
pub mod seekable_bytes_reader;
pub mod hailo_common;
pub mod hef;

// Re-exporting for easier access
pub use layer_info::{LayerInfo, LayerType, BufferIndices, ConnectedContextInfo, DdrInfo};
pub use context_switch_actions::{
    ContextSwitchConfigAction, ContextSwitchConfigActionPtr, ActionType, NoneAction,
    ActivateConfigChannelAction, DeactivateConfigChannelAction,
};
pub use core_op_metadata::{CoreOpMetadata, CoreOpMetadataPtr, NetworkGroupMetadata, SupportedFeatures, ConfigChannelInfo};
pub use config_buffer::{ConfigBuffer, ConfigBufferInfo, ConfigBufferInfoMap};
pub use proto_hef::{ProtoHEFHeader, ProtoHEFExtension, ProtoHEFOptionalExtension, ProtoHEFCoreOpMock, ProtoHEFNetworkGroup, ProtoHEFContext};
pub use hailo_format::{HailoFormat, FormatOrder, QuantInfo, NmsInfo};
pub use stream_info::{StreamInfo, StreamDirection, VStreamInfo, QuantInfo};
pub use supported_features::SupportedFeatures;
pub use layer_utils::LayerUtils;
pub use vdma_channel::{VdmaChannel, VdmaChannelId};
pub use nn_stream_config::{ControlProtocol, HostBufferInfo, StreamConfig};
pub use seekable_bytes_reader::SeekableBytesReader;
pub use nn_stream_config::{NNStreamConfig, StreamConfigParameters};
pub use hailo_common::{HailoError, HailoResult, alignment};
pub use hef::HefData;