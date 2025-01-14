cmake_minimum_required(VERSION 3.14)
project(hailors)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Include the Hailo header files
include_directories("/usr/include/hailo")
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/src)

# Add the source files
add_library(hailors SHARED 
    src/device_api_wrapper.cpp
    src/custom_wrapper.cpp  # Add this line for your new wrapper
)

# Link against the required Hailo runtime library
target_link_libraries(hailors hailort)

# Include directory for Rust's FFI generation
target_include_directories(hailors PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/src)

# Ensure that the output is placed correctly for Rust bindings
set_target_properties(hailors PROPERTIES
    LIBRARY_OUTPUT_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}/../"
)

