#include "device_api_wrapper.hpp"
#include <vector>
#include <thread>
#include <iostream>
#include <cstring>

using namespace hailort;

extern "C" hailo_status hailors_create_vdevice(hailo_vdevice_handle* vdevice) {
    auto vdevice_result = VDevice::create();
    if (!vdevice_result) {
        return vdevice_result.status();
    }
    *vdevice = vdevice_result.value().release();  // Store as raw pointer
    return HAILO_SUCCESS;
}

extern "C" hailo_status hailors_release_vdevice(hailo_vdevice_handle vdevice) {
    delete static_cast<VDevice*>(vdevice);
    return HAILO_SUCCESS;
}
