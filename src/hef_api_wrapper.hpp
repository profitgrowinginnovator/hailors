#ifndef HEF_API_WRAPPER_HPP
#define HEF_API_WRAPPER_HPP

#include <cstddef>
#include <cstdint>
#include "hailo/hailort.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Struct representing a network information entry.
 */
typedef struct {
    char *name;                  ///< Name of the network.
    size_t input_count;                ///< Number of input streams.
    size_t output_count;               ///< Number of output streams.
} hailors_network_info_t;

/**
 * Struct representing a stream information entry (input or output).
 */
typedef struct {
    char *name;                  ///< Name of the stream.
    char *data_type;             ///< Data type (e.g., "UINT8", "FLOAT32").
    char *shape;                 ///< Shape (e.g., "NHWC(640x640x3)").
    char *attributes;            ///< Optional attributes for the stream.
} hailors_stream_info_t;

/**
 * Struct representing a post-processing operation entry.
 */
typedef struct {
    char *name;                  ///< Name of the post-processing operation.
    char *description;           ///< Description of the operation.
} hailors_post_processing_op_t;

/**
 * Retrieves general network information.
 *
 * @param hef_path Path to the HEF file.
 * @param network_infos Pointer to the array of network information structs.
 * @param network_count Pointer to the number of networks in the array.
 * @return hailo_status Status of the operation.
 */
hailo_status hailors_get_network_infos(const char *hef_path, hailors_network_info_t **network_infos, size_t *network_count);

/**
 * Retrieves input stream information for a specific network.
 *
 * @param hef_path Path to the HEF file.
 * @param network_name Name of the network.
 * @param input_stream_infos Pointer to the array of input stream information structs.
 * @param input_count Pointer to the number of input streams in the array.
 * @return hailo_status Status of the operation.
 */
hailo_status hailors_get_input_stream_infos(const char *hef_path, const char *network_name, hailors_stream_info_t **input_stream_infos, size_t *input_count);

/**
 * Retrieves output stream information for a specific network.
 *
 * @param hef_path Path to the HEF file.
 * @param network_name Name of the network.
 * @param output_stream_infos Pointer to the array of output stream information structs.
 * @param output_count Pointer to the number of output streams in the array.
 * @return hailo_status Status of the operation.
 */
hailo_status hailors_get_output_stream_infos(const char *hef_path, const char *network_name, hailors_stream_info_t **output_stream_infos, size_t *output_count);

/**
 * Retrieves post-processing operations for a specific network.
 *
 * @param hef_path Path to the HEF file.
 * @param network_name Name of the network.
 * @param post_processing_ops Pointer to the array of post-processing operation structs.
 * @param op_count Pointer to the number of operations in the array.
 * @return hailo_status Status of the operation.
 */
hailo_status hailors_get_post_processing_ops(const char *hef_path, const char *network_name, hailors_post_processing_op_t **post_processing_ops, size_t *op_count);

/**
 * Frees the memory allocated for network information.
 *
 * @param network_infos Pointer to the array of network information structs to free.
 * @param network_count Number of network information structs in the array.
 */
void hailors_free_network_infos(hailors_network_info_t *network_infos, size_t network_count);

/**
 * Frees the memory allocated for stream information.
 *
 * @param stream_infos Pointer to the array of stream information structs to free.
 * @param stream_count Number of stream information structs in the array.
 */
void hailors_free_stream_infos(hailors_stream_info_t *stream_infos, size_t stream_count);

/**
 * Frees the memory allocated for post-processing operations.
 *
 * @param post_processing_ops Pointer to the array of post-processing operation structs to free.
 * @param op_count Number of post-processing operation structs in the array.
 */
void hailors_free_post_processing_ops(hailors_post_processing_op_t *post_processing_ops, size_t op_count);

#ifdef __cplusplus
}
#endif

#endif // HEF_API_WRAPPER_HPP
