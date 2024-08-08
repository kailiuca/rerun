// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/depth_image.fbs".

#include "depth_image.hpp"

#include "../collection_adapter_builtins.hpp"

namespace rerun::archetypes {}

namespace rerun {

    Result<std::vector<DataCell>> AsComponents<archetypes::DepthImage>::serialize(
        const archetypes::DepthImage& archetype
    ) {
        using namespace archetypes;
        std::vector<DataCell> cells;
        cells.reserve(7);

        {
            auto result = DataCell::from_loggable(archetype.buffer);
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        {
            auto result = DataCell::from_loggable(archetype.format);
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.meter.has_value()) {
            auto result = DataCell::from_loggable(archetype.meter.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.colormap.has_value()) {
            auto result = DataCell::from_loggable(archetype.colormap.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.point_fill_ratio.has_value()) {
            auto result = DataCell::from_loggable(archetype.point_fill_ratio.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        if (archetype.draw_order.has_value()) {
            auto result = DataCell::from_loggable(archetype.draw_order.value());
            RR_RETURN_NOT_OK(result.error);
            cells.push_back(std::move(result.value));
        }
        {
            auto indicator = DepthImage::IndicatorComponent();
            auto result = DataCell::from_loggable(indicator);
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun
