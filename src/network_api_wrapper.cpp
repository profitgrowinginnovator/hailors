#include "hailo/hef.hpp"
#include "hef_api_wrapper.hpp"

#include <cstring>
#include <vector>
#include <string>

extern "C" {

hailo_status hailors_initialize_hef(const char* hef_path, hailort::Hef** hef_out,
                                    hailors_network_info_t** network_infos, size_t* network_count,
                                    char*** stream_names, size_t* stream_count) {
    if (!hef_out || !network_infos || !network_count || !stream_names || !stream_count) {
        return HAILO_INVALID_ARGUMENT;
    }

    // Create the Hef object
    auto hef_result = hailort::Hef::create(hef_path);
    if (!hef_result) {
        return hef_result.status();
    }

    *hef_out = new hailort::Hef(std::move(hef_result.value()));

    // Get network info
    auto network_groups_result = (*hef_out)->get_network_infos();
    if (!network_groups_result) {
        delete *hef_out;
        *hef_out = nullptr;
        return network_groups_result.status();
    }

    const auto& network_groups = network_groups_result.value();
    *network_count = network_groups.size();

    // Allocate memory for network info
    *network_infos = static_cast<hailors_network_info_t*>(malloc(network_groups.size() * sizeof(hailors_network_info_t)));
    if (!*network_infos) {
        delete *hef_out;
        *hef_out = nullptr;
        return HAILO_OUT_OF_HOST_MEMORY;
    }

    for (size_t i = 0; i < network_groups.size(); i++) {
        strncpy((*network_infos)[i].name, network_groups[i].name, HAILO_MAX_NETWORK_NAME_SIZE);
        (*network_infos)[i].name[HAILO_MAX_NETWORK_NAME_SIZE - 1] = '\0'; // Null-terminate
    }

    // Get input stream names for the first network
    if (network_groups.empty()) {
        free(*network_infos);
        *network_infos = nullptr;
        *network_count = 0;
        delete *hef_out;
        *hef_out = nullptr;
        return HAILO_INVALID_ARGUMENT;
    }

    auto input_streams_result = (*hef_out)->get_input_vstream_infos(network_groups[0].name);
    if (!input_streams_result) {
        free(*network_infos);
        *network_infos = nullptr;
        *network_count = 0;
        delete *hef_out;
        *hef_out = nullptr;
        return input_streams_result.status();
    }

    const auto& input_streams = input_streams_result.value();
    *stream_count = input_streams.size();

    // Allocate memory for stream names
    *stream_names = static_cast<char**>(malloc(input_streams.size() * sizeof(char*)));
    if (!*stream_names) {
        free(*network_infos);
        *network_infos = nullptr;
        *network_count = 0;
        delete *hef_out;
        *hef_out = nullptr;
        return HAILO_OUT_OF_HOST_MEMORY;
    }

    for (size_t i = 0; i < input_streams.size(); i++) {
        (*stream_names)[i] = strdup(input_streams[i].name);
        if (!(*stream_names)[i]) {
            for (size_t j = 0; j < i; j++) {
                free((*stream_names)[j]);
            }
            free(*stream_names);
            *stream_names = nullptr;
            free(*network_infos);
            *network_infos = nullptr;
            *network_count = 0;
            delete *hef_out;
            *hef_out = nullptr;
            return HAILO_OUT_OF_HOST_MEMORY;
        }
    }

    return HAILO_SUCCESS;
}

// Cleanup function to release all resources
void hailors_cleanup(hailort::Hef* hef, hailors_network_info_t* network_infos, size_t network_count,
                     char** stream_names, size_t stream_count) {
    if (hef) {
        delete hef;
    }
    if (network_infos) {
        free(network_infos);
    }
    if (stream_names) {
        for (size_t i = 0; i < stream_count; i++) {
            free(stream_names[i]);
        }
        free(stream_names);
    }
}

}
