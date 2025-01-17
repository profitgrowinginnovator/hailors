/// A trait representing a neural network type.
///
/// This trait defines the expected output type for a network and a method to parse raw output data
/// into the associated output type.
pub trait Network {
    /// The output type of the network (e.g., `Detection` for YOLO, `Pose` for pose estimation).
    type Output;

    /// Parses the raw output data into a structured output type.
    ///
    /// # Arguments
    ///
    /// * `output_data` - A slice of `f32` values representing raw output data from the network.
    ///
    /// # Returns
    ///
    /// A vector of parsed outputs of type `Self::Output`.
    fn parse_output(&self, output_data: &[f32]) -> Vec<Self::Output>;
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
pub struct YoloDetection {
    /// Number of object classes (e.g., COCO dataset has 80 classes).
    pub num_classes: usize,
    /// Maximum number of bounding boxes per class.
    pub max_bboxes_per_class: usize,
    /// Confidence threshold for detections.
    pub threshold: f32,
}

/// Represents a detection result for the YOLO Detection network.
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

    fn parse_output(&self, output_data: &[f32]) -> Vec<Self::Output> {
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

        detections
    }
}

/// Configuration for the YOLO Pose estimation network.
pub struct YoloPose {
    /// Number of keypoints to detect (e.g., 17 for human pose estimation).
    pub num_keypoints: usize,
    /// Confidence threshold for pose detections.
    pub threshold: f32,
}

/// Represents a pose estimation result for the YOLO Pose network.
pub struct Pose {
    /// List of (x, y) coordinates for detected keypoints.
    pub keypoints: Vec<(f32, f32)>,
    /// Confidence score of the pose estimation.
    pub confidence: f32,
}

/// Implementation of the `Network` trait for `YoloPose`.
///
/// This parses the raw output data into a list of `Pose` objects.
impl Network for YoloPose {
    type Output = Pose;

    fn parse_output(&self, output_data: &[f32]) -> Vec<Self::Output> {
        let mut poses = Vec::new();
        let mut offset = 0;

        // Parse the raw output data for each pose.
        while offset < output_data.len() {
            // Extract confidence score for the pose.
            let confidence = output_data[offset + self.num_keypoints * 2];

            // Add the pose if confidence is above the threshold.
            if confidence >= self.threshold {
                let mut keypoints = Vec::new();
                // Extract keypoints (x, y) coordinates.
                for i in 0..self.num_keypoints {
                    let x = output_data[offset + i * 2];
                    let y = output_data[offset + i * 2 + 1];
                    keypoints.push((x, y));
                }

                poses.push(Pose {
                    keypoints,
                    confidence,
                });
            }

            // Move to the next pose in the output data.
            offset += self.num_keypoints * 2 + 1;
        }

        poses
    }
}
