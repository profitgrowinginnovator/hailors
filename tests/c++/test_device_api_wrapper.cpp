#include <gtest/gtest.h>
#include "../../src/device_api_wrapper.hpp"
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


struct Detection {
    float x1;        // Top-left X coordinate
    float y1;        // Top-left Y coordinate
    float x2;        // Bottom-right X coordinate
    float y2;        // Bottom-right Y coordinate
    float confidence;
    int class_id;
};


// Function to parse detections
std::vector<Detection> parse_detections(const std::vector<float> &output_data, float threshold = 0.5f) {
    std::vector<Detection> parsed_detections;

    size_t index = 0;
    size_t num_classes = 80; // number of classes
    size_t max_bboxes_per_class = 100; // max bboxes per class

    for (size_t class_id = 0; class_id < num_classes; class_id++) {
        size_t bbox_count = static_cast<size_t>(output_data[index++]); // Number of bboxes for this class

        // Iterate over bounding boxes for the current class
        for (size_t i = 0; i < bbox_count; i++) {
            float x1 = output_data[index++];
            float y1 = output_data[index++];
            float x2 = output_data[index++];
            float y2 = output_data[index++];
            float confidence = output_data[index++];

            // Filter detections based on confidence threshold
            if (confidence >= threshold) {
                Detection detection = {x1, y1, x2, y2, confidence, static_cast<int>(class_id)};
                parsed_detections.push_back(detection);
            }
        }
    }

    return parsed_detections;
}


void normalize_and_resize(const std::string& input_filename, std::vector<float>& output_data, int width, int height, size_t& input_frame_size) {
    // Load image using stb_image (or another image loading library)
    int img_width, img_height, channels;
    unsigned char* img_data = stbi_load(input_filename.c_str(), &img_width, &img_height, &channels, STBI_rgb);
    if (!img_data) {
        std::cerr << "Failed to load image." << std::endl;
        return;
    }

    // Resize logic here (e.g., using OpenCV or a custom resizing function)
    // You may need to implement the resize manually or use OpenCV for resizing
    // Resize the image to match the expected input size (640x640)
    
    output_data.resize(width * height * 3);  // 3 channels for RGB
    
    for (size_t i = 0; i < output_data.size(); i++) {
        output_data[i] = img_data[i] / 255.0f;  // Normalize to [0, 1]
    }

    // Update input frame size (width * height * channels)
    input_frame_size = width * height * 3;  // 3 channels for RGB

    stbi_image_free(img_data);  // Free image data after use
}



void normalize_rgb_to_nrgb(const std::string& input_filename, const std::string& output_filename, int width, int height) {
    // Open the input .rgb file
    std::ifstream input_file(input_filename, std::ios::binary);
    if (!input_file) {
        std::cerr << "Failed to open input file: " << input_filename << std::endl;
        return;
    }

    // The RGB file has width * height * 3 bytes (RGB channels)
    size_t image_size = width * height * 3;

    // Read the raw RGB data into a buffer
    std::vector<unsigned char> rgb_data(image_size);
    input_file.read(reinterpret_cast<char*>(rgb_data.data()), image_size);

    if (input_file.gcount() != image_size) {
        std::cerr << "Error reading the image data from file." << std::endl;
        return;
    }

    // Normalize the image data (scale each pixel value to [0, 1])
    std::vector<float> nrgb_data(image_size);
    for (size_t i = 0; i < rgb_data.size(); ++i) {
        nrgb_data[i] = rgb_data[i] / 255.0f;  // Normalize each byte to [0, 1]
    }

    // Open the output .nrgb file
    std::ofstream output_file(output_filename, std::ios::binary);
    if (!output_file) {
        std::cerr << "Failed to open output file: " << output_filename << std::endl;
        return;
    }

    // Write the normalized data to the file
    output_file.write(reinterpret_cast<const char*>(nrgb_data.data()), nrgb_data.size() * sizeof(float));

    if (!output_file) {
        std::cerr << "Error writing normalized data to file." << std::endl;
        return;
    }

    std::cout << "Successfully normalized and wrote image to " << output_filename << std::endl;
}

