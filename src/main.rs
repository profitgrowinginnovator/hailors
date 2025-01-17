use clap::Parser;
use anyhow::Result;
pub mod network;
use hailors::network::{NetworkType, YoloDetection}; // Adjusted imports



/// Command-line interface for the Hailo inference application
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the HEF file
    #[arg(short, long)]
    hef: String,

    /// Input file to process
    #[arg(short, long)]
    input: String,

    /// Select the network type
    #[arg(short, long, value_enum)]
    network: NetworkType,

    /// Confidence threshold for detections (default: 0.5)
    #[arg(short, long, default_value = "0.5")]
    threshold: f32,
}



fn main() -> Result<()> {
    // Parse command-line arguments
    let cli = Cli::parse();

    // Initialize Hailo device with the provided HEF file
    let device = hailors::HailoDevice::new(&cli.hef)?;

    // Load the input data
    let input_data = std::fs::read(&cli.input)?;

    // Choose the network type
    match cli.network {
        NetworkType::YoloDetection => {
            let network= YoloDetection {
                num_classes: 80,
                max_bboxes_per_class: 100,
                threshold: cli.threshold,
            };

            // Perform inference
            device.write_input(&input_data)?;
            let detections = device.read_output(&network)?;

            // Print results
            for detection in detections {
                println!(
                    "YOLO Detection: Class {}, Confidence {:.2}, BBox: ({:.2}, {:.2}, {:.2}, {:.2})",
                    detection.class_id, detection.confidence, detection.bbox.0, detection.bbox.1, detection.bbox.2, detection.bbox.3
                );
            }
        }
        NetworkType::YoloPose => {
            let network = hailors::network::YoloPose {
                num_keypoints: 17,
                threshold: cli.threshold,
            };

            // Perform inference
            device.write_input(&input_data)?;
            let poses = device.read_output(&network)?;

            // Print results
            for pose in poses {
                println!("Pose Detection: Confidence {:.2}, Keypoints: {:?}", pose.confidence, pose.keypoints);
            }
        }
    }

    Ok(())
}
