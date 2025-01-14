#include "custom_wrapper.hpp"

hailo_status create_input_vstream(ConfiguredNetworkGroup *group, InputVStream **vstream) {
    auto result = group->make_input_vstream_params(false, HAILO_FORMAT_TYPE_AUTO, HAILO_DEFAULT_VSTREAM_TIMEOUT_MS, HAILO_DEFAULT_VSTREAM_QUEUE_SIZE, "");
    if (!result) {
        return result.status();
    }

    auto vstream_result = hailort::VStreamsBuilder::create_input_vstreams(*group, result.value());
    if (!vstream_result) {
        return vstream_result.status();
    }

    *vstream = new InputVStream(std::move(vstream_result.value().front()));
    return HAILO_SUCCESS;
}

hailo_status release_input_vstream(InputVStream *vstream) {
    if (!vstream) {
        return HAILO_INVALID_ARGUMENT;
    }
    delete vstream;
    return HAILO_SUCCESS;
}

hailo_status create_output_vstream(ConfiguredNetworkGroup *group, OutputVStream **vstream) {
    auto result = group->make_output_vstream_params(false, HAILO_FORMAT_TYPE_AUTO, HAILO_DEFAULT_VSTREAM_TIMEOUT_MS, HAILO_DEFAULT_VSTREAM_QUEUE_SIZE, "");
    if (!result) {
        return result.status();
    }

    auto vstream_result = hailort::VStreamsBuilder::create_output_vstreams(*group, result.value());
    if (!vstream_result) {
        return vstream_result.status();
    }

    *vstream = new OutputVStream(std::move(vstream_result.value().front()));
    return HAILO_SUCCESS;
}

hailo_status release_output_vstream(OutputVStream *vstream) {
    if (!vstream) {
        return HAILO_INVALID_ARGUMENT;
    }
    delete vstream;
    return HAILO_SUCCESS;
}
