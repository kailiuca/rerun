// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/depth_image.fbs".

#include "depth_image.hpp"

#include "../collection_adapter_builtins.hpp"

namespace rerun::archetypes {
    DepthImage DepthImage::clear_fields() {
        auto archetype = DepthImage();
        archetype.buffer = ComponentBatch::empty<rerun::components::ImageBuffer>(Descriptor_buffer)
                               .value_or_throw();
        archetype.format = ComponentBatch::empty<rerun::components::ImageFormat>(Descriptor_format)
                               .value_or_throw();
        archetype.meter =
            ComponentBatch::empty<rerun::components::DepthMeter>(Descriptor_meter).value_or_throw();
        archetype.colormap = ComponentBatch::empty<rerun::components::Colormap>(Descriptor_colormap)
                                 .value_or_throw();
        archetype.depth_range =
            ComponentBatch::empty<rerun::components::ValueRange>(Descriptor_depth_range)
                .value_or_throw();
        archetype.point_fill_ratio =
            ComponentBatch::empty<rerun::components::FillRatio>(Descriptor_point_fill_ratio)
                .value_or_throw();
        archetype.draw_order =
            ComponentBatch::empty<rerun::components::DrawOrder>(Descriptor_draw_order)
                .value_or_throw();
        return archetype;
    }
} // namespace rerun::archetypes

namespace rerun {

    Result<std::vector<ComponentBatch>> AsComponents<archetypes::DepthImage>::serialize(
        const archetypes::DepthImage& archetype
    ) {
        using namespace archetypes;
        std::vector<ComponentBatch> cells;
        cells.reserve(8);

        if (archetype.buffer.has_value()) {
            cells.push_back(archetype.buffer.value());
        }
        if (archetype.format.has_value()) {
            cells.push_back(archetype.format.value());
        }
        if (archetype.meter.has_value()) {
            cells.push_back(archetype.meter.value());
        }
        if (archetype.colormap.has_value()) {
            cells.push_back(archetype.colormap.value());
        }
        if (archetype.depth_range.has_value()) {
            cells.push_back(archetype.depth_range.value());
        }
        if (archetype.point_fill_ratio.has_value()) {
            cells.push_back(archetype.point_fill_ratio.value());
        }
        if (archetype.draw_order.has_value()) {
            cells.push_back(archetype.draw_order.value());
        }
        {
            auto indicator = DepthImage::IndicatorComponent();
            auto result = ComponentBatch::from_loggable(indicator);
            RR_RETURN_NOT_OK(result.error);
            cells.emplace_back(std::move(result.value));
        }

        return cells;
    }
} // namespace rerun
