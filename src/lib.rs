mod status;

use autocxx::prelude::*;
use std::ptr;
use std::ffi::CString;
use status::HailoStatus;

include_cpp! {
    #include "device_api_wrapper.hpp"
    safety!(unsafe_ffi)
    generate!("hailors_open_device")
    generate!("hailors_close_device")
    generate!("hailors_vdevice_create")
    generate!("hailors_load_hef")
    generate!("hailors_release_vdevice")
    generate!("hailors_scan_devices")
    generate!("hailors_create_input_vstreams")
    generate!("hailors_create_output_vstreams")
    generate!("hailo_status")
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HailoFormatType {
    UINT8 = 0,
    INT16,
    FLOAT32,
}

extern "C" {
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

pub fn scan_devices() -> Result<Vec<String>, String> {
    const MAX_DEVICES: usize = 32;
    let mut device_ids: [u8; 64 * MAX_DEVICES] = [0; 64 * MAX_DEVICES];
    let mut count = MAX_DEVICES;

    let status = unsafe { ffi::hailors_scan_devices(device_ids.as_mut_ptr() as *mut _, &mut count) };
    let hailo_status = HailoStatus::from_i32(status as i32);
    if hailo_status != HailoStatus::Success {
        return Err(format!("Failed to scan devices: {}", hailo_status));
    }

    let mut devices = Vec::new();
    for i in 0..count {
        let id_start = i * 64;
        let id_end = id_start + 64;
        if let Ok(id) = std::str::from_utf8(&device_ids[id_start..id_end]).map(|s| s.trim_matches(char::from(0)).to_string()) {
            devices.push(id);
        }
    }
    Ok(devices)
}

pub fn open_device(device_id: &str) -> Result<*mut c_void, String> {
    let c_device_id = CString::new(device_id).map_err(|_| "Invalid device ID".to_string())?;
    let mut device_handle: *mut c_void = std::ptr::null_mut();
    let status = unsafe { ffi::hailors_open_device(c_device_id.as_ptr(), &mut device_handle) };

    let hailo_status = HailoStatus::from_i32(status as i32);
    if hailo_status == HailoStatus::Success {
        Ok(device_handle)
    } else {
        Err(format!("Failed to open device: {}", hailo_status))
    }
}

pub fn close_device(device_handle: *mut c_void) -> Result<(), String> {
    let status = unsafe { ffi::hailors_close_device(device_handle) };
    let hailo_status = HailoStatus::from_i32(status as i32);
    if hailo_status == HailoStatus::Success {
        Ok(())
    } else {
        Err(format!("Failed to close device: {}", hailo_status))
    }
}

pub fn create_vdevice() -> Result<*mut c_void, String> {
    let mut vdevice_handle: *mut c_void = std::ptr::null_mut();
    let status = unsafe { ffi::hailors_vdevice_create(&mut vdevice_handle) };
    let hailo_status = HailoStatus::from_i32(status as i32);
    if hailo_status == HailoStatus::Success {
        Ok(vdevice_handle)
    } else {
        Err(format!("Failed to create VDevice: {}", hailo_status))
    }
}

pub fn release_vdevice(vdevice_handle: *mut c_void) -> Result<(), String> {
    let status = unsafe { ffi::hailors_release_vdevice(vdevice_handle) };
    let hailo_status = HailoStatus::from_i32(status as i32);
    if hailo_status == HailoStatus::Success {
        Ok(())
    } else {
        Err(format!("Failed to release VDevice: {}", hailo_status))
    }
}

pub fn load_hef(hef_path: &str, vdevice_handle: *mut c_void) -> Result<*mut c_void, String> {
    let c_hef_path = CString::new(hef_path).map_err(|_| "Invalid HEF path".to_string())?;
    let mut network_group_handle: *mut c_void = std::ptr::null_mut();

    let status = unsafe { ffi::hailors_load_hef(c_hef_path.as_ptr(), &mut network_group_handle, vdevice_handle as *mut _) };
    let hailo_status = HailoStatus::from_i32(status as i32);
    if hailo_status == HailoStatus::Success {
        Ok(network_group_handle)
    } else {
        Err(format!("Failed to load HEF: {}", hailo_status))
    }
}

pub fn create_vstreams(
    network_group: *mut c_void,
    format_type: HailoFormatType,
    max_params_count: usize,
) -> Result<(Vec<Vec<u8>>, Vec<Vec<u8>>), String> {
    let mut input_vstreams: Vec<*mut c_void> = vec![std::ptr::null_mut(); max_params_count];
    let mut output_vstreams: Vec<*mut c_void> = vec![std::ptr::null_mut(); max_params_count];

    let mut input_count: usize = 0;
    let mut output_count: usize = 0;

    // Call to `hailors_create_input_vstreams`
    let status = unsafe {
        ffi::hailors_create_input_vstreams(
            network_group,
            std::ptr::null_mut(),
            max_params_count,
            input_vstreams.as_mut_ptr(),
            &mut input_count,
        )
    };
    let hailo_status = HailoStatus::from_i32(status as i32);
    if hailo_status != HailoStatus::Success {
        return Err(format!("Failed to create input vstreams: {:?}", hailo_status));
    }

    // Call to `hailors_create_output_vstreams`
    let status = unsafe {
        ffi::hailors_create_output_vstreams(
            network_group,
            std::ptr::null_mut(),
            max_params_count,
            output_vstreams.as_mut_ptr(),
            &mut output_count,
        )
    };
    let hailo_status = HailoStatus::from_i32(status as i32);
    if hailo_status != HailoStatus::Success {
        return Err(format!("Failed to create output vstreams: {:?}", hailo_status));
    }

    println!("Created {} input vstreams and {} output vstreams", input_count, output_count);

    let input_buffers = vec![vec![0u8; 4096]; input_count];
    let output_buffers = vec![vec![0u8; 4096]; output_count];

    Ok((input_buffers, output_buffers))
}

pub fn run_inference(
    network_group: *mut c_void,
    inference_buffers: &InferenceBuffers,
    frames_count: usize,
) -> Result<(), String> {
    let status = unsafe {
        hailors_infer(
            network_group,
            ptr::null_mut(),
            inference_buffers.input_buffers_ptrs.as_ptr() as *mut c_void,
            inference_buffers.input_buffers_ptrs.len(),
            ptr::null_mut(),
            inference_buffers.output_buffers_ptrs.as_ptr() as *mut c_void,
            inference_buffers.output_buffers_ptrs.len(),
            frames_count,
        )
    };

    let hailo_status = HailoStatus::from_i32(status as i32);
    if hailo_status == HailoStatus::Success {
        println!("Inference completed successfully!");
        Ok(())
    } else {
        Err(format!("Inference execution failed: {}", hailo_status))
    }
}

pub struct InferenceBuffers {
    pub input_buffers: Vec<Vec<u8>>,
    pub output_buffers: Vec<Vec<u8>>,
    pub input_buffers_ptrs: Vec<*mut c_void>,
    pub output_buffers_ptrs: Vec<*mut c_void>,
}

impl InferenceBuffers {
    pub fn new(input_buffers: &[Vec<u8>], output_buffers: &[Vec<u8>]) -> Self {
        let input_buffers_ptrs: Vec<*mut c_void> = input_buffers.iter().map(|buf| buf.as_ptr() as *mut c_void).collect();
        let output_buffers_ptrs: Vec<*mut c_void> = output_buffers.iter().map(|buf| buf.as_ptr() as *mut c_void).collect();

        InferenceBuffers {
            input_buffers: input_buffers.to_vec(),
            output_buffers: output_buffers.to_vec(),
            input_buffers_ptrs,
            output_buffers_ptrs,
        }
    }
}
