#include <gtest/gtest.h>
#include <vector>
#include <fstream>
#include <cstring>
#include "../../src/device_api_wrapper.hpp"

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
            return false;  // Return false on failure
        }

        size_t file_size = file.tellg();
        if (file_size != expected_size) {
            std::cerr << "Image size mismatch! Expected: " << expected_size << ", but got: " << file_size << std::endl;
            return false;  // Return false on failure
        }

        buffer.resize(file_size);
        file.seekg(0, std::ios::beg);
        file.read(reinterpret_cast<char*>(buffer.data()), file_size);
        return true;  // Return true on success
    }


};

// Initialize static member
hailo_vdevice_handle HailoTestSuite::vdevice_handle = nullptr;

TEST_F(HailoTestSuite, ConfigureNetworkGroup) {
    const char* hef_path = "./hef/yolov8s_h8.hef";
    hailo_network_group_handle network_group_handle = nullptr;
    hailo_input_vstream_handle input_vstreams = nullptr;
    hailo_output_vstream_handle output_vstreams = nullptr;
    size_t input_count = 16;
    size_t output_count = 16;
    size_t input_frame_size = 0;
    size_t output_frame_size = 0;

    hailo_status status = hailors_configure_hef(
        vdevice_handle, hef_path, &network_group_handle,
        &input_vstreams, &input_count, &output_vstreams, &output_count,
        &input_frame_size, &output_frame_size
    );
    ASSERT_EQ(status, HAILO_SUCCESS);
    ASSERT_GT(input_frame_size, 0) << "Input frame size should be greater than 0.";
    ASSERT_GT(output_frame_size, 0) << "Output frame size should be greater than 0.";
}

TEST_F(HailoTestSuite, PerformInference) {
    const char* hef_path = "./hef/yolov8s_h8.hef";
    const char* image_path = "./images/dog.rgb";  // Path to the dog.rgb image
    hailo_network_group_handle network_group_handle = nullptr;
    hailo_input_vstream_handle input_vstreams = nullptr;
    hailo_output_vstream_handle output_vstreams = nullptr;
    size_t input_count = 16;
    size_t output_count = 16;
    size_t input_frame_size = 0;
    size_t output_frame_size = 0;

    hailo_status status = hailors_configure_hef(
        vdevice_handle, hef_path, &network_group_handle,
        &input_vstreams, &input_count, &output_vstreams, &output_count,
        &input_frame_size, &output_frame_size
    );
    ASSERT_EQ(status, HAILO_SUCCESS);

    // Declare a buffer to store the image data
    std::vector<unsigned char> input_data;
/*
    // Load the image into the buffer
    ASSERT_TRUE(load_test_image(image_path, input_frame_size, input_data)) << "Failed to load test image";

    // Perform inference
    status = hailors_write_input_frame(input_vstreams[0], input_data.data(), input_data.size());
    ASSERT_EQ(status, HAILO_SUCCESS);

    // Prepare a buffer to store the output detections
    std::vector<float> output_data(output_frame_size / sizeof(float));
    status = hailors_read_output_frame(output_vstreams[0], reinterpret_cast<void*>(output_data.data()), output_frame_size);
    ASSERT_EQ(status, HAILO_SUCCESS);

    // Assume that the dog class ID is 16 in the model (standard for COCO dataset)
    const int DOG_CLASS_ID = 16;
    bool detected_dog = false;

    // Parse the output and check if any detection is a dog
    for (size_t i = 0; i < output_data.size(); i += 6) {  // Each detection is 6 floats: class_id, confidence, x_min, y_min, x_max, y_max
        int class_id = static_cast<int>(output_data[i]);      // Class ID
        float confidence = output_data[i + 1];                // Confidence
        if (class_id == DOG_CLASS_ID && confidence > 0.5f) {  // Check for dog class and high confidence
            detected_dog = true;
            break;
        }
    }

    ASSERT_TRUE(detected_dog) << "Dog not detected in the image.";
   */ 
}


int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
