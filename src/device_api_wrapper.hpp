#ifndef DEVICE_API_WRAPPER_HPP
#define DEVICE_API_WRAPPER_HPP

#include "hailo/hailort.hpp"


extern "C" {



// Handle typedefs
typedef void* hailo_vdevice_handle;

// Function declarations
hailo_status hailors_create_vdevice(hailo_vdevice_handle* vdevice);
hailo_status hailors_release_vdevice(hailo_vdevice_handle vdevice);
}
#endif // DEVICE_API_WRAPPER_HPP
