cmake_minimum_required(VERSION 3.14)
project(hailors)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Ensure the build type is Debug (if not specified, default to Debug)
if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Debug CACHE STRING "Build type" FORCE)
endif()

# Set debug flags (e.g., -g for debugging symbols and -O0 to disable optimizations)
set(CMAKE_CXX_FLAGS_DEBUG "-g -O0")

# Include the Hailo header files
include_directories("/usr/include/hailo")
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/src)

# Add the source files for the shared library
add_library(hailors SHARED 
    src/device_api_wrapper.cpp
)

# Link against the required Hailo runtime library
target_link_libraries(hailors hailort)

# Include directory for Rust's FFI generation
target_include_directories(hailors PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/src)

# Ensure that the output is placed correctly for Rust bindings
set_target_properties(hailors PROPERTIES
    LIBRARY_OUTPUT_DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}/../"
)

# ======================
# Google Test Integration
# ======================
# Download Google Test if not installed
include(FetchContent)
FetchContent_Declare(
  googletest
  URL https://github.com/google/googletest/archive/refs/tags/release-1.12.1.zip
  DOWNLOAD_EXTRACT_TIMESTAMP TRUE  # Add this line to avoid the warning
)

FetchContent_MakeAvailable(googletest)

# Enable testing
enable_testing()

# Add the test executable
add_executable(hailors_test
    tests/c++/test_device_api_wrapper.cpp
)

# Link Google Test libraries
target_link_libraries(hailors_test
    gtest
    gtest_main
    hailors  # Link against the `hailors` shared library
    pthread
)

# Add a test command
add_test(NAME HailoDeviceTest COMMAND hailors_test)

# Optional: Add the Debug-specific settings if you're debugging with Visual Studio Code or IDE
# You can add more configurations or customize the build type further here if needed.
