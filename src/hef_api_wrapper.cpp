#include "hailo/hef.hpp"
#include "nlohmann/json.hpp" // JSON library for C++

nlohmann::json stream_info_to_json(const hailo_stream_info_t &stream_info) {
    return {
        {"name", stream_info.name},
        {"data_type", stream_info.data_type},
        {"shape", {
            {"height", stream_info.shape.height},
            {"width", stream_info.shape.width},
            {"features", stream_info.shape.features}
        }},
        {"format", {
            {"type", static_cast<int>(stream_info.format.type)},
            {"order", static_cast<int>(stream_info.format.order)}
        }},
        {"quant_info", {
            {"scale", stream_info.quant_info.scale},
            {"zero_point", stream_info.quant_info.zero_point}
        }},
        {"direction", static_cast<int>(stream_info.direction)}
    };
}

nlohmann::json post_processing_op_to_json(const hailors_post_processing_op_t &op) {
    return {
        {"name", op.name},
        {"description", op.description}
    };
}

nlohmann::json network_info_to_json(const hailo_network_info_t &network_info) {
    return {
        {"name", network_info.name},
        {"input_count", network_info.input_count},
        {"output_count", network_info.output_count}
    };
}

nlohmann::json layer_info_to_json(const hailort::LayerInfo &layer_info) {
    return {
        {"type", static_cast<int>(layer_info.type)},
        {"name", layer_info.name},
        {"network_name", layer_info.network_name},
        {"shape", {
            {"height", layer_info.shape.height},
            {"width", layer_info.shape.width},
            {"features", layer_info.shape.features}
        }},
        {"is_mux", layer_info.is_mux},
        {"is_multi_planar", layer_info.is_multi_planar},
        {"is_defused_nms", layer_info.is_defused_nms}
    };
}

nlohmann::json hailors_get_full_hef_info(const std::string &hef_path) {
    hailo_status status;
    auto hef_result = hailort::Hef::create(hef_path);
    if (!hef_result) {
        throw std::runtime_error("Failed to load HEF file: " + std::to_string(hef_result.status()));
    }

    auto hef = std::move(hef_result.value());
    nlohmann::json hef_json;

    // Fetch network information
    auto network_infos_result = hef.get_network_infos();
    if (network_infos_result) {
        for (const auto &network_info : network_infos_result.value()) {
            nlohmann::json network_json = network_info_to_json(network_info);

            // Input streams
            auto input_streams_result = hef.get_input_stream_infos(network_info.name);
            if (input_streams_result) {
                for (const auto &stream_info : input_streams_result.value()) {
                    network_json["input_streams"].push_back(stream_info_to_json(stream_info));
                }
            }

            // Output streams
            auto output_streams_result = hef.get_output_stream_infos(network_info.name);
            if (output_streams_result) {
                for (const auto &stream_info : output_streams_result.value()) {
                    network_json["output_streams"].push_back(stream_info_to_json(stream_info));
                }
            }

            // Post-processing operations
            auto post_processing_ops_result = hef.get_post_processing_ops(network_info.name);
            if (post_processing_ops_result) {
                for (const auto &op : post_processing_ops_result.value()) {
                    network_json["post_processing_ops"].push_back(post_processing_op_to_json(op));
                }
            }

            hef_json["networks"].push_back(network_json);
        }
    }

    // Fetch layer information for each core operation
    auto core_ops = hef.core_ops();
    for (const auto &core_op : core_ops) {
        nlohmann::json core_op_json;
        core_op_json["name"] = core_op.core_op_name;

        // Input layers
        auto input_layers = core_op.get_input_layer_infos();
        for (const auto &layer : input_layers) {
            core_op_json["input_layers"].push_back(layer_info_to_json(layer));
        }

        // Output layers
        auto output_layers = core_op.get_output_layer_infos();
        for (const auto &layer : output_layers) {
            core_op_json["output_layers"].push_back(layer_info_to_json(layer));
        }

        // Additional metadata
        core_op_json["supports_fast_batch_switch"] = core_op.get_can_fast_batch_switch();

        hef_json["core_ops"].push_back(core_op_json);
    }

    return hef_json;
}
