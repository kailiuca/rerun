// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/components/map_provider.fbs".

#pragma once

#include "../../result.hpp"

#include <cstdint>
#include <memory>

namespace arrow {
    /// \private
    template <typename T>
    class NumericBuilder;

    class Array;
    class DataType;
    class UInt8Type;
    using UInt8Builder = NumericBuilder<UInt8Type>;
} // namespace arrow

namespace rerun::blueprint::components {
    /// **Component**: Name of the map provider to be used in Map views.
    enum class MapProvider : uint8_t {

        /// `OpenStreetMap` is the default map provider.
        OpenStreetMap = 1,

        /// Mapbox Streets is a minimalistic map designed by Mapbox.
        MapboxStreets = 2,

        /// Mapbox Dark is a dark-themed map designed by Mapbox.
        MapboxDark = 3,

        /// Mapbox Satellite is a satellite map designed by Mapbox.
        MapboxSatellite = 4,
    };
} // namespace rerun::blueprint::components

namespace rerun {
    template <typename T>
    struct Loggable;

    /// \private
    template <>
    struct Loggable<blueprint::components::MapProvider> {
        static constexpr const char Name[] = "rerun.blueprint.components.MapProvider";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype();

        /// Serializes an array of `rerun::blueprint:: components::MapProvider` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const blueprint::components::MapProvider* instances, size_t num_instances
        );

        /// Fills an arrow array builder with an array of this type.
        static rerun::Error fill_arrow_array_builder(
            arrow::UInt8Builder* builder, const blueprint::components::MapProvider* elements,
            size_t num_elements
        );
    };
} // namespace rerun
