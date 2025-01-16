pub trait Network {
    /// The output type of the network (e.g., Detection, Pose)
    type Output;

    /// Parses the raw output data into the associated output type
    fn parse_output(&self, output_data: &[f32]) -> Vec<Self::Output>;
}


/// Supported network types for the CLI
#[derive(clap::ValueEnum, Clone, Debug)]
pub enum NetworkType {
    YoloDetection,
    YoloPose,
}





pub struct YoloDetection {
    pub num_classes: usize,
    pub max_bboxes_per_class: usize,
    pub threshold: f32,
}

pub struct Detection {
    pub class_id: u32,
    pub confidence: f32,
    pub bbox: (f32, f32, f32, f32), // (x_min, y_min, x_max, y_max)
}

impl Network for YoloDetection {
    type Output = Detection;

    fn parse_output(&self, output_data: &[f32]) -> Vec<Self::Output> {
        let mut detections = Vec::new();
        let mut offset = 0;

        for class_id in 0..self.num_classes {
            let bbox_count = output_data[offset] as usize; // Number of bounding boxes for this class
            offset += 1;

            for _ in 0..bbox_count {
                if offset + 6 > output_data.len() {
                    break;
                }

                let confidence = output_data[offset + 4];
                if confidence >= self.threshold {
                    detections.push(Detection {
                        class_id: class_id as u32,
                        confidence,
                        bbox: (
                            output_data[offset],     // x_min
                            output_data[offset + 1], // y_min
                            output_data[offset + 2], // x_max
                            output_data[offset + 3], // y_max
                        ),
                    });
                }
                offset += 6;
            }
        }

        detections
    }
}



pub struct YoloPose {
    pub num_keypoints: usize,
    pub threshold: f32,
}


pub struct Pose {
    pub keypoints: Vec<(f32, f32)>, // List of (x, y) keypoints
    pub confidence: f32,
}

impl Network for YoloPose {
    type Output = Pose;

    fn parse_output(&self, output_data: &[f32]) -> Vec<Self::Output> {
        let mut poses = Vec::new();
        let mut offset = 0;

        while offset < output_data.len() {
            let confidence = output_data[offset + self.num_keypoints * 2];
            if confidence >= self.threshold {
                let mut keypoints = Vec::new();
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
            offset += self.num_keypoints * 2 + 1;
        }

        poses
    }
}
