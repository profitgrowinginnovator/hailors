#ifndef DEVICE_API_WRAPPER_HPP
#define DEVICE_API_WRAPPER_HPP

#include "hailo/hailort.hpp"


extern "C" {

// Handle typedefs
typedef void* hailo_vdevice_handle;
typedef void* hailo_network_group_handle;
typedef void* hailo_input_vstream_handle;
typedef void* hailo_output_vstream_handle;

// Function declarations
hailo_status hailors_create_vdevice(hailo_vdevice_handle* vdevice);
hailo_status hailors_release_vdevice(hailo_vdevice_handle vdevice);

hailo_status hailors_configure_hef(
    hailo_vdevice_handle vdevice,
    const char* hef_path,
    hailo_network_group_handle* network_group,
    hailo_input_vstream_handle* input_vstreams,
    size_t* input_count,
    hailo_output_vstream_handle* output_vstreams,
    size_t* output_count);

hailo_status hailors_infer(
    hailo_network_group_handle network_group,
    hailo_input_vstream_handle* input_vstreams,
    size_t input_count,
    hailo_output_vstream_handle* output_vstreams,
    size_t output_count);

}

#endif // DEVICE_API_WRAPPER_HPP
