use std::ffi::{CString, c_void};
use std::ptr;
use anyhow::Result;

mod status;
use status::HailoStatus;
pub mod network;
pub use crate::network::Network;

pub struct HailoDevice {
    pub device_handle: *mut c_void,
    pub network_group: *mut c_void,
    pub input_vstream: *mut c_void,
    pub output_vstream: *mut c_void,
    pub input_frame_size: usize,
    pub output_frame_size: usize,
}

impl HailoDevice {
    /// Creates a new Hailo device and configures it with the provided HEF file
    pub fn new(hef_path: &str) -> Result<Self> {
        let mut device_handle: *mut c_void = ptr::null_mut();
        let mut network_group: *mut c_void = ptr::null_mut();
        let mut input_vstream: *mut c_void = ptr::null_mut();
        let mut output_vstream: *mut c_void = ptr::null_mut();
        let mut input_frame_size: usize = 0;
        let mut output_frame_size: usize = 0;

        // Call FFI function to configure the HEF and vstreams
        let hef_path_cstr = CString::new(hef_path)?;
        unsafe {
            let status = hailors_create_vdevice(&mut device_handle);
            if status != HailoStatus::Success {
                return Err(anyhow::anyhow!("Failed to create VDevice"));
            }

            let configure_status = hailors_configure_hef(
                device_handle,
                hef_path_cstr.as_ptr() as *const i8,
                &mut network_group,
                &mut input_vstream,
                &mut output_vstream,
                &mut input_frame_size,
                &mut output_frame_size,
            );
            if configure_status != HailoStatus::Success {
                return Err(anyhow::anyhow!("Failed to configure HEF"));
            }
        }

        Ok(Self {
            device_handle,
            network_group,
            input_vstream,
            output_vstream,
            input_frame_size,
            output_frame_size,
        })
    }

    /// Writes a frame to the input vstream (handles preprocessing)
    pub fn write_input(&self, frame: &[u8]) -> Result<()> {
        if frame.len() != self.input_frame_size {
            return Err(anyhow::anyhow!(
                "Input frame size mismatch: expected {}, got {}",
                self.input_frame_size,
                frame.len()
            ));
        }

        unsafe {
            let status = hailors_write_input_frame(self.input_vstream, frame.as_ptr() as *const c_void, frame.len());
            if status != HailoStatus::Success {
                return Err(anyhow::anyhow!("Failed to write input frame"));
            }
        }
        Ok(())
    }

    /// Reads the output vstream and parses detection results
    pub fn read_output<T: Network>(&self, network_type: &T) -> Result<Vec<T::Output>> {
        let mut output_data = vec![0.0_f32; self.output_frame_size / 4]; // FLOAT32

        unsafe {
            let status = hailors_read_output_frame(
                self.output_vstream,
                output_data.as_mut_ptr() as *mut c_void,
                output_data.len() * 4,
            );
            if status != HailoStatus::Success {
                return Err(anyhow::anyhow!("Failed to read output frame"));
            }
        }

        // Use the network type to parse the output data
        let results = network_type.parse_output(&output_data);
        Ok(results)
    }
}

impl Drop for HailoDevice {
    fn drop(&mut self) {
        unsafe {
            hailors_release_vdevice(self.device_handle);
        }
    }
}

extern "C" {
    fn hailors_create_vdevice(device_handle: *mut *mut c_void) -> HailoStatus;
    fn hailors_configure_hef(
        device_handle: *mut c_void,
        hef_path: *const i8,
        network_group: *mut *mut c_void,
        input_vstream: *mut *mut c_void,
        output_vstream: *mut *mut c_void,
        input_frame_size: *mut usize,
        output_frame_size: *mut usize,
    ) -> HailoStatus;
    fn hailors_write_input_frame(input_vstream: *mut c_void, data: *const c_void, len: usize) -> HailoStatus;
    fn hailors_read_output_frame(output_vstream: *mut c_void, data: *mut c_void, len: usize) -> HailoStatus;
    fn hailors_release_vdevice(device_handle: *mut c_void) -> HailoStatus;
}
