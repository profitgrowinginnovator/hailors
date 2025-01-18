#include "network_api_wrapper.hpp"
#include "device_api_wrapper.hpp"
#include "hailo/hailort.hpp"
#include <vector>
#include <thread>
#include <iostream>
#include <cstring>

using namespace hailort;

extern "C" hailo_status hailors_create_network_group(hailo_vdevice_handle vdevice, const char *hef_path, hailo_network_group_handle *network_group)
{
    if (!vdevice || !hef_path || !network_group) {
        std::cerr << "Invalid arguments passed to hailors_create_network_group." << std::endl;
        return HAILO_INVALID_ARGUMENT;
    }

    auto vdevice_ptr = static_cast<VDevice *>(vdevice);
    auto hef_result = Hef::create(hef_path);
    if (!hef_result) {
        std::cerr << "Failed to create HEF: " << hef_result.status() << std::endl;
        return hef_result.status();
    }
    auto hef = std::move(hef_result.value());

    auto configure_params = vdevice_ptr->create_configure_params(hef);
    if (!configure_params) {
        std::cerr << "Failed to create configure params: " << configure_params.status() << std::endl;
        return configure_params.status();
    }

    auto network_groups_result = vdevice_ptr->configure(hef, configure_params.value());
    if (!network_groups_result || network_groups_result->empty()) {
        std::cerr << "Failed to configure network groups: " << network_groups_result.status() << std::endl;
        return network_groups_result.status();
    }

    *network_group = network_groups_result.value()[0].get();
    return HAILO_SUCCESS;
}

extern "C" hailo_status hailors_release_network_group(hailo_network_group_handle network_group)
{
    delete static_cast<ConfiguredNetworkGroup *>(network_group);
    return HAILO_SUCCESS;
}

extern "C" hailo_status hailors_release_input_vstream(hailo_input_vstream_handle input_vstream)
{
    if (!input_vstream) {
        std::cerr << "Invalid input_vstream handle provided to hailors_release_input_vstream." << std::endl;
        return HAILO_INVALID_ARGUMENT;
    }

    // Cast the handle to the InputVStream and delete it
    delete static_cast<hailort::InputVStream*>(input_vstream);
}

extern "C" hailo_status hailors_write_input_frame(
    hailo_input_vstream_handle input_vstream,
    const void* data,
    size_t data_size
) {
    if (!input_vstream || !data) {
        std::cerr << "Invalid input stream handle or data buffer." << std::endl;
        return HAILO_INVALID_ARGUMENT;
    }

    // Cast the handle to InputVStream
    auto vstream = static_cast<hailort::InputVStream*>(input_vstream);

    // Create a MemoryView for the input data (casting to `void*`)
    hailort::MemoryView input_view(const_cast<void*>(data), data_size);

    // Write data using the MemoryView
    auto status = vstream->write(input_view);
    if (status != HAILO_SUCCESS) {
        std::cerr << "Failed to write data to input vstream. Status: " << status << std::endl;
    }

    return status;
}

/**
 * @brief Releases a previously created output vstream.
 *
 * This function deallocates resources associated with the given output vstream.
 *
 * @param[in] output_vstream The output vstream handle to release.
 * @return HAILO_SUCCESS on success, or an appropriate error code on failure.
 */
extern "C" hailo_status hailors_release_output_vstream(void* output_vstream)
{
    if (!output_vstream) {
        std::cerr << "Invalid output_vstream handle provided to hailors_release_output_vstream." << std::endl;
        return HAILO_INVALID_ARGUMENT;
    }

    // Cast the handle to the OutputVStream and delete it
    delete static_cast<hailort::OutputVStream*>(output_vstream);

    return HAILO_SUCCESS;
}

extern "C" hailo_status hailors_read_output_frame(
    void* output_vstream, 
    void* buffer,
    size_t buffer_size
) {
    if (!output_vstream) {
        std::cerr << "output_vstream is null." << std::endl;
        return HAILO_INVALID_ARGUMENT;
    }

    if (!buffer || buffer_size == 0) {
        std::cerr << "Buffer is null or has invalid size." << std::endl;
        return HAILO_INVALID_ARGUMENT;
    }


        // Cast the handle to OutputVStream
    auto vstream = static_cast<hailort::OutputVStream*>(output_vstream);

    // Create a MemoryView for the output buffer
    hailort::MemoryView output_view(buffer, buffer_size);
    if (!vstream) {
        std::cerr << "Invalid output_vstream (null pointer)." << std::endl;
        return HAILO_INVALID_ARGUMENT;
    }

    // Read data using the MemoryView
    auto status = vstream->read(output_view);


    if (status != HAILO_SUCCESS) {
        std::cerr << "Failed to read data from output vstream. Status: " << status << std::endl;
    }

    return status;
}