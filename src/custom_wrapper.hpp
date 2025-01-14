#ifndef CUSTOM_WRAPPER_HPP
#define CUSTOM_WRAPPER_HPP

#include "hailo/hailort.hpp"  // Ensure you include the correct header for Hailo C++ API

using hailort::ConfiguredNetworkGroup;
using hailort::InputVStream;
using hailort::OutputVStream;

extern "C" {
    hailo_status create_input_vstream(ConfiguredNetworkGroup *group, InputVStream **vstream);
    hailo_status release_input_vstream(InputVStream *vstream);
    hailo_status create_output_vstream(ConfiguredNetworkGroup *group, OutputVStream **vstream);
    hailo_status release_output_vstream(OutputVStream *vstream);
}

#endif  // CUSTOM_WRAPPER_HPP
