mod status;

use autocxx::prelude::*;
use std::mem::MaybeUninit;
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
    generate!("hailo_get_input_stream_info")
    generate!("hailo_get_output_stream_info")
    generate!("hailo_configured_network_group")
    generate!("get_shape")
    generate!("get_input_stream_info")
    generate!("get_output_stream_info")
    generate!("get_stream_name")

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

const HAILO_MAX_STREAM_NAME_SIZE: usize = 64;

#[repr(C)]
pub union HailoStreamShapeUnion {
    pub shape: Hailo3DImageShapePair,
    pub nms_info: NmsInfo,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Hailo3DImageShapePair {
    pub shape: Hailo3DImageShape,
    pub hw_shape: Hailo3DImageShape,
}

impl std::fmt::Debug for HailoStreamShapeUnion {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        unsafe {
            write!(
                f,
                "Shape: {:?}, HW Shape: {:?}",
                self.shape.shape, self.shape.hw_shape
            )
        }
    }
}


#[repr(C)]
pub struct HailoStreamInfo {
    pub shape_union: HailoStreamShapeUnion,
    pub hw_data_bytes: u32,
    pub hw_frame_size: u32,
    pub format: i32,
    pub direction: i32,
    pub index: u8,
    pub name: [u8; HAILO_MAX_STREAM_NAME_SIZE],
    pub quant_info: HailoQuantInfo,
    pub is_mux: bool,
}

