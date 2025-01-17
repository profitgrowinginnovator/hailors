use std::ffi::{CString, c_void};
use std::ptr;
use anyhow::Result;

mod status;
use status::HailoStatus;
pub mod network;
pub use crate::network::Network;

/// Represents a device for interfacing with the Hailo AI hardware.
pub struct HailoDevice {
    /// Handle to the Hailo device.
    pub device_handle: *mut c_void,
    /// Handle to the configured network group.
    pub network_group: *mut c_void,
    /// Pointer to an array of input virtual streams.
    pub input_vstream: *mut *mut c_void,
    /// Pointer to an array of output virtual streams.
    pub output_vstream: *mut *mut c_void,
    /// Size of the input frame in bytes.
    pub input_frame_size: usize,
    /// Size of the output frame in bytes.
    pub output_frame_size: usize,
}

impl HailoDevice {
    /// Creates a new Hailo device and configures it with the provided HEF file.
    ///
    /// # Arguments
    ///
    /// * `hef_path` - Path to the Hailo Execution File (HEF).
    ///
    /// # Returns
    ///
    /// Returns a `HailoDevice` instance on success or an error on failure.
    ///
    /// # Example
    ///
    /// ```rust
    /// use hailors::HailoDevice;
    ///
    /// let device = HailoDevice::new("./hef/yolov8s_h8.hef")
    ///     .expect("Failed to create HailoDevice");
    /// ```
    pub fn new(hef_path: &str) -> Result<Self> {
        let mut device_handle: *mut c_void = ptr::null_mut();
        let mut network_group: *mut c_void = ptr::null_mut();
        let mut input_vstreams: *mut *mut c_void = ptr::null_mut();
        let mut output_vstreams: *mut *mut c_void = ptr::null_mut();
        let mut input_count: usize = 0;
        let mut output_count: usize = 0;
        let mut input_frame_size: usize = 0;
        let mut output_frame_size: usize = 0;

        // Call FFI function to configure the HEF and virtual streams
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
                &mut input_vstreams,
                &mut input_count,
                &mut output_vstreams,
                &mut output_count,
                &mut input_frame_size,
                &mut output_frame_size,
            );
            if configure_status != HailoStatus::Success {
                return Err(anyhow::anyhow!("Failed to configure HEF"));
            }

            if input_vstreams.is_null() || output_vstreams.is_null() {
                hailors_release_vdevice(device_handle);
                return Err(anyhow::anyhow!("Failed to allocate input or output vstreams"));
            }
        }

        Ok(Self {
            device_handle,
            network_group,
            input_vstream: input_vstreams,
            output_vstream: output_vstreams,
            input_frame_size,
            output_frame_size,
        })
    }

    /// Writes a frame to the input virtual stream.
    ///
    /// # Arguments
    ///
    /// * `frame` - A byte slice representing the input frame.
    ///
    /// # Errors
    ///
    /// Returns an error if the input frame size does not match the expected size or if writing fails.
    pub fn write_input(&self, frame: &[u8]) -> Result<()> {
        if frame.len() != self.input_frame_size {
            return Err(anyhow::anyhow!(
                "Input frame size mismatch: expected {}, got {}",
                self.input_frame_size,
                frame.len()
            ));
        }

        unsafe {
            let status = hailors_write_input_frame(*self.input_vstream, frame.as_ptr() as *const c_void, frame.len());
            if status != HailoStatus::Success {
                return Err(anyhow::anyhow!("Failed to write input frame"));
            }
        }
        Ok(())
    }

    /// Reads the output virtual stream and parses detection results.
    ///
    /// # Type Parameters
    ///
    /// * `T` - A type implementing the `Network` trait.
    ///
    /// # Arguments
    ///
    /// * `network_type` - A reference to the network type used to parse the output.
    ///
    /// # Returns
    ///
    /// Returns a vector of parsed results of type `T::Output`.
    pub fn read_output<T: Network>(&self, network_type: &T) -> Result<Vec<T::Output>> {
        let mut output_data = vec![0.0_f32; self.output_frame_size / 4]; // FLOAT32

        unsafe {
            let status = hailors_read_output_frame(
                *self.output_vstream,
                output_data.as_mut_ptr() as *mut c_void,
                output_data.len() * 4,
            );
            if status != HailoStatus::Success {
                return Err(anyhow::anyhow!("Failed to read output frame"));
            }
        }

        let results = network_type.parse_output(&output_data);
        Ok(results)
    }
}

impl Drop for HailoDevice {
    /// Releases the Hailo device and associated resources when the `HailoDevice` is dropped.
    fn drop(&mut self) {
        unsafe {
            hailors_release_vdevice(self.device_handle);
        }
    }
}

extern "C" {
    /// Creates a Hailo virtual device.
    fn hailors_create_vdevice(device_handle: *mut *mut c_void) -> HailoStatus;

    /// Configures a Hailo Execution File (HEF) and sets up virtual streams.
    fn hailors_configure_hef(
        device_handle: *mut c_void,
        hef_path: *const i8,
        network_group: *mut *mut c_void,
        input_vstreams: *mut *mut *mut c_void,
        input_count: *mut usize,
        output_vstreams: *mut *mut *mut c_void,
        output_count: *mut usize,
        input_frame_size: *mut usize,
        output_frame_size: *mut usize,
    ) -> HailoStatus;

    /// Writes a frame to the input virtual stream.
    fn hailors_write_input_frame(input_vstream: *mut c_void, data: *const c_void, len: usize) -> HailoStatus;

    /// Reads data from the output virtual stream.
    fn hailors_read_output_frame(output_vstream: *mut c_void, data: *mut c_void, len: usize) -> HailoStatus;

    /// Releases a Hailo virtual device.
    fn hailors_release_vdevice(device_handle: *mut c_void) -> HailoStatus;
}
