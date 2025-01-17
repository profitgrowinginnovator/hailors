/// A trait representing a neural network type.
///
/// This trait defines the expected output type for a network and a method to parse raw output data
/// into the associated output type.
pub trait Network {
    /// The output type of the network (e.g., `Detection` for YOLO, `Pose` for pose estimation).
    type Output;

    /// Parses `f32` data into a structured output type.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of `f32` values representing the raw output data.
    ///
    /// # Returns
    ///
    /// An optional vector of parsed outputs of type `Self::Output`. Returns `None` if not implemented.
    fn parse_output_f32(&self, _data: &[f32]) -> Option<Vec<Self::Output>> {
        None
    }

    /// Parses `u16` data into a structured output type.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of `u16` values representing the raw output data.
    ///
    /// # Returns
    ///
    /// An optional vector of parsed outputs of type `Self::Output`. Returns `None` if not implemented.
    fn parse_output_u16(&self, _data: &[u16]) -> Option<Vec<Self::Output>> {
        None
    }

    /// Parses `u8` data into a structured output type.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of `u8` values representing the raw output data.
    ///
    /// # Returns
    ///
    /// An optional vector of parsed outputs of type `Self::Output`. Returns `None` if not implemented.
    fn parse_output_u8(&self, _data: &[u8]) -> Option<Vec<Self::Output>> {
        None
    }

    /// Main entry point to parse data of any type, with automatic conversion if necessary.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of raw `u8` values representing the output data.
    /// * `input_format` - The format of the input data (e.g., `UINT8`, `FLOAT32`, `UINT16`).
    /// * `desired_format` - The desired format for parsing (e.g., `FLOAT32`, `UINT16`, `UINT8`).
    ///
    /// # Returns
    ///
    /// A vector of parsed outputs of type `Self::Output`.
    ///
    /// # Panics
    ///
    /// Panics if the desired format is unsupported or if the corresponding parsing method is not implemented.
    fn parse_output(
        &self,
        data: &[u8],
        input_format: DataType,
        desired_format: DataType,
    ) -> Result<ConvertedData, &'static str>  {
    

        match (input_format.clone(), desired_format) {
            (DataType::Float32, DataType::Uint16) => {
                let f32_data = parse_f32(data)?;
                let u16_data = f32_data.into_iter().map(|v| v as u16).collect();
                Ok(ConvertedData::Uint16(u16_data))
            }
            (DataType::Float32, DataType::Uint8) => {
                let f32_data = parse_f32(data)?;
                let u8_data = f32_data.into_iter().map(|v| v as u8).collect();
                Ok(ConvertedData::Uint8(u8_data))
            }
            (DataType::Uint16, DataType::Float32) => {
                let u16_data = parse_u16(data)?;
                let f32_data = u16_data.into_iter().map(|v| v as f32).collect();
                Ok(ConvertedData::Float32(f32_data))
            }
            (DataType::Uint16, DataType::Uint8) => {
                let u16_data = parse_u16(data)?;
                let u8_data = u16_data.into_iter().map(|v| v as u8).collect();
                Ok(ConvertedData::Uint8(u8_data))
            }
            (DataType::Uint8, DataType::Float32) => {
                let f32_data = data.iter().map(|&v| v as f32).collect();
                Ok(ConvertedData::Float32(f32_data))
            }
            (DataType::Uint8, DataType::Uint16) => {
                let u16_data = data.iter().map(|&v| v as u16).collect();
                Ok(ConvertedData::Uint16(u16_data))
            }
            _ => Err("Unsupported data type conversion"),
        }
    }


    /// Converts data between supported formats.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of raw `u8` values representing the output data.
    /// * `input_format` - The input data type.
    /// * `desired_format` - The desired output data type.
    ///
    /// # Returns
    ///
    /// A `Result` containing the converted data or an error if the conversion is unsupported.
    fn convert_data(data: &[u8], input_format: DataType, desired_format: DataType) -> Result<ConvertedData, &'static str> {
        match (input_format, desired_format) {
            (DataType::Float32, DataType::Uint16) => {
                let f32_data = parse_f32(data)?;
                let u16_data = f32_data.into_iter().map(|v| v as u16).collect();
                Ok(ConvertedData::Uint16(u16_data))
            }
            (DataType::Float32, DataType::Uint8) => {
                let f32_data = parse_f32(data)?;
                let u8_data = f32_data.into_iter().map(|v| v as u8).collect();
                Ok(ConvertedData::Uint8(u8_data))
            }
            (DataType::Uint16, DataType::Float32) => {
                let u16_data = parse_u16(data)?;
                let f32_data = u16_data.into_iter().map(|v| v as f32).collect();
                Ok(ConvertedData::Float32(f32_data))
            }
            (DataType::Uint16, DataType::Uint8) => {
                let u16_data = parse_u16(data)?;
                let u8_data = u16_data.into_iter().map(|v| v as u8).collect();
                Ok(ConvertedData::Uint8(u8_data))
            }
            (DataType::Uint8, DataType::Float32) => {
                let f32_data = data.iter().map(|&v| v as f32).collect();
                Ok(ConvertedData::Float32(f32_data))
            }
            (DataType::Uint8, DataType::Uint16) => {
                let u16_data = data.iter().map(|&v| v as u16).collect();
                Ok(ConvertedData::Uint16(u16_data))
            }
            _ => Err("Unsupported data type conversion"),
        }
    }

}