impl HailoStreamInfo {
    pub fn get_name(&self) -> String {
        let c_str = unsafe { std::ffi::CStr::from_ptr(self.name.as_ptr() as *const u8) };
        c_str.to_str().unwrap_or("invalid").to_string()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Hailo3DImageShape {
    pub height: u32,
    pub width: u32,
    pub features: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NmsInfo {
    pub max_boxes: u32,
    pub classes: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HailoQuantInfo {
    pub scale: f32,
    pub zero_point: i32,
}


#[repr(C)]
#[derive(Debug)]
pub struct HailoVstreamShape {
    pub height: u32,
    pub width: u32,
    pub features: u32,  // Channels
}


#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HailoVstreamStatsFlags {
    None = 0,
    MeasureFPS = 1 << 0,
    MeasureLatency = 1 << 1,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HailoPipelineElemStatsFlags {
    None = 0,
    MeasureFPS = 1 << 0,
    MeasureLatency = 1 << 1,
    MeasureQueueSize = 1 << 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HailoVstreamParams {
    pub user_buffer_format: i32,
    pub timeout_ms: u32,
    pub queue_size: u32,
    pub vstream_stats_flags: u32,
    pub pipeline_elements_stats_flags: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HailoInputVstreamParamsByName {
    pub name: [u8; 64],
    pub params: HailoVstreamParams,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HailoOutputVstreamParamsByName {
    pub name: [u8; 64],
    pub params: HailoVstreamParams,
}


extern "C" {
    fn get_input_stream_info(stream: *mut c_void, info: *mut HailoStreamInfo) -> i32;
    fn get_output_stream_info(stream: *mut c_void, info: *mut HailoStreamInfo) -> i32;
    fn hailors_create_input_vstreams(
        network_group: *mut c_void,
        input_params: *const HailoInputVstreamParamsByName,  // Manual struct
        input_params_count: usize,
        input_vstreams: *mut *mut c_void,
        input_count: *mut usize,
    ) -> i32;

    fn hailors_create_output_vstreams(
        network_group: *mut c_void,
        output_params: *const HailoOutputVstreamParamsByName,  // Manual struct
        output_params_count: usize,
        output_vstreams: *mut *mut c_void,
        output_count: *mut usize,
    ) -> i32;
}

pub fn fetch_input_stream_info(stream: *mut c_void) -> Result<HailoStreamInfo, String> {
    let mut info: HailoStreamInfo = unsafe { std::mem::zeroed() };
    let status = unsafe { get_input_stream_info(stream, &mut info) };

    if status == 0 {
        Ok(info)
    } else {
        Err("Failed to get input stream info".to_string())
    }
}

pub fn fetch_output_stream_info(stream: *mut c_void) -> Result<HailoStreamInfo, String> {
    let mut info: HailoStreamInfo = unsafe { std::mem::zeroed() };
    let status = unsafe { get_output_stream_info(stream, &mut info) };

    if status == 0 {
        Ok(info)
    } else {
        Err("Failed to get output stream info".to_string())
    }
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

pub fn load_hef(hef_path: &str, vdevice_handle: *mut c_void) -> Result<*mut ffi::hailo_configured_network_group, String> {
    let c_hef_path = CString::new(hef_path).map_err(|_| "Invalid HEF path".to_string())?;
    let mut network_group_handle: *mut ffi::hailo_configured_network_group = std::ptr::null_mut();

    let status = unsafe { 
        ffi::hailors_load_hef(
            c_hef_path.as_ptr(), 
            &mut network_group_handle as *mut _ as *mut *mut c_void,  
            vdevice_handle as *mut _) 
        };
    let hailo_status = HailoStatus::from_i32(status as i32);
    if hailo_status == HailoStatus::Success {
        Ok(network_group_handle)
    } else {
        Err(format!("Failed to load HEF: {}", hailo_status))
    }
}
pub fn create_vstreams(
    network_group: *mut ffi::hailo_configured_network_group,
    format_type: HailoFormatType,
    max_params_count: Option<usize>,
) -> Result<(Vec<Vec<u8>>, Vec<Vec<u8>>), String> {
    let max_params_count = max_params_count.unwrap_or(16);

    // Initialize vectors for streams and stream info
    let mut input_vstreams: Vec<*mut ffi::hailo_input_stream> = vec![std::ptr::null_mut(); max_params_count];
    let mut output_vstreams: Vec<*mut ffi::hailo_output_stream> = vec![std::ptr::null_mut(); max_params_count];
    let mut input_stream_infos: Vec<HailoStreamInfo> = Vec::with_capacity(max_params_count);
    let mut output_stream_infos: Vec<HailoStreamInfo> = Vec::with_capacity(max_params_count);

    // Query input stream info dynamically
    let input_stream_infos = get_network_input_stream_infos(network_group)?;

    let mut input_params: Vec<HailoInputVstreamParamsByName> = input_stream_infos
    .iter()
    .map(|info| {
        let mut name_buf = [0u8; 64];
        let stream_name = info.get_name();
        name_buf[..stream_name.len()].copy_from_slice(stream_name.as_bytes());

        HailoInputVstreamParamsByName {
            name: name_buf,
            params: HailoVstreamParams {
                user_buffer_format: format_type as i32,
                timeout_ms: 5000,
                queue_size: 16,
                vstream_stats_flags: 0,
                pipeline_elements_stats_flags: 0,
            },
        }
    })
    .collect();

    let mut input_count: usize = input_params.len();  // Initialize with the number of input params


    // Call the C function directly using the manual FFI types:
    let status = unsafe {
        hailors_create_input_vstreams(
            network_group as *mut c_void,
            input_params.as_mut_ptr(),
            input_params.len(),
            input_vstreams.as_mut_ptr() as *mut *mut c_void,
            &mut input_count,
        )
    };

    if HailoStatus::from_i32(status as i32) != HailoStatus::Success {
        return Err("Failed to create input vstreams from network group".to_string());
    }

    // Fetch input stream info using `fetch_stream_info`
    let output_stream_infos = get_network_output_stream_infos(network_group)?;

    let output_params: Vec<HailoOutputVstreamParamsByName> = output_stream_infos
    .iter()
    .map(|info| {
        let mut name_buf = [0u8; 64];
        let stream_name = info.get_name();
        name_buf[..stream_name.len()].copy_from_slice(stream_name.as_bytes());

        HailoOutputVstreamParamsByName {
            name: name_buf,
            params: HailoVstreamParams {
                user_buffer_format: format_type as i32,
                timeout_ms: 5000,
                queue_size: 16,
                vstream_stats_flags: 0,
                pipeline_elements_stats_flags: 0,
            },
        }
    })
    .collect();

    // Query output streams from network group
    let mut output_count: usize = output_params.len();  // Requesting to create this many output streams

    let status = unsafe {
        hailors_create_output_vstreams(
            network_group as *mut c_void,
            output_params.as_ptr(),
            output_params.len(),  // Pass the number of params you provide
            output_vstreams.as_mut_ptr() as *mut *mut c_void,
            &mut output_count,    // The function will overwrite this with the actual number created
        )
    };
    if HailoStatus::from_i32(status as i32) != HailoStatus::Success {
        return Err("Failed to create output vstreams from network group".to_string());
    }

    // Fetch output stream info using `fetch_stream_info`
    for &output_vstream in output_vstreams.iter().take(output_count) {
        match fetch_output_stream_info(output_vstream as *mut c_void) {
            Ok(info) => output_stream_infos.push(info),
            Err(e) => return Err(format!("Failed to get output stream info: {}", e)),
        }
    }

    // Create input and output buffers with appropriate sizes
    let input_buffers: Vec<Vec<u8>> = input_stream_infos
    .iter()
    .map(|info| vec![0u8; info.hw_data_bytes as usize])
    .collect();
    let output_buffers: Vec<Vec<u8>> = output_stream_infos
    .iter()
    .map(|info| vec![0u8; info.hw_data_bytes as usize])
    .collect();

    Ok((input_buffers, output_buffers))

}


pub fn run_inference(
    network_group: *mut ffi::hailo_configured_network_group,
    inference_buffers: &InferenceBuffers,
    frames_count: usize,
) -> Result<(), String> {
    let status = unsafe {
        hailors_infer(
            network_group as *mut c_void,
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
