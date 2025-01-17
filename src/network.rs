/// Represents metadata and data for an output layer.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LayerOutput {
    pub name: String,                 // Name of the layer
    pub data: Vec<u8>,                // Raw data for the layer
    pub data_type: DataType,          // Data type (e.g., UINT8, FLOAT32)
    pub shape: String,                // Shape of the output (e.g., FCR, NHWC)
    pub attributes: Option<String>,   // Optional layer-specific attributes
}

/// Represents metadata for the HEF file.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HefMetadata {
    pub network_name: String,          // Name of the network
    pub layers: Vec<LayerMetadata>,    // Metadata for each output layer
}

/// Represents metadata for an individual layer.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LayerMetadata {
    pub name: String,                 // Name of the layer
    pub data_type: DataType,          // Data type (e.g., UINT8, FLOAT32)
    pub shape: String,                // Shape of the layer output
    pub attributes: Option<String>,   // Optional layer-specific attributes
}

/// Supported data types for layers.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DataType {
    Uint8,
    Float32,
    Uint16,
}

/// A trait representing a neural network type.
///
/// Each implementation of this trait is responsible for parsing its own `LayerOutput` objects.
pub trait Network {
    type Output;

    /// Parses a single layer's `LayerOutput` into structured results.
    ///
    /// # Arguments
    /// * `layer` - The `LayerOutput` object containing raw data and metadata.
    ///
    /// # Returns
    /// A vector of parsed results of type `Self::Output`.
    fn parse_layer_output(&self, layer: &LayerOutput) -> Vec<Self::Output>;

    /// Constructs `LayerOutput` objects from HEF metadata.
    ///
    /// # Arguments
    /// * `hef_metadata` - The metadata extracted from the HEF file.
    ///
    /// # Returns
    /// A vector of `LayerOutput` objects.
    fn construct_layer_outputs(&self, hef_metadata: &HefMetadata) -> Vec<LayerOutput>;
}

/// Configuration for the YOLO Detection network.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct YoloDetection {
    pub num_classes: usize,           // Number of object classes
    pub max_bboxes_per_class: usize,  // Maximum bounding boxes per class
    pub threshold: f32,               // Confidence threshold
}

/// Represents a detection result.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Detection {
    pub class_id: u32,                // Class ID of the detected object
    pub confidence: f32,              // Confidence score of the detection
    pub bbox: (f32, f32, f32, f32),   // Bounding box coordinates (x_min, y_min, x_max, y_max)
}

/// Implementation of the `Network` trait for `YoloDetection`.
impl Network for YoloDetection {
    type Output = Detection;

    fn parse_layer_output(&self, layer: &LayerOutput) -> Vec<Self::Output> {
        let mut detections = Vec::new();
        let data = layer.data.as_slice();
        let output_data: Vec<f32> = match layer.data_type {
            DataType::Float32 => data
                .chunks_exact(4)
                .map(|chunk| f32::from_le_bytes(chunk.try_into().unwrap()))
                .collect(),
            _ => {
                eprintln!("Unsupported data type for detection parsing: {:?}", layer.data_type);
                return detections;
            }
        };

        let mut offset = 0;
        for class_id in 0..self.num_classes {
            if offset >= output_data.len() {
                break;
            }

            let bbox_count = output_data[offset] as usize;
            offset += 1;

            for _ in 0..bbox_count.min(self.max_bboxes_per_class) {
                if offset + 5 > output_data.len() {
                    break;
                }
                let x1 = output_data[offset];
                let y1 = output_data[offset + 1];
                let x2 = output_data[offset + 2];
                let y2 = output_data[offset + 3];
                let confidence = output_data[offset + 4];
                offset += 5;

                if confidence >= self.threshold {
                    detections.push(Detection {
                        class_id: class_id as u32,
                        confidence,
                        bbox: (x1, y1, x2, y2),
                    });
                }
            }
        }

        detections
    }

    fn construct_layer_outputs(&self, hef_metadata: &HefMetadata) -> Vec<LayerOutput> {
        hef_metadata
            .layers
            .iter()
            .map(|layer| LayerOutput {
                name: layer.name.clone(),
                data: vec![], // Data will be populated later
                data_type: layer.data_type.clone(),
                shape: layer.shape.clone(),
                attributes: layer.attributes.clone(),
            })
            .collect()
    }
}

/// Configuration for the YOLO Pose estimation network.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct YoloPose {
    pub num_keypoints: usize,         // Number of keypoints
    pub threshold: f32,               // Confidence threshold
    pub max_bboxes_per_class: usize,  // Maximum bounding boxes per class
}

/// Represents a pose estimation result.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Pose {
    pub keypoints: Vec<(f32, f32)>,   // List of (x, y) coordinates for keypoints
    pub confidence: f32,              // Confidence score
}

/// Represents a combined detection and pose result.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DetectionAndPose {
    pub detections: Vec<Detection>,   // List of detections
    pub poses: Vec<Pose>,             // List of poses
}

/// Implementation of the `Network` trait for `YoloPose`.
impl Network for YoloPose {
    type Output = DetectionAndPose;

    fn parse_layer_output(&self, layer: &LayerOutput) -> Vec<Self::Output> {
        let mut detections = Vec::new();
        let mut poses = Vec::new();

        let data = layer.data.as_slice();
        let output_data: Vec<f32> = match layer.data_type {
            DataType::Float32 => data
                .chunks_exact(4)
                .map(|chunk| f32::from_le_bytes(chunk.try_into().unwrap()))
                .collect(),
            _ => {
                eprintln!("Unsupported data type for pose parsing: {:?}", layer.data_type);
                return vec![];
            }
        };

        let mut offset = 0;

        // Parse bounding boxes
        let bbox_count = (output_data[offset] as usize).min(self.max_bboxes_per_class);
        offset += 1;

        for _ in 0..bbox_count {
            if offset + 5 > output_data.len() {
                break;
            }

            let x1 = output_data[offset];
            let y1 = output_data[offset + 1];
            let x2 = output_data[offset + 2];
            let y2 = output_data[offset + 3];
            let confidence = output_data[offset + 4];
            offset += 5;

            if confidence >= self.threshold {
                detections.push(Detection {
                    class_id: 0, // Default class ID
                    confidence,
                    bbox: (x1, y1, x2, y2),
                });
            }
        }

        // Parse keypoints
        while offset + self.num_keypoints * 3 <= output_data.len() {
            let mut keypoints = Vec::new();
            let mut valid_pose = false;

            for i in 0..self.num_keypoints {
                let x = output_data[offset + i * 3];
                let y = output_data[offset + i * 3 + 1];
                let confidence = output_data[offset + i * 3 + 2];

                if confidence >= self.threshold {
                    valid_pose = true;
                }

                keypoints.push((x, y));
            }

            offset += self.num_keypoints * 3;

            if valid_pose {
                poses.push(Pose {
                    keypoints,
                    confidence: self.threshold,
                });
            }
        }

        vec![DetectionAndPose {
            detections,
            poses,
        }]
    }

    fn construct_layer_outputs(&self, hef_metadata: &HefMetadata) -> Vec<LayerOutput> {
        hef_metadata
            .layers
            .iter()
            .map(|layer| LayerOutput {
                name: layer.name.clone(),
                data: vec![], // Data will be populated later
                data_type: layer.data_type.clone(),
                shape: layer.shape.clone(),
                attributes: layer.attributes.clone(),
            })
            .collect()
    }
}