void write_output_to_file(const std::vector<float>& output_data, const std::string& filename, size_t per_line) {
    // Open the output file in text mode for CSV format
    std::ofstream output_file(filename, std::ios::out);
    if (!output_file) {
        std::cerr << "Failed to open file for writing: " << filename << std::endl;
        return;
    }

    // Write the output data to the CSV file
    size_t count = 0;
    for (const auto& value : output_data) {
        // Write the value to the file
        output_file << value;

        // Increment the count
        count++;

        // Add a comma if it's not the last value in the row
        if (count % per_line != 0 && count != output_data.size()) {
            output_file << ",";
        }
        // If we've written `per_line` values, go to the next line
        else if (count % per_line == 0) {
            output_file << "\n";
        }
    }

    // If the last line isn't completely filled, ensure we don't add an extra newline
    if (count % per_line != 0) {
        output_file << "\n";
    }

    std::cout << "Successfully wrote " << output_data.size() << " values to " << filename << std::endl;
}

TEST_F(HailoTestSuite, CreateAndReleaseVDevice) {
    // This test just checks the vdevice created in SetUpTestSuite
    ASSERT_NE(vdevice_handle, nullptr);
}

TEST_F(HailoTestSuite, ConfigureNetworkGroup) {
    const char* hef_path = "./hef/yolov8s_h8.hef";
    hailo_network_group_handle network_group_handle = nullptr;

    // Change to use smart pointers (unique_ptr)
    void **input_vstreams = nullptr; // Pointer to an array of input vstreams
    void **output_vstreams = nullptr; // Pointer to an array of output vstreams
    size_t input_count = 0; // Number of input vstreams
    size_t output_count = 0; // Number of output vstreams
    size_t input_frame_size = 0;
    size_t output_frame_size = 0;
    hailo_status status = hailors_configure_hef(
        vdevice_handle,
        hef_path,
        &network_group_handle,
        &input_vstreams,   // Pass pointer to the input vstreams
        &input_count,      // Pass pointer to the input count
        &output_vstreams,  // Pass pointer to the output vstreams
        &output_count,     // Pass pointer to the output count
        &input_frame_size,
        &output_frame_size
    );
    ASSERT_EQ(status, HAILO_SUCCESS);
    ASSERT_GT(input_frame_size, 0) << "Input frame size should be greater than 0.";
    ASSERT_GT(output_frame_size, 0) << "Output frame size should be greater than 0.";
}

TEST_F(HailoTestSuite, PerformInference) {
    const char* hef_path = "./hef/yolov8s_h8.hef";
    const char* image_path = "./images/dog.rgb";  // Path to the dog.rgb image
    hailo_network_group_handle network_group_handle = nullptr;

    // Change to use smart pointers (unique_ptr)
    void **input_vstreams = nullptr; // Pointer to an array of input vstreams
    void **output_vstreams = nullptr; // Pointer to an array of output vstreams
    size_t input_count = 0; // Number of input vstreams
    size_t output_count = 0; // Number of output vstreams
    size_t input_frame_size = 0;
    size_t output_frame_size = 0;

    hailo_status status = hailors_configure_hef(
        vdevice_handle,
        hef_path,
        &network_group_handle,
        &input_vstreams,   // Pass pointer to the input vstreams
        &input_count,      // Pass pointer to the input count
        &output_vstreams,  // Pass pointer to the output vstreams
        &output_count,     // Pass pointer to the output count
        &input_frame_size,
        &output_frame_size
    );

    ASSERT_EQ(status, HAILO_SUCCESS);

    // Declare a buffer to store the image data
    std::vector<unsigned char> input_data;

    // Load the image into the buffer
    ASSERT_TRUE(load_test_image(image_path, input_frame_size, input_data)) << "Failed to load test image";

    // Perform inference
    status = hailors_write_input_frame(static_cast<hailort::InputVStream*>(input_vstreams[0]), input_data.data(), input_data.size());
    ASSERT_EQ(status, HAILO_SUCCESS);

    // Prepare a buffer to store the output detections
    std::vector<float> output_data(output_frame_size / sizeof(float));
    status = hailors_read_output_frame(static_cast<hailort::OutputVStream*>(output_vstreams[0]), reinterpret_cast<void*>(output_data.data()), output_frame_size);

    ASSERT_EQ(status, HAILO_SUCCESS);

    //write_output_to_file(output_data, "output.csv", 6);

    auto detections = parse_detections(output_data, 0.85F);


    ASSERT_TRUE(detections[0].class_id == 16);
} 


int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}

