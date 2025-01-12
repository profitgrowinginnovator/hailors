#ifndef DEVICE_API_WRAPPER_HPP
#define DEVICE_API_WRAPPER_HPP

#include "hailort.h"

extern "C" {
    typedef void* hailo_device_handle; 
    typedef void* hailo_vdevice_handle;
    typedef void* hailo_network_group_handle;
    typedef void* hailo_vstream_handle;
    hailo_status hailors_open_device(const char *device_id, hailo_device_handle *device);
    hailo_status hailors_close_device(hailo_device_handle device);
    hailo_status hailors_vdevice_create(hailo_vdevice_handle *vdevice);
    hailo_status hailors_load_hef(const char *hef_path, hailo_network_group_handle *network_group, hailo_vdevice_handle optional_vdevice);
    hailo_status hailors_release_vdevice(hailo_vdevice_handle vdevice);
    hailo_status hailors_scan_devices(hailo_device_id_t *device_ids, size_t *device_count);
    hailo_status hailors_create_input_vstreams(
        hailo_network_group_handle network_group,
        hailo_input_vstream_params_by_name_t *input_params,
        size_t input_params_count,
        hailo_vstream_handle *input_vstreams,
        size_t *input_count
    );
    hailo_status hailors_create_output_vstreams(
        hailo_network_group_handle network_group,
        hailo_output_vstream_params_by_name_t *output_params,
        size_t output_params_count,
        hailo_vstream_handle *output_vstreams,
        size_t *output_count
    );
    hailo_status hailors_infer(
        hailo_network_group_handle network_group,
        hailo_input_vstream_params_by_name_t *input_params,
        hailo_stream_raw_buffer_by_name_t *input_buffers,
        size_t inputs_count,
        hailo_output_vstream_params_by_name_t *output_params,
        hailo_stream_raw_buffer_by_name_t *output_buffers,
        size_t outputs_count,
        size_t frames_count
    );
}

#endif // DEVICE_API_WRAPPER_HPP
