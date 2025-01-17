#ifndef NETWORK_API_WRAPPER_HPP
#define NETWORK_API_WRAPPER_HPP

#include "hailo/hailort.hpp"

extern "C" {

// Handle typedefs
typedef void* hailo_vdevice_handle;
typedef void* hailo_network_group_handle;
typedef void* hailo_input_vstream_handle;

extern "C" hailo_status hailors_create_network_group(hailo_vdevice_handle vdevice, const char *hef_path, hailo_network_group_handle *network_group);
extern "C" hailo_status hailors_release_network_group(hailo_network_group_handle network_group);
extern "C" hailo_status hailors_release_input_vstream(hailo_input_vstream_handle input_vstream);
extern "C" hailo_status hailors_write_input_frame(
    hailo_input_vstream_handle input_vstream,
    const void* data,
    size_t data_size
);
extern "C" hailo_status hailors_release_output_vstream(void* output_vstream);
extern "C" hailo_status hailors_read_output_frame(
    void* output_vstream, 
    void* buffer,
    size_t buffer_size
);
}
#endif // NETWORK_API_WRAPPER_HPP
