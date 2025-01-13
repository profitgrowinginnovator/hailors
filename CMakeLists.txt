cmake_minimum_required(VERSION 3.10)
project(hailors)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_FLAGS "-Wall -Werror")

add_library(hailo_lib STATIC src/device_api_wrapper.cpp)

# HailoRT include and library paths
find_library(HAILO_LIB hailort PATHS /usr/local/lib /usr/lib REQUIRED)
include_directories("/usr/include/hailo")

target_link_libraries(hailo_lib PRIVATE ${HAILO_LIB})

add_executable(test_device_api_wrapper test/test_device_api_wrapper.cpp)
find_package(GTest REQUIRED)

target_link_libraries(test_device_api_wrapper PRIVATE hailo_lib GTest::GTest GTest::Main pthread)
