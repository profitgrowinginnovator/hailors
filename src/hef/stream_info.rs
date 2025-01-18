use super::HailoFormat;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamInfo {
    pub name: String,
    pub direction: StreamDirection,
    pub hw_frame_size: u32,
    pub hw_shape: Shape,
    pub shape: Shape,
    pub format: HailoFormat,
    pub quant_info: QuantInfo,
    pub is_mux: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VStreamInfo {
    pub name: String,
    pub network_name: String,
    pub direction: StreamDirection,
    pub shape: Shape,
    pub format: HailoFormat,
    pub quant_info: QuantInfo,
    pub nms_shape: Option<NmsShape>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamDirection {
    HostToDevice,
    DeviceToHost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shape {
    pub height: u32,
    pub width: u32,
    pub features: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NmsShape {
    pub max_bboxes_per_class: u32,
    pub number_of_classes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantInfo {
    pub scale: f32,
    pub zero_point: i32,
}

impl StreamInfo {
    pub fn new(
        name: String,
        direction: StreamDirection,
        hw_frame_size: u32,
        hw_shape: Shape,
        shape: Shape,
        format: HailoFormat,
        quant_info: QuantInfo,
        is_mux: bool,
    ) -> Self {
        Self {
            name,
            direction,
            hw_frame_size,
            hw_shape,
            shape,
            format,
            quant_info,
            is_mux,
        }
    }

    pub fn describe(&self) -> String {
        format!(
            "StreamInfo: {{\n  Name: {},\n  Direction: {:?},\n  HW Frame Size: {},\n  HW Shape: {:?},\n  Shape: {:?},\n  Format: {:?},\n  Quant Info: {:?},\n  Is Mux: {}\n}}",
            self.name, self.direction, self.hw_frame_size, self.hw_shape, self.shape, self.format, self.quant_info, self.is_mux
        )
    }
}

impl VStreamInfo {
    pub fn new(
        name: String,
        network_name: String,
        direction: StreamDirection,
        shape: Shape,
        format: HailoFormat,
        quant_info: QuantInfo,
        nms_shape: Option<NmsShape>,
    ) -> Self {
        Self {
            name,
            network_name,
            direction,
            shape,
            format,
            quant_info,
            nms_shape,
        }
    }

    pub fn describe(&self) -> String {
        format!(
            "VStreamInfo: {{\n  Name: {},\n  Network Name: {},\n  Direction: {:?},\n  Shape: {:?},\n  Format: {:?},\n  Quant Info: {:?},\n  NMS Shape: {:?}\n}}",
            self.name, self.network_name, self.direction, self.shape, self.format, self.quant_info, self.nms_shape
        )
    }
}

impl Shape {
    pub fn new(height: u32, width: u32, features: u32) -> Self {
        Self {
            height,
            width,
            features,
        }
    }
}

impl QuantInfo {
    pub fn new(scale: f32, zero_point: i32) -> Self {
        Self {
            scale,
            zero_point,
        }
    }
}

impl NmsShape {
    pub fn new(max_bboxes_per_class: u32, number_of_classes: u32) -> Self {
        Self {
            max_bboxes_per_class,
            number_of_classes,
        }
    }
}
