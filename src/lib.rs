// Include the necessary imports and macros.
pub mod status;

pub use status::HailoStatus;
pub use autocxx::prelude::*;
use std::ffi::CString;
use std::ptr;




include_cpp! {
    #include "custom_wrapper.hpp"
    #include "device_api_wrapper.hpp"
    generate!("hailors_create_vdevice")
    generate!("hailors_release_vdevice")
    generate!("hailors_configure_hef")
    generate!("hailors_infer")
    safety!(unsafe)
}

pub type ConfiguredNetworkGroup = c_void;
pub type InputVStream = c_void;
pub type OutputVStream = c_void;

/// A struct representing a `HailoDevice`.
pub struct HailoDevice {
    handle: *mut c_void,
}

impl HailoDevice {
    /// Creates a new HailoDevice.
    pub fn new() -> Result<Self, HailoStatus> {
        let mut raw_handle: *mut c_void = ptr::null_mut();
        let status = unsafe { ffi::hailors_create_vdevice(&mut raw_handle as *mut *mut c_void) };

        if status == ffi::hailo_status::HAILO_SUCCESS {
            Ok(Self { handle: raw_handle })
        } else {
            Err(HailoStatus::from_i32(status as i32))
        }
    }

    /// Configures a HEF file for the device.
    pub fn configure_hef(
        &self,
        hef_path: &str,
        input_vstreams: &mut [*mut InputVStream],
        output_vstreams: &mut [*mut OutputVStream],
    ) -> Result<*mut ConfiguredNetworkGroup, HailoStatus> {
        let mut network_group: *mut ConfiguredNetworkGroup = ptr::null_mut();
        let c_hef_path = CString::new(hef_path).expect("Invalid HEF path");

        let mut input_count = input_vstreams.len();
        let mut output_count = output_vstreams.len();

        let status = unsafe {
            ffi::hailors_configure_hef(
                self.handle,
                c_hef_path.as_ptr(),
                &mut network_group as *mut _ as *mut *mut c_void, 
                input_vstreams.as_mut_ptr() as *mut *mut c_void,
                &mut input_count,
                output_vstreams.as_mut_ptr() as *mut *mut c_void, 
                &mut output_count,
            )
        };

        if status == ffi::hailo_status::HAILO_SUCCESS {
            Ok(network_group)
        } else {
            Err(HailoStatus::from_i32(status as i32))
        }
    }

    /// Releases the device.
    pub fn release(&mut self) -> Result<(), HailoStatus> {
        if self.handle.is_null() {
            return Err(HailoStatus::InvalidOperation);
        }

        let status = unsafe { ffi::hailors_release_vdevice(self.handle) };
        if status == ffi::hailo_status::HAILO_SUCCESS {
            self.handle = ptr::null_mut();
            Ok(())
        } else {
            Err(HailoStatus::from_i32(status as i32))
        }
    }

    /// Inference function for testing the configuration.
    pub fn infer(
        &self,
        network_group: *mut ConfiguredNetworkGroup,
        input_vstreams: &mut [*mut InputVStream],  // `&mut` instead of `&`
        output_vstreams: &mut [*mut OutputVStream],
    ) -> Result<(), HailoStatus> {
        let status = unsafe {
            ffi::hailors_infer(
                network_group as *mut c_void,
                input_vstreams.as_mut_ptr() as *mut *mut c_void,  // Cast to `*mut *mut c_void`
                input_vstreams.len(),
                output_vstreams.as_mut_ptr() as *mut *mut c_void, // Cast to `*mut *mut c_void`
                output_vstreams.len(),
            )
        };

        if status == ffi::hailo_status::HAILO_SUCCESS {
            Ok(())
        } else {
            Err(HailoStatus::from_i32(status as i32))
        }
    }
}

impl Drop for HailoDevice {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            if let Err(e) = self.release() {
                eprintln!("Failed to release VDevice: {}", e);
            }
        }
    }
}


