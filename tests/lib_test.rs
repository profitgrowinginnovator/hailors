

#[cfg(test)]
mod tests {
    use hailors::*;

    #[test]
    fn test_device_creation_and_release() {
        let mut device = HailoDevice::new().expect("Failed to create HailoDevice");
        assert!(device.release().is_ok(), "Failed to release HailoDevice");
    }

    #[test]
    fn test_hef_configuration() {
        let device = HailoDevice::new().expect("Failed to create device");

        let hef_path = "../hef/yolov6n_h8.hef";
        let mut input_vstreams: [*mut InputVStream; 16] = [std::ptr::null_mut(); 16];  // Use `InputVStream` directly
        let mut output_vstreams: [*mut OutputVStream; 16] = [std::ptr::null_mut(); 16];  // Use `OutputVStream` directly

        let result = device.configure_hef(hef_path, &mut input_vstreams, &mut output_vstreams);
        assert!(result.is_ok(), "Failed to configure HEF: {:?}", result.err());
    }

    #[test]
    fn test_inference() {
        let device = HailoDevice::new().expect("Failed to create device");

        let hef_path = "../hef/yolov6n_h8.hef";
        let mut input_vstreams: [*mut InputVStream; 16] = [std::ptr::null_mut(); 16];
        let mut output_vstreams: [*mut OutputVStream; 16] = [std::ptr::null_mut(); 16];

        let network_group = device
            .configure_hef(hef_path, &mut input_vstreams, &mut output_vstreams)
            .expect("Failed to configure HEF");

        // Perform inference with mutable references
        let infer_result = device.infer(network_group, &mut input_vstreams, &mut output_vstreams);
        assert!(infer_result.is_ok(), "Failed to perform inference: {:?}", infer_result.err());
    }
}