#[cfg(test)]
mod tests {
    use hailors::*;
    use std::ptr;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_device_creation_and_release() {
        let device = Arc::new(Mutex::new(HailoDevice::new().expect("Failed to create HailoDevice")));
        let mut device_lock = device.lock().unwrap();
        assert!(device_lock.release().is_ok(), "Failed to release HailoDevice");
    }

    #[test]
    fn test_hef_configuration() {
        let device = Arc::new(Mutex::new(HailoDevice::new().expect("Failed to create HailoDevice")));
        let device = device.lock().unwrap();

        let hef_path = "./hef/yolov6n_h8.hef";
        let mut input_vstreams: [*mut HailoInputVStreamHandle; 16] = [ptr::null_mut(); 16];
        let mut output_vstreams: [*mut HailoOutputVStreamHandle; 16] = [ptr::null_mut(); 16];

        let input_vstreams_slice = unsafe {
            std::slice::from_raw_parts_mut(
                input_vstreams.as_mut_ptr() as *mut *mut std::ffi::c_void,
                input_vstreams.len(),
            )
        };
        let output_vstreams_slice = unsafe {
            std::slice::from_raw_parts_mut(
                output_vstreams.as_mut_ptr() as *mut *mut std::ffi::c_void,
                output_vstreams.len(),
            )
        };

        let result = device.configure_hef(hef_path, input_vstreams_slice, output_vstreams_slice);
        assert!(result.is_ok(), "Failed to configure HEF: {:?}", result.err());
    }

    #[test]
    fn test_inference() {
        let device = Arc::new(Mutex::new(HailoDevice::new().expect("Failed to create device")));
        let device = device.lock().unwrap();

        let hef_path = "./hef/yolov6n_h8.hef";
        let mut input_vstreams: [*mut HailoInputVStreamHandle; 16] = [ptr::null_mut(); 16];
        let mut output_vstreams: [*mut HailoOutputVStreamHandle; 16] = [ptr::null_mut(); 16];

        let input_vstreams_slice = unsafe {
            std::slice::from_raw_parts_mut(
                input_vstreams.as_mut_ptr() as *mut *mut std::ffi::c_void,
                input_vstreams.len(),
            )
        };
        let output_vstreams_slice = unsafe {
            std::slice::from_raw_parts_mut(
                output_vstreams.as_mut_ptr() as *mut *mut std::ffi::c_void,
                output_vstreams.len(),
            )
        };

        let network_group = device
            .configure_hef(hef_path, input_vstreams_slice, output_vstreams_slice)
            .expect("Failed to configure HEF");
        // Resize slices to match the actual stream counts.
        let input_count = input_vstreams_slice.iter().filter(|v| !v.is_null()).count();
        let output_count = output_vstreams_slice.iter().filter(|v| !v.is_null()).count();


        assert_eq!(input_count, 1, "Unexpected number of input streams.");
        assert_eq!(output_count, 1, "Unexpected number of output streams.");
            

        // Check input and output vstreams are valid (similar to C++ ASSERT)
        for (i, vstream) in input_vstreams_slice.iter().enumerate() {
            assert!(!vstream.is_null(), "Input vstream {} is null!", i);
        }
        for (i, vstream) in output_vstreams_slice.iter().enumerate() {
            assert!(!vstream.is_null(), "Output vstream {} is null!", i);
        }


        let infer_result = device.infer(network_group, input_vstreams_slice, output_vstreams_slice);
        assert!(infer_result.is_ok(), "Failed to perform inference: {:?}", infer_result.err());
    }
}
