use autocxx::prelude::*;
//use std::ffi::CStr;
use std::ptr;
use std::ffi::{CString,CStr};
use std::os::raw::c_char;

include_cpp! {
    #include "device_api_wrapper.hpp"
    safety!(unsafe_ffi)
    generate!("hailors_open_device")
    generate!("hailors_close_device")

    generate!("hailo_status")
    generate!("hailors_vdevice_create")
    generate!("hailors_load_hef")

    generate!("hailors_release_vdevice")
    generate!("hailors_scan_devices")

}
extern "C" {
    fn hailors_create_vstreams(
        network_group: *mut c_void,
        input_params: *mut c_void,
        input_params_count: usize,
        output_params: *mut c_void,
        output_params_count: usize,
        input_vstreams: *mut *mut c_void,
        input_count: *mut usize,
        output_vstreams: *mut *mut c_void,
        output_count: *mut usize,
    ) -> i32;
 
    fn hailors_infer(
            network_group: *mut c_void,
            inputs_params: *mut c_void,
            input_buffers: *mut c_void,
            inputs_count: usize,
            outputs_params: *mut c_void,
            output_buffers: *mut c_void,
            outputs_count: usize,
            frames_count: usize,
    ) -> i32;
    
}



#[repr(C)]
#[derive(Clone, PartialEq, Eq)]
pub enum HailoStatus {
    Success = 0,
    Failure = 1,  // Add other variants as needed
}

impl std::fmt::Debug for HailoStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HailoStatus::Success => write!(f, "HailoStatus::Success"),
            HailoStatus::Failure => write!(f, "HailoStatus::Failure"),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct HailoDeviceId {
    pub id: [u8; 64], // Assuming HAILO_MAX_DEVICE_ID_LENGTH is 64
}

impl HailoDeviceId {
    fn as_cstr(&self) -> &CStr {
        let end = self.id.iter().position(|&c| c == 0).unwrap_or(self.id.len());
        unsafe { CStr::from_bytes_with_nul_unchecked(&self.id[..=end]) }
    }
}

type HailoDeviceHandle = *mut c_void;
type VDeviceHandle = *mut c_void;
type NetworkGroupHandle = *mut c_void;
type VStreamHandle = *mut c_void;

pub fn create_vdevice() -> Result<VDeviceHandle, String> {
    let mut vdevice: VDeviceHandle = ptr::null_mut();
    let status = unsafe { ffi::hailors_vdevice_create(&mut vdevice) };
    if status as i32 == HailoStatus::Success as i32 {
        Ok(vdevice)
    } else {
        Err(format!("Failed to create VDevice"))
    }
}

pub fn load_hef(hef_path: &str) -> Result<NetworkGroupHandle, String> {
    let c_hef_path = CString::new(hef_path).map_err(|_| "Invalid HEF path".to_string())?;
    let mut network_group: NetworkGroupHandle = ptr::null_mut();
    let status = unsafe { ffi::hailors_load_hef(c_hef_path.as_ptr(), &mut network_group) };
    if status as i32 == HailoStatus::Success as i32 {
        Ok(network_group)
    } else {
        Err(format!("Failed to load HEF: {}", hef_path))
    }
}

/// Create input and output virtual streams.
#[allow(non_snake_case)]
pub fn create_vstreams(network_group: NetworkGroupHandle) -> Result<(Vec<VStreamHandle>, Vec<VStreamHandle>), String> {

    let mut input_count: usize = 0;
    let mut output_count: usize = 0;
    let mut input_vstreams: Vec<*mut c_void> = vec![ptr::null_mut(); input_count];
    let mut output_vstreams: Vec<*mut c_void> = vec![ptr::null_mut(); output_count];


    let status = unsafe {
        hailors_create_vstreams(
            network_group,
            ptr::null_mut(),
            0,
            ptr::null_mut(),
            0,
            input_vstreams.as_mut_ptr(),
            &mut input_count,
            output_vstreams.as_mut_ptr(),
            &mut output_count,
        )
    };

    if status as i32 == HailoStatus::Success as i32 {
        Ok((
            input_vstreams.iter().map(|&ptr| ptr).collect(),
            output_vstreams.iter().map(|&ptr| ptr).collect(),
        ))
    } else {
        Err("Failed to create virtual streams".to_string())
    }
}


pub fn run_inference(
    network_group: NetworkGroupHandle,
    inputs_params: *mut c_void,
    input_buffers: *mut c_void,
    inputs_count: usize,
    outputs_params: *mut c_void,
    output_buffers: *mut c_void,
    outputs_count: usize,
    frames_count: usize,
) -> Result<(), String> {
    let status = unsafe {
        hailors_infer(
            network_group,
            inputs_params,
            input_buffers,
            inputs_count,
            outputs_params,
            output_buffers,
            outputs_count,
            frames_count,
        )
    };

    if status as i32 == HailoStatus::Success as i32 {
        Ok(())
    } else {
        Err("Inference failed".to_string())
    }
}

pub fn release_vdevice(vdevice: VDeviceHandle) -> Result<(), String> {
    let status = unsafe { ffi::hailors_release_vdevice(vdevice) };
    if status as i32 == HailoStatus::Success as i32 {
        Ok(())
    } else {
        Err(format!("Failed to release VDevice"))
    }
}

/// Open a Hailo device by device ID.
pub fn open_device(device_id: &str) -> Result<HailoDeviceHandle, String> {
    let c_device_id = std::ffi::CString::new(device_id).map_err(|_| "Invalid device ID".to_string())?;
    let mut device_handle: HailoDeviceHandle = ptr::null_mut();

    let status = unsafe {
        ffi::hailors_open_device(c_device_id.as_ptr(), &mut device_handle)
    };

    if status as i32 == HailoStatus::Success as i32 {
        Ok(device_handle)
    } else {
        Err(format!("Failed to open device"))
    }
}

/// Close a Hailo device by handle.
pub fn close_device(device_handle: HailoDeviceHandle) -> Result<(), String> {
    let status = unsafe { ffi::hailors_close_device(device_handle) };

    if status as i32 == HailoStatus::Success as i32 {
        Ok(())
    } else {
        Err(format!("Failed to close device"))
    }
}

pub fn scan_devices() -> Result<Vec<String>, String> {
    const MAX_DEVICES: usize = 32;

    let mut device_ids: [HailoDeviceId; MAX_DEVICES] = unsafe { std::mem::zeroed() };
    let mut device_count: usize = MAX_DEVICES;

    let status = unsafe { ffi::hailors_scan_devices(device_ids.as_mut_ptr() as *mut _, &mut device_count) };

    if status as i32 != HailoStatus::Success as i32 {
        return Err(format!("Failed to scan devices"));
    }

    let mut devices = Vec::new();
    for i in 0..device_count {
        let c_str = device_ids[i].as_cstr();
        match c_str.to_str() {
            Ok(device_str) => devices.push(device_str.to_string()),
            Err(_) => return Err("Invalid UTF-8 device ID".to_string()),
        }
    }

    Ok(devices)
}


