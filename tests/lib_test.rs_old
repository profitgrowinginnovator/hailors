use std::sync::{Arc, Mutex, MutexGuard};
use lazy_static::lazy_static; // Add `lazy_static` dependency

lazy_static! {
    static ref DEVICE_MUTEX: Mutex<()> = Mutex::new(()); // Global Mutex for device access
}

#[cfg(test)]
mod tests {
    use super::*;
    use hailors::{network::{YoloDetection, YoloPose}, HailoDevice};
    use std::ptr;

    fn get_device_lock() -> MutexGuard<'static, ()> {
        DEVICE_MUTEX.lock().unwrap() // Acquire the global lock
    }

    #[test]
    fn test_device_creation_and_release() {
        let _device_lock = get_device_lock(); // Lock the device for this test

        // Create a new HailoDevice
        let hef_path = "./hef/yolov8s_h8.hef";
        let device = Arc::new(Mutex::new(
            HailoDevice::new(hef_path).expect("Failed to create HailoDevice"),
        ));

        // Verify release of device
        {
            let device_lock = device.lock().unwrap();
            assert!(device_lock.device_handle != ptr::null_mut(), "Device handle is null");
        }
    }

    #[test]
    fn test_hef_configuration() {
        let _device_lock = get_device_lock(); // Lock the device for this test

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
        let _device_lock = get_device_lock(); // Lock the device for this test

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


    #[test]
    fn test_pose_inference() {
        // Path to the HEF file and input image
        let hef_path = "./hef/yolov8s_pose_h8.hef";
        let input_file_path = "./images/person.rgb";

        // Create a HailoDevice
        let device = Arc::new(Mutex::new(
            HailoDevice::new(hef_path).expect("Failed to create HailoDevice"),
        ));

        let device_lock = device.lock().unwrap();

        // Define the YOLO Pose network type
        let yolo_pose_network = YoloPose {
            num_keypoints: 17,   // Number of keypoints in pose detection
            threshold: 0.5,      // Confidence threshold for keypoints
            max_bboxes_per_class: 100,
        };

        // Read the input RGB file
        let input_data = std::fs::read(input_file_path).expect("Failed to read input file");

        // Verify that the input file size matches the expected input frame size
        assert_eq!(
            input_data.len(),
            device_lock.input_frame_size,
            "Input file size does not match the expected frame size"
        );

        // Write the input data to the device
        device_lock
            .write_input(&input_data)
            .expect("Failed to write input frame");

        // Perform inference and parse output into poses
        let poses = device_lock
            .read_output(&yolo_pose_network)
            .expect("Failed to read and parse output");

        // Verify that at least one pose is detected
        assert!(
            !poses.is_empty(),
            "No poses found; check input or inference pipeline"
        );

        println!("poses dected: {:?}", poses.len());
/* 
        // Validate the detected poses
        for pose in poses {
            assert!(
                pose.confidence >= 0.5,
                "Pose confidence is below the threshold"
            );
            assert_eq!(
                pose.keypoints.len(),
                yolo_pose_network.num_keypoints,
                "Number of keypoints does not match expected value"
            );

            // Print pose details for debugging
            println!(
                "Pose Confidence: {:.2}, Keypoints: {:?}",
                pose.confidence, pose.keypoints
            );
        }
*/        
    }


}
