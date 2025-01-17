#include <gtest/gtest.h>
#include "../../src/device_api_wrapper.hpp"
#include "../../src/hef_api_wrapper.hpp"
#include "../../src/network_api_wrapper.hpp"
#define STB_IMAGE_IMPLEMENTATION
#include "stb_image.h"
#include <fstream>

class HailoTestSuite : public ::testing::Test {
protected:
    static hailo_vdevice_handle vdevice_handle;

    static void SetUpTestSuite() {
        hailo_status status = hailors_create_vdevice(&vdevice_handle);
        ASSERT_EQ(status, HAILO_SUCCESS);
        ASSERT_NE(vdevice_handle, nullptr);
    }

    static void TearDownTestSuite() {
        hailo_status status = hailors_release_vdevice(vdevice_handle);
        ASSERT_EQ(status, HAILO_SUCCESS);
    }

    static bool load_test_image(const std::string& image_path, size_t expected_size, std::vector<unsigned char>& buffer) {
        // Open the file
        std::ifstream file(image_path, std::ios::binary | std::ios::ate);
        if (!file.is_open()) {
            std::cerr << "Failed to open test image file: " << image_path << std::endl;
            return false;
        }

        size_t file_size = file.tellg();
        if (file_size != expected_size) {
            std::cerr << "Image size mismatch! Expected: " << expected_size << ", but got: " << file_size << std::endl;
            return false;
        }

        buffer.resize(file_size);
        file.seekg(0, std::ios::beg);
        file.read(reinterpret_cast<char*>(buffer.data()), file_size);
        return true;
    }
};

// Initialize static member
hailo_vdevice_handle HailoTestSuite::vdevice_handle = nullptr;

TEST_F(HailoTestSuite, LoadHefMetadata) {
    const char* hef_path = "./hef/yolov8s_h8.hef";

    // Fetch network information
    hailors_network_info_t* network_infos = nullptr;
    size_t network_count = 0;
    hailo_status status = hailors_get_network_infos(hef_path, &network_infos, &network_count);
    ASSERT_EQ(status, HAILO_SUCCESS);

    ASSERT_GT(network_count, 0) << "No networks found in the HEF.";
    for (size_t i = 0; i < network_count; i++) {
        std::cout << "Network Name: " << network_infos[i].name << ", Inputs: " << network_infos[i].input_count
                  << ", Outputs: " << network_infos[i].output_count << std::endl;
    }

    hailors_free_network_infos(network_infos, network_count);
}

TEST_F(HailoTestSuite, ConfigureAndInfer) {
    const char* hef_path = "./hef/yolov8s_h8.hef";
    hailo_network_group_handle network_group_handle = nullptr;

    hailors_network_info_t* network_infos = nullptr;
    size_t network_count = 0;
    hailo_status status = hailors_get_network_infos(hef_path, &network_infos, &network_count);
    ASSERT_EQ(status, HAILO_SUCCESS);

    ASSERT_GT(network_count, 0) << "No networks found in the HEF.";
    const char* network_name = network_infos[0].name;  // Use the first network for this test

    // Configure the network group
    status = hailors_create_network_group(vdevice_handle, hef_path, &network_group_handle);
    ASSERT_EQ(status, HAILO_SUCCESS);

    hailors_stream_info_t* input_streams = nullptr;
    size_t input_count = 0;
    status = hailors_get_input_stream_infos(hef_path, network_name, &input_streams, &input_count);
    ASSERT_EQ(status, HAILO_SUCCESS);
    ASSERT_GT(input_count, 0);

    hailors_stream_info_t* output_streams = nullptr;
    size_t output_count = 0;
    status = hailors_get_output_stream_infos(hef_path, network_name, &output_streams, &output_count);
    ASSERT_EQ(status, HAILO_SUCCESS);
    ASSERT_GT(output_count, 0);

    // Allocate and load an image
    const char* image_path = "./images/dog.rgb";
    size_t input_frame_size = 640 * 640 * 3; // Assume 640x640x3 for the input frame size
    std::vector<unsigned char> input_data;
    ASSERT_TRUE(load_test_image(image_path, input_frame_size, input_data)) << "Failed to load test image";

    // Write the image to the input vstream
    hailo_input_vstream_handle input_vstream = nullptr;
    // Assuming input_vstream was obtained during configuration; otherwise, this needs to be initialized.
    status = hailors_write_input_frame(input_vstream, input_data.data(), input_data.size());
    ASSERT_EQ(status, HAILO_SUCCESS);

    // Allocate handles for output vstreams during configuration
    std::vector<void*> output_vstreams(output_count, nullptr);

    // Read and process output data
    for (size_t i = 0; i < output_count; i++) {
        // Ensure output vstream handle is obtained during configuration
        void* output_vstream_handle = output_vstreams[i];
        ASSERT_NE(output_vstream_handle, nullptr) << "Output vstream handle is null for index " << i;

        // Allocate output data buffer based on shape or assumed size
        std::vector<float> output_data(640 * 640 * sizeof(float), 0);

        status = hailors_read_output_frame(
            output_vstream_handle,            // Use the actual output vstream handle
            output_data.data(),               // Pass the output data buffer
            output_data.size() * sizeof(float) // Specify the buffer size
        );

        ASSERT_EQ(status, HAILO_SUCCESS);

        // Process output data based on stream information
        std::cout << "Processed output stream: " << output_streams[i].name << std::endl;
    }

    // Cleanup
    hailors_free_stream_infos(input_streams, input_count);
    hailors_free_stream_infos(output_streams, output_count);
    hailors_release_network_group(network_group_handle);
}


int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
