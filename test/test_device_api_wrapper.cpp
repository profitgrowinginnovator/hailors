#include <gtest/gtest.h>
#include "../src/device_api_wrapper.hpp"

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
};

// Initialize static member
hailo_vdevice_handle HailoTestSuite::vdevice_handle = nullptr;

TEST_F(HailoTestSuite, CreateAndReleaseVDevice) {
    // This test just checks the vdevice created in SetUpTestSuite
    ASSERT_NE(vdevice_handle, nullptr);
}

TEST_F(HailoTestSuite, ConfigureNetworkGroup) {
    const char* hef_path = "../hef/yolov6n_h8.hef";
    hailo_network_group_handle network_group_handle = nullptr;
    hailo_input_vstream_handle input_vstreams[16];
    hailo_output_vstream_handle output_vstreams[16];
    size_t input_count = 16;
    size_t output_count = 16;

    hailo_status status = hailors_configure_hef(vdevice_handle, hef_path, &network_group_handle, input_vstreams, &input_count, output_vstreams, &output_count);
    ASSERT_EQ(status, HAILO_SUCCESS);
}

TEST_F(HailoTestSuite, PerformInference) {
    const char* hef_path = "../hef/yolov6n_h8.hef";
    hailo_network_group_handle network_group_handle = nullptr;
    hailo_input_vstream_handle input_vstreams[16];
    hailo_output_vstream_handle output_vstreams[16];
    size_t input_count = 16;
    size_t output_count = 16;

    hailo_status status = hailors_configure_hef(vdevice_handle, hef_path, &network_group_handle, input_vstreams, &input_count, output_vstreams, &output_count);
    ASSERT_EQ(status, HAILO_SUCCESS);

    status = hailors_infer(network_group_handle, input_vstreams, input_count, output_vstreams, output_count);
    ASSERT_EQ(status, HAILO_SUCCESS);
}

int main(int argc, char** argv) {
    ::testing::InitGoogleTest(&argc, argv);
    return RUN_ALL_TESTS();
}
