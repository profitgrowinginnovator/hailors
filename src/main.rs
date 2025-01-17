use clap::Parser;
use anyhow::Result;

pub mod network;
use hailors::network::{NetworkType, YoloDetection};

/// Command-line interface for the Hailo inference application.
///
/// This CLI allows users to run inference using the Hailo AI hardware.
/// Users can specify the HEF file, input data, and the network type (e.g., YOLO Detection).
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the Hailo Execution File (HEF).
    ///
    /// This file contains the compiled neural network model for the Hailo hardware.
    #[arg(short, long)]
    hef: String,

    /// Input file to process.
    ///
    /// The input file should match the dimensions and format expected by the model.
    /// For example, an image in `.rgb` format with appropriate resolution.
    #[arg(short, long)]
    input: String,

    /// Select the network type.
    ///
    /// Choose between available network types, such as YOLO Detection or YOLO Pose.
    #[arg(short, long, value_enum)]
    network: NetworkType,

    /// Confidence threshold for detections (default: 0.5).
    ///
    /// Only detections with a confidence score above this threshold will be reported.
    #[arg(short, long, default_value = "0.5")]
    threshold: f32,
}

fn main() -> Result<()> {
    // Parse command-line arguments provided by the user.
    let cli = Cli::parse();

    // Initialize the Hailo device with the provided HEF file.
    let device = hailors::HailoDevice::new(&cli.hef)
        .expect("Failed to initialize the Hailo device with the specified HEF file.");

    // Load the input data (e.g., an image file) into memory.
    let input_data = std::fs::read(&cli.input)
        .expect("Failed to read the specified input file.");

    // Determine the network type specified by the user.
    match cli.network {
        // YOLO Detection branch: Processes the input data for object detection.
        NetworkType::YoloDetection => {
            // Configure the YOLO Detection network with specified parameters.
            let network = YoloDetection {
                num_classes: 80,              // Number of object classes (e.g., COCO dataset classes).
                max_bboxes_per_class: 100,    // Maximum bounding boxes per class.
                threshold: cli.threshold,     // Confidence threshold for detections.
            };

            // Write the input data to the Hailo device for inference.
            device.write_input(&input_data)
                .expect("Failed to write input frame to the Hailo device.");

            // Perform inference and parse the output into detection results.
            let detections = device.read_output(&network)
                .expect("Failed to read and parse output from the Hailo device.");

            // Iterate over and display the detection results.
            for detection in detections {
                println!(
                    "YOLO Detection: Class {}, Confidence {:.2}, BBox: ({:.2}, {:.2}, {:.2}, {:.2})",
                    detection.class_id,       // Detected object class ID.
                    detection.confidence,     // Confidence score of the detection.
                    detection.bbox.0,         // Bounding box: Top-left X coordinate.
                    detection.bbox.1,         // Bounding box: Top-left Y coordinate.
                    detection.bbox.2,         // Bounding box: Bottom-right X coordinate.
                    detection.bbox.3          // Bounding box: Bottom-right Y coordinate.
                );
            }
        }

        // YOLO Pose branch: Processes the input data for pose detection.
        NetworkType::YoloPose => {
            // Configure the YOLO Pose network with specified parameters.
            let network = hailors::network::YoloPose {
                num_keypoints: 17,            // Number of keypoints for pose detection.
                threshold: cli.threshold,     // Confidence threshold for detections.
            };

            // Write the input data to the Hailo device for inference.
            device.write_input(&input_data)
                .expect("Failed to write input frame to the Hailo device.");

            // Perform inference and parse the output into pose results.
            let poses = device.read_output(&network)
                .expect("Failed to read and parse output from the Hailo device.");

            // Iterate over and display the pose detection results.
            for pose in poses {
                println!(
                    "Pose Detection: Confidence {:.2}, Keypoints: {:?}",
                    pose.confidence,           // Confidence score of the pose.
                    pose.keypoints             // List of keypoints (X, Y coordinates).
                );
            }
        }
    }

    Ok(())
}