/// Represents metadata and data for an output layer.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct LayerOutput {
    pub name: String,                 // Name of the layer
    pub data: Vec<u8>,                // Raw data for the layer
    pub data_type: DataType,          // Data type (UINT8, FLOAT32, etc.)
    pub shape: String,                // Shape of the output (e.g., FCR, NHWC)
    pub attributes: Option<String>,   // Optional layer-specific attributes
}

/// the supported data types for different network layers
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum DataType {
    Uint8,
    Float32,
    Uint16,
    // Add more types as needed
}

/// Enum representing supported network types for the CLI.
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum NetworkType {
    /// YOLO Detection network.
    YoloDetection,
    /// YOLO Pose estimation network.
    YoloPose,
}

/// Configuration for the YOLO Detection network.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct YoloDetection {
    /// Number of object classes (e.g., COCO dataset has 80 classes).
    pub num_classes: usize,
    /// Maximum number of bounding boxes per class.
    pub max_bboxes_per_class: usize,
    /// Confidence threshold for detections.
    pub threshold: f32,
}

/// Represents a detection result for the YOLO Detection network.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Detection {
    /// Class ID of the detected object.
    pub class_id: u32,
    /// Confidence score of the detection.
    pub confidence: f32,
    /// Bounding box coordinates: (x_min, y_min, x_max, y_max).
    pub bbox: (f32, f32, f32, f32),
}

/// Implementation of the `Network` trait for `YoloDetection`.
///
/// This parses the raw output data into a list of `Detection` objects.
impl Network for YoloDetection {
    type Output = Detection;

    fn parse_output_f32(&self, output_data: &[f32]) -> Option<Vec<Self::Output>> {
        let mut detections = Vec::new();
        let mut offset = 0;
        

        // Iterate through each class to parse its detections.
        for class_id in 0..self.num_classes {
            // Ensure there is sufficient data for the bbox_count.
            if offset >= output_data.len() {
                break;
            }
            let bbox_count = output_data[offset] as usize; // Number of bounding boxes for this class.
            offset += 1;

            // Validate and truncate bbox_count if it exceeds the maximum allowed.
            if bbox_count > self.max_bboxes_per_class {
                eprintln!(
                    "Warning: Class {} has bbox_count {} exceeding max_bboxes_per_class {}. Truncating.",
                    class_id, bbox_count, self.max_bboxes_per_class
                );
            }
            let valid_bbox_count = bbox_count.min(self.max_bboxes_per_class);

            // Parse each bounding box for the current class.
            for _ in 0..valid_bbox_count {
                // Ensure there is sufficient data for a complete bounding box.
                if offset + 5 > output_data.len() {
                    eprintln!(
                        "Warning: Truncated data for class {}. Expected complete bbox, but data is insufficient.",
                        class_id
                    );
                    break;
                }

                let x1 = output_data[offset];
                let y1 = output_data[offset + 1];
                let x2 = output_data[offset + 2];
                let y2 = output_data[offset + 3];
                let confidence = output_data[offset + 4];

                // Add detection if confidence is above the threshold.
                if confidence >= self.threshold {
                    detections.push(Detection {
                        class_id: class_id as u32,
                        confidence,
                        bbox: (x1, y1, x2, y2),
                    });
                }

                offset += 5; // Each bounding box consumes 5 values.
            }
        }

        Some(detections)
    }
}

/// Configuration for the YOLO Pose estimation network
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct YoloPose {
    /// Number of keypoints to detect (e.g., 17 for human pose estimation).
    pub num_keypoints: usize,
    /// Confidence threshold for pose detections.
    pub threshold: f32,
    /// how many bboxes are there per class
    pub max_bboxes_per_class: usize,
}

/// Represents a pose estimation result for the YOLO Pose network.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Pose {
    /// List of (x, y) coordinates for detected keypoints.
    pub keypoints: Vec<(f32, f32)>,
    /// Confidence score of the pose estimation.
    pub confidence: f32,
}
/// Represents both the dections and poses for the Yolo Pose network.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DetectionAndPose {
    pub detections: Vec<Detection>,
    pub poses: Vec<Pose>,
}

/// Implementation of the `Network` trait for `YoloPose`.
///
/// This parses the raw output data into a list of Detection` and `Pose` objects.
impl Network for YoloPose {
    type Output = DetectionAndPose;

