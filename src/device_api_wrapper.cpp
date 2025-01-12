#include "device_api_wrapper.hpp"
#include "hailort.h"
#include "hailort.hpp"  

#include <cstring>
#include <memory>
#include <iostream>

extern "C" hailo_status hailors_open_device(const char *device_id, hailo_device_handle *device) {
    if (!device_id || !device) {
        return HAILO_INVALID_ARGUMENT;
    }

    auto device_result = hailort::Device::create(device_id);
    if (!device_result) {
        return device_result.status();
    }

    *device = device_result.value().release();
    return HAILO_SUCCESS;
}

extern "C" hailo_status hailors_close_device(hailo_device_handle device) {
    if (!device) {
        return HAILO_INVALID_ARGUMENT;
    }

    delete static_cast<hailort::Device *>(device);
    return HAILO_SUCCESS;
}

extern "C" hailo_status hailors_vdevice_create(hailo_vdevice_handle *vdevice) {
    if (!vdevice) {
        return HAILO_INVALID_ARGUMENT;
    }

    auto vdevice_result = hailort::VDevice::create();
    if (!vdevice_result) {
        return vdevice_result.status();
    }

    *vdevice = vdevice_result.value().release();
    return HAILO_SUCCESS;
}

extern "C" hailo_status get_shape(const hailo_stream_info_t *info, hailo_3d_image_shape_t *out_shape) {
    if (!info || !out_shape) {
        return HAILO_INVALID_ARGUMENT;
    }

    if (info->direction == HAILO_H2D_STREAM) { // Host-to-Device, assuming relevant for shape
        *out_shape = info->shape;
        return HAILO_SUCCESS;
    }

    return HAILO_INVALID_OPERATION; // No shape for non-image streams
}

extern "C" hailo_status get_input_stream_info(hailo_input_stream *stream, hailo_stream_info_t *info) {
    if (!stream || !info) {
        return HAILO_INVALID_ARGUMENT;
    }
    return hailo_get_input_stream_info(*stream, info);
}

extern "C" hailo_status get_output_stream_info(hailo_output_stream *stream, hailo_stream_info_t *info) {
    if (!stream || !info) {
        return HAILO_INVALID_ARGUMENT;
    }
    return hailo_get_output_stream_info(*stream, info);
}

extern "C" const char* get_stream_name(const hailo_stream_info_t *info) {
    if (!info) {
        return nullptr;
    }
    return info->name;
}

extern "C" hailo_status hailors_load_hef(const char *hef_path, hailo_network_group_handle *network_group, hailo_vdevice_handle optional_vdevice) {
    if (!hef_path || !network_group) {
        return HAILO_INVALID_ARGUMENT;
    }

    auto hef_result = hailort::Hef::create(hef_path);
    if (!hef_result) {
        return hef_result.status();
    }

    std::shared_ptr<hailort::VDevice> vdevice;
    if (optional_vdevice != nullptr) {
        vdevice = std::shared_ptr<hailort::VDevice>(reinterpret_cast<hailort::VDevice *>(optional_vdevice));
    } else {
        auto vdevice_result = hailort::VDevice::create();
        if (!vdevice_result) {
            return vdevice_result.status();
        }
        vdevice = std::shared_ptr<hailort::VDevice>(std::move(vdevice_result.value()));
    }

    auto network_groups_result = vdevice->configure(hef_result.value());
    if (!network_groups_result) {
        return network_groups_result.status();
    }

    if (network_groups_result->empty()) {
        return HAILO_NOT_FOUND;
    }

    *network_group = network_groups_result->at(0).get();
    return HAILO_SUCCESS;
}

extern "C" hailo_status hailors_release_vdevice(hailo_vdevice_handle vdevice) {
    if (!vdevice) {
        return HAILO_INVALID_ARGUMENT;
    }

    delete static_cast<hailort::VDevice *>(vdevice);
    return HAILO_SUCCESS;
}

extern "C" hailo_status hailors_scan_devices(hailo_device_id_t *device_ids, size_t *device_count) {
    if (!device_ids || !device_count) {
        return HAILO_INVALID_ARGUMENT;
    }

    hailo_status status = hailo_scan_devices(nullptr, device_ids, device_count);
    if (status != HAILO_SUCCESS) {
        *device_count = 0;
        return status;
    }

    return HAILO_SUCCESS;
}

extern "C" hailo_status hailors_create_input_vstreams(
    hailo_network_group_handle network_group,
    hailo_input_vstream_params_by_name_t *input_params,
    size_t input_params_count,
    hailo_vstream_handle *input_vstreams,
    size_t *input_count
) {
    if (!network_group || !input_params || !input_vstreams || !input_count) {
        return HAILO_INVALID_ARGUMENT;
    }

    hailo_status status = hailo_create_input_vstreams(
        *static_cast<hailo_configured_network_group *>(network_group),
        input_params,
        input_params_count,
        *reinterpret_cast<hailo_input_vstream **>(input_vstreams)
    );

    return status;
}

extern "C" hailo_status hailors_create_output_vstreams(
    hailo_network_group_handle network_group,
    hailo_output_vstream_params_by_name_t *output_params,
    size_t output_params_count,
    hailo_vstream_handle *output_vstreams,
    size_t *output_count
) {
    if (!network_group || !output_params || !output_vstreams || !output_count) {
        return HAILO_INVALID_ARGUMENT;
    }

    hailo_status status = hailo_create_output_vstreams(
        *static_cast<hailo_configured_network_group *>(network_group),
        output_params,
        output_params_count,
        *reinterpret_cast<hailo_output_vstream **>(output_vstreams)
    );

    return status;
}

extern "C" hailo_status hailors_infer(
    hailo_network_group_handle network_group,
    hailo_input_vstream_params_by_name_t *input_params,
    hailo_stream_raw_buffer_by_name_t *input_buffers,
    size_t inputs_count,
    hailo_output_vstream_params_by_name_t *output_params,
    hailo_stream_raw_buffer_by_name_t *output_buffers,
    size_t outputs_count,
    size_t frames_count
) {
    if (!network_group) {
        return HAILO_INVALID_ARGUMENT;
    }

    hailo_status status = hailo_infer(
        *static_cast<hailo_configured_network_group *>(network_group),
        input_params,
        input_buffers,
        inputs_count,
        output_params,
        output_buffers,
        outputs_count,
        frames_count
    );

    return status;
}
