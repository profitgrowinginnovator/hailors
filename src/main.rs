use hailors::{
    scan_devices, open_device, close_device, create_vdevice, load_hef, create_vstreams,
    run_inference, release_vdevice, HailoFormatType, InferenceBuffers,
};
use clap::{Arg, Command};

fn main() {
    // Create command-line argument parser
    let matches = Command::new("Hailo Inference Example")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Runs Hailo inference with optional device and HEF file selection")
        .arg(
            Arg::new("device_id")
                .short('d')
                .long("device_id")
                .value_name("DEVICE_ID")
                .help("Specify the device ID to use. If not set, the first available device will be used."),
        )
        .arg(
            Arg::new("hef")
                .long("hef")
                .value_name("HEF_PATH")
                .help("Specify the HEF file path. If not set, the default HEF file will be used."),
        )
        .get_matches();

    // Parse arguments
    let device_id = matches.get_one::<String>("device_id").cloned();
    let hef_path = matches
        .get_one::<String>("hef")
        .cloned()
        .unwrap_or_else(|| "/usr/share/hailo-models/yolov6n_h8.hef".to_string());

    println!("Using HEF file: {}", hef_path);

    let device_ids = match device_id {
        Some(id) => vec![id],
        None => match scan_devices() {
            Ok(ids) if !ids.is_empty() => {
                println!("Found {} devices:", ids.len());
                for (i, id) in ids.iter().enumerate() {
                    println!("{}. {}", i + 1, id);
                }
                ids
            }
            Ok(_) => {
                eprintln!("No devices found.");
                return;
            }
            Err(e) => {
                eprintln!("Error scanning devices: {}", e);
                return;
            }
        },
    };

    for id in device_ids {
        let device_handle = match open_device(&id) {
            Ok(handle) => {
                println!("Successfully opened device: {}", id);
                handle
            }
            Err(e) => {
                eprintln!("Error opening device {}: {}", id, e);
                continue;
            }
        };

        // Create VDevice
        let vdevice = match create_vdevice() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error creating VDevice: {}", e);
                let _ = close_device(device_handle);
                return;
            }
        };

        // Load HEF and configure network group
        let network_group = match load_hef(&hef_path, vdevice) {
            Ok(ng) => ng,
            Err(e) => {
                eprintln!("Error loading HEF: {}", e);
                let _ = release_vdevice(vdevice);
                let _ = close_device(device_handle);
                return;
            }
        };
 
        let max_params_count = 10;

        // Input/output stream format
        let format_type = HailoFormatType::UINT8;

        // Create input and output vstreams
        let (input_buffers, output_buffers) = match create_vstreams(network_group, format_type, Some(max_params_count)) {
            Ok((input, output)) => (input, output),
            Err(e) => {
                eprintln!("Error creating vstreams: {}", e);
                let _ = release_vdevice(vdevice);
                let _ = close_device(device_handle);
                return;
            }
        };

        // Create inference buffers (automatically allocates input/output buffers)
        let inference_buffers = InferenceBuffers::new(&input_buffers, &output_buffers);

        // Number of frames for inference
        let frames_count = 1;

        // Run inference
        match run_inference(network_group, &inference_buffers, frames_count) {
            Ok(()) => println!("Inference completed successfully!"),
            Err(e) => eprintln!("Inference failed: {}", e),
        }

        // Release the VDevice
        if let Err(e) = release_vdevice(vdevice) {
            eprintln!("Error releasing VDevice: {}", e);
        }

        // Close the device
        if let Err(e) = close_device(device_handle) {
            eprintln!("Error closing device {}: {}", id, e);
        } else {
            println!("Successfully closed device: {}", id);
        }
    }
}