    fn parse_output_f32(
                &self, 
                output_data: &[f32]) -> Option<Vec<Self::Output>> {
        let mut detections = Vec::new();
        let mut poses = Vec::new();
        let mut offset = 0;

        // Step 1: Parse Bounding Boxes
        let bbox_count = (output_data[offset] as usize).min(self.max_bboxes_per_class);
        offset += 1; // First value indicates the number of bounding boxes

        for _ in 0..bbox_count {
            if offset + 5 > output_data.len() {
                eprintln!("Error: Insufficient data for bounding box parsing");
                break;
            }

            let x1 = output_data[offset];
            let y1 = output_data[offset + 1];
            let x2 = output_data[offset + 2];
            let y2 = output_data[offset + 3];
            let confidence = output_data[offset + 4];
            offset += 5; // Each bounding box takes 5 values

            // Add detection if confidence is above the threshold
            if confidence >= self.threshold {
                detections.push(Detection {
                    class_id: 0, // Replace with appropriate class ID if available
                    confidence,
                    bbox: (x1, y1, x2, y2),
                });
            }
        }

        // Step 2: Parse Poses (Keypoints)
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

                keypoints.push((x, y, confidence));
            }

            offset += self.num_keypoints * 3;

            if valid_pose {
                let pose_confidence = keypoints
                    .iter()
                    .filter(|(_, _, conf)| *conf >= self.threshold)
                    .map(|(_, _, conf)| *conf)
                    .sum::<f32>()
                    / self.num_keypoints as f32;

                poses.push(Pose {
                    keypoints: keypoints.iter().map(|(x, y, _)| (*x, *y)).collect(),
                    confidence: pose_confidence,
                });
            }
        }

        // Step 3: Return Combined Output
        Some(vec![DetectionAndPose {
            detections,
            poses,
        }])
    }
}



pub fn non_max_suppression(
    detections: &mut Vec<Detection>,
    iou_threshold: f32,
) -> Vec<Detection> {
    detections.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

    let mut selected_detections = Vec::new();
    let mut suppressed = vec![false; detections.len()];

    for i in 0..detections.len() {
        if suppressed[i] {
            continue;
        }
        selected_detections.push(detections[i].clone());

        for j in (i + 1)..detections.len() {
            if suppressed[j] {
                continue;
            }

            let iou = calculate_iou(detections[i].bbox, detections[j].bbox);
            if iou > iou_threshold {
                suppressed[j] = true;
            }
        }
    }

    selected_detections
}

pub fn calculate_iou(bbox1: (f32, f32, f32, f32), bbox2: (f32, f32, f32, f32)) -> f32 {
    let (x1, y1, x2, y2) = bbox1;
    let (x1p, y1p, x2p, y2p) = bbox2;

    let inter_x1 = x1.max(x1p);
    let inter_y1 = y1.max(y1p);
    let inter_x2 = x2.min(x2p);
    let inter_y2 = y2.min(y2p);

    let inter_area = ((inter_x2 - inter_x1).max(0.0)) * ((inter_y2 - inter_y1).max(0.0));
    let bbox1_area = (x2 - x1) * (y2 - y1);
    let bbox2_area = (x2p - x1p) * (y2p - y1p);

    inter_area / (bbox1_area + bbox2_area - inter_area)
}


/// Represents a generic converted data container.
pub enum ConvertedData {
    Float32(Vec<f32>),
    Uint16(Vec<u16>),
    Uint8(Vec<u8>),
}

impl ConvertedData {
    /// Access the data as `f32`.
    fn into_f32(self) -> Vec<f32> {
        match self {
            ConvertedData::Float32(data) => data,
            _ => panic!("Incorrect conversion to f32"),
        }
    }

    /// Access the data as `u16`.
    fn into_u16(self) -> Vec<u16> {
        match self {
            ConvertedData::Uint16(data) => data,
            _ => panic!("Incorrect conversion to u16"),
        }
    }

    /// Access the data as `u8`.
    fn into_u8(self) -> Vec<u8> {
        match self {
            ConvertedData::Uint8(data) => data,
            _ => panic!("Incorrect conversion to u8"),
        }
    }
}

/// Parses `u8` data into `f32`.
pub fn parse_f32(data: &[u8]) -> Result<Vec<f32>, &'static str> {
    if data.len() % 4 != 0 {
        return Err("Data length is not a multiple of 4 for FLOAT32 conversion");
    }
    Ok(data
        .chunks_exact(4)
        .map(|chunk| f32::from_le_bytes(chunk.try_into().unwrap()))
        .collect())
}

/// Parses `u8` data into `u16`.
pub fn parse_u16(data: &[u8]) -> Result<Vec<u16>, &'static str> {
    if data.len() % 2 != 0 {
        return Err("Data length is not a multiple of 2 for UINT16 conversion");
    }
    Ok(data
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes(chunk.try_into().unwrap()))
        .collect())
}
