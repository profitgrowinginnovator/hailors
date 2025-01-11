#include "device_api_wrapper.hpp"
#include "hailort.h"
#include <cstring>
#include <iomanip>

using namespace hailort;

#define MAX_DEVICES 32

extern "C" hailo_status hailors_infer(
    hailo_network_group_handle network_group,
    hailo_input_vstream_params_by_name_t *input_params,
    hailo_stream_raw_buffer_by_name_t *input_buffers,
    size_t inputs_count,
    hailo_output_vstream_params_by_name_t *output_params,
    hailo_stream_raw_buffer_by_name_t *output_buffers,
    size_t outputs_count,
    size_t frames_count)
{
    auto *configured_group = reinterpret_cast<hailo_configured_network_group *>(network_group);
    if (!configured_group) {
        return HAILO_INVALID_ARGUMENT;  // Handle null pointer case
    }

    // Call the Hailo `hailo_infer` function
    hailo_status status = hailo_infer(
        *configured_group,
        input_params, input_buffers, inputs_count,
        output_params, output_buffers, outputs_count,
        frames_count);

    return status;  // Return the status
}

extern "C" hailo_status hailors_vdevice_create(hailo_vdevice_handle *vdevice) {
    auto vdev = VDevice::create();
    if (!vdev) return vdev.status();
    *vdevice = vdev.value().release();  // Release ownership of the device
    return HAILO_SUCCESS;
}

extern "C" hailo_status hailors_load_hef(const char *hef_path, hailo_network_group_handle *network_group) {
    auto hef = Hef::create(hef_path);
    if (!hef) {
        return hef.status();  // Return the error status if HEF creation fails
    }

    auto vdevice = VDevice::create();
    if (!vdevice) {
        return vdevice.status();  // Return the error status if VDevice creation fails
    }

    auto network_groups = vdevice.value()->configure(hef.value());
    if (!network_groups) {
        return network_groups.status();  // Return the error status if network group configuration fails
    }

    if (network_groups->empty()) {
        return HAILO_NOT_FOUND;  // Handle the case where no network groups are returned
    }

    // Pass the raw pointer to the single-level `network_group`
    *network_group = network_groups->at(0).get();  // Obtain the raw pointer from shared_ptr
    return HAILO_SUCCESS;
}

extern "C" hailo_status hailors_create_vstreams(
hailo_network_group_handle network_group,
    hailo_input_vstream_params_by_name_t *input_params,
    size_t input_params_count,
    hailo_output_vstream_params_by_name_t *output_params,
    size_t output_params_count,
    hailo_vstream_handle *input_vstreams,
    size_t *input_count,
    hailo_vstream_handle *output_vstreams,
    size_t *output_count)
{
    auto *configured_group = reinterpret_cast<hailo_configured_network_group*>(network_group);
    if (!configured_group) {
        return HAILO_INVALID_ARGUMENT;  // Handle null pointer case
    }

    hailo_status status = hailo_create_input_vstreams(
        *configured_group,
        input_params,
        input_params_count,
        reinterpret_cast<hailo_input_vstream*>(input_vstreams));
    if (HAILO_SUCCESS != status) {
        return status;
    }

    status = hailo_create_output_vstreams(
        *configured_group,
        output_params,
        output_params_count,
        reinterpret_cast<hailo_output_vstream*>(output_vstreams));
    if (HAILO_SUCCESS != status) {
        hailo_release_input_vstreams(reinterpret_cast<hailo_input_vstream*>(input_vstreams), input_params_count);
        return status;
    }

    *input_count = input_params_count;
    *output_count = output_params_count;

    return HAILO_SUCCESS;
}

extern "C" hailo_status hailors_close_device(hailo_device_handle device) {
    delete static_cast<Device*>(device);  // Safely delete
    return HAILO_SUCCESS;
}

extern "C" hailo_status hailors_scan_devices(hailo_device_id_t *device_ids, size_t *device_count) {
    if (!device_ids || !device_count) {
        return HAILO_INVALID_ARGUMENT;  // Handle null pointers
    }

    // Call the Hailo API function directly with the default parameters
    hailo_status status = hailo_scan_devices(nullptr, device_ids, device_count);
    if (status != HAILO_SUCCESS) {
        *device_count = 0;  // No devices found or an error occurred
        return status;
    }

    return HAILO_SUCCESS;  // Successfully scanned devices
}




extern "C" hailo_status hailors_release_vdevice(hailo_vdevice_handle vdevice) {
    if (!vdevice) {
        return HAILO_INVALID_ARGUMENT;  // Handle null pointer case
    }
    delete static_cast<VDevice *>(vdevice);  // Release the VDevice instance
    return HAILO_SUCCESS;
}

extern "C" hailo_status hailors_open_device(const char *device_id, hailo_device_handle *device) {
    if (!device_id || !device) {
        return HAILO_INVALID_ARGUMENT;  // Handle null pointers
    }

    auto device_result = Device::create(device_id);
    if (!device_result) {
        return device_result.status();  // Return error status if creation fails
    }

    *device = device_result.value().release();  // Release ownership to the caller
    return HAILO_SUCCESS;
}
