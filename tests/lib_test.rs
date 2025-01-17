#[cfg(test)]
mod tests {
    

    
    use std::ptr;
    use std::sync::{Arc, Mutex};

    use hailors::{HailoDevice, network::YoloDetection};

    #[test]
    fn test_device_creation_and_release() {
        // Create a new HailoDevice
        let hef_path = "./hef/yolov8s_h8.hef";
        let device = Arc::new(Mutex::new(
            HailoDevice::new(hef_path).expect("Failed to create HailoDevice"),
        ));

        // Verify release of device
        {
            let device_lock = device.lock().unwrap();
            // Implicit drop will call HailoDevice's `drop` implementation
            assert!(device_lock.device_handle != ptr::null_mut(), "Device handle is null");
        }
    }

    #[test]
    fn test_hef_configuration() {
        // Create a HailoDevice
        let hef_path = "./hef/yolov8s_h8.hef";
        let device = Arc::new(Mutex::new(
            HailoDevice::new(hef_path).expect("Failed to create HailoDevice"),
        ));

        let device_lock = device.lock().unwrap();

        // Verify vstreams setup
        assert!(
            device_lock.input_vstream != ptr::null_mut(),
            "Input vstream is null"
        );
        assert!(
            device_lock.output_vstream != ptr::null_mut(),
            "Output vstream is null"
        );
        assert!(
            device_lock.input_frame_size > 0,
            "Input frame size should be greater than zero"
        );
        assert!(
            device_lock.output_frame_size > 0,
            "Output frame size should be greater than zero"
        );
    }

    #[test]
    fn test_yolo_inference() {
        // Create a HailoDevice
        let hef_path = "./hef/yolov8s_h8.hef";
        let device = Arc::new(Mutex::new(
            HailoDevice::new(hef_path).expect("Failed to create HailoDevice"),
        ));

        let device_lock = device.lock().unwrap();

        // Define a YOLO network type
        let yolo_network = YoloDetection {
            num_classes: 80,
            max_bboxes_per_class: 100,
            threshold: 0.85,
        };

 
        // Read the input RGB file
        let input_file_path = "./images/dog.rgb";
        let input_data = std::fs::read(input_file_path).expect("Failed to read input file");

        // Verify that the input file size matches the expected input frame size
        assert_eq!(
            input_data.len(),
            device_lock.input_frame_size,
            "Input file size does not match the expected frame size"
        );

        device_lock
            .write_input(&input_data)
            .expect("Failed to write input frame");

        // Perform inference and parse output
        let detections = device_lock
            .read_output(&yolo_network)
            .expect("Failed to read and parse output");

        // Verify detections
        assert!(
            !detections.is_empty(),
            "No detections found; check input or inference pipeline"
        );

        // Check for the presence of the "dog" class (assuming class ID for "dog" is 16)
        let dog_detected = detections.iter().any(|d| d.class_id == 16 && d.confidence >= 0.5);

        assert!(
            dog_detected,
            "Dog was not detected in the image with sufficient confidence"
        );

    }
}
