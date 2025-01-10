use hailors::{scan_devices, open_device, close_device, create_vdevice, load_hef, create_vstreams, run_inference, release_vdevice};
use std::env;
use std::ptr;
use autocxx::c_void; 

fn main() {
    let args: Vec<String> = env::args().collect();
    let device_id = if args.len() > 1 {
        Some(args[1].clone())
    } else {
        None
    };

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
                vec![] 
                //return;
            }
            Err(e) => {
                eprintln!("Error scanning devices: {}", e);
                vec![] 
                //return;
            }
        },
    };

    for id in device_ids {
        match open_device(&id) {
            Ok(handle) => {
                println!("Successfully opened device: {}", id);

                // Close the device after use
                if let Err(e) = close_device(handle) {
                    eprintln!("Error closing device {}: {}", id, e);
                } else {
                    println!("Successfully closed device: {}", id);
                }
            }
            Err(e) => eprintln!("Error opening device {}: {}", id, e),
        }
    }

    let hef_path = env::args().nth(1).unwrap_or_else(|| "/usr/share/hailo-models/yolov6n_h8.hef".to_string());

    // Create VDevice
    let vdevice = match create_vdevice() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error creating VDevice: {}", e);
            return;
        }
    };

    // Load HEF and configure network group
    let network_group = match load_hef(&hef_path) {
        Ok(ng) => ng,
        Err(e) => {
            eprintln!("Error loading HEF: {}", e);
            let _ = release_vdevice(vdevice);
            return;
        }
    };

    // Create input/output virtual streams
    let (input_vstreams, output_vstreams) = match create_vstreams(network_group) {
        Ok((inputs, outputs)) => (inputs, outputs),
        Err(e) => {
            eprintln!("Error creating virtual streams: {}", e);
            let _ = release_vdevice(vdevice);
            return;
        }
    };

    // Placeholder for the input/output params and buffers
    let inputs_params: *mut c_void = ptr::null_mut();  // Replace with actual input vstream params
    let input_buffers: *mut c_void = ptr::null_mut();  // Replace with actual input buffers
    let outputs_params: *mut c_void = ptr::null_mut(); // Replace with actual output vstream params
    let output_buffers: *mut c_void = ptr::null_mut(); // Replace with actual output buffers

    let inputs_count = input_vstreams.len();  // Number of input streams
    let outputs_count = output_vstreams.len(); // Number of output streams
    let frames_count = 1;  // Set the number of frames for inference

    // Run inference
    if let Err(e) = run_inference(
        network_group,
        inputs_params,
        input_buffers,
        inputs_count,
        outputs_params,
        output_buffers,
        outputs_count,
        frames_count,
    ) {
        eprintln!("Inference failed: {}", e);
    }
    // Release VDevice
    if let Err(e) = release_vdevice(vdevice) {
        eprintln!("Error releasing VDevice: {}", e);
    }
}
