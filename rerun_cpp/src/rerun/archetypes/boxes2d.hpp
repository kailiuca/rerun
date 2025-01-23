// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/boxes2d.fbs".

#pragma once

#include "../collection.hpp"
#include "../compiler_utils.hpp"
#include "../component_batch.hpp"
#include "../components/class_id.hpp"
#include "../components/color.hpp"
#include "../components/draw_order.hpp"
#include "../components/half_size2d.hpp"
#include "../components/position2d.hpp"
#include "../components/radius.hpp"
#include "../components/show_labels.hpp"
#include "../components/text.hpp"
#include "../indicator_component.hpp"
#include "../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::archetypes {
    /// **Archetype**: 2D boxes with half-extents and optional center, colors etc.
    ///
    /// ## Example
    ///
    /// ### Simple 2D boxes
    /// ![image](https://static.rerun.io/box2d_simple/ac4424f3cf747382867649610cbd749c45b2020b/full.png)
    ///
    /// ```cpp
    /// #include <rerun.hpp>
    ///
    /// int main() {
    ///     const auto rec = rerun::RecordingStream("rerun_example_box2d");
    ///     rec.spawn().exit_on_failure();
    ///
    ///     rec.log("simple", rerun::Boxes2D::from_mins_and_sizes({{-1.f, -1.f}}, {{2.f, 2.f}}));
    /// }
    /// ```
    struct Boxes2D {
        /// All half-extents that make up the batch of boxes.
        std::optional<ComponentBatch> half_sizes;

        /// Optional center positions of the boxes.
        std::optional<ComponentBatch> centers;

        /// Optional colors for the boxes.
        std::optional<ComponentBatch> colors;

        /// Optional radii for the lines that make up the boxes.
        std::optional<ComponentBatch> radii;

        /// Optional text labels for the boxes.
        ///
        /// If there's a single label present, it will be placed at the center of the entity.
        /// Otherwise, each instance will have its own label.
        std::optional<ComponentBatch> labels;

        /// Optional choice of whether the text labels should be shown by default.
        std::optional<ComponentBatch> show_labels;

        /// An optional floating point value that specifies the 2D drawing order.
        ///
        /// Objects with higher values are drawn on top of those with lower values.
        ///
        /// The default for 2D boxes is 10.0.
        std::optional<ComponentBatch> draw_order;

        /// Optional `components::ClassId`s for the boxes.
        ///
        /// The `components::ClassId` provides colors and labels if not specified explicitly.
        std::optional<ComponentBatch> class_ids;

      public:
        static constexpr const char IndicatorComponentName[] = "rerun.components.Boxes2DIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;
        /// The name of the archetype as used in `ComponentDescriptor`s.
        static constexpr const char ArchetypeName[] = "rerun.archetypes.Boxes2D";

        /// `ComponentDescriptor` for the `half_sizes` field.
        static constexpr auto Descriptor_half_sizes = ComponentDescriptor(
            ArchetypeName, "half_sizes",
            Loggable<rerun::components::HalfSize2D>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `centers` field.
        static constexpr auto Descriptor_centers = ComponentDescriptor(
            ArchetypeName, "centers",
            Loggable<rerun::components::Position2D>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `colors` field.
        static constexpr auto Descriptor_colors = ComponentDescriptor(
            ArchetypeName, "colors", Loggable<rerun::components::Color>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `radii` field.
        static constexpr auto Descriptor_radii = ComponentDescriptor(
            ArchetypeName, "radii", Loggable<rerun::components::Radius>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `labels` field.
        static constexpr auto Descriptor_labels = ComponentDescriptor(
            ArchetypeName, "labels", Loggable<rerun::components::Text>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `show_labels` field.
        static constexpr auto Descriptor_show_labels = ComponentDescriptor(
            ArchetypeName, "show_labels",
            Loggable<rerun::components::ShowLabels>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `draw_order` field.
        static constexpr auto Descriptor_draw_order = ComponentDescriptor(
            ArchetypeName, "draw_order",
            Loggable<rerun::components::DrawOrder>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `class_ids` field.
        static constexpr auto Descriptor_class_ids = ComponentDescriptor(
            ArchetypeName, "class_ids",
            Loggable<rerun::components::ClassId>::Descriptor.component_name
        );

      public: // START of extensions from boxes2d_ext.cpp:
        /// Creates new `Boxes2D` with `half_sizes` centered around the local origin.
        static Boxes2D from_half_sizes(Collection<components::HalfSize2D> half_sizes) {
            return Boxes2D().with_half_sizes(std::move(half_sizes));
        }

        /// Creates new `Boxes2D` with `centers` and `half_sizes`.
        static Boxes2D from_centers_and_half_sizes(
            Collection<components::Position2D> centers,
            Collection<components::HalfSize2D> half_sizes
        ) {
            return Boxes2D()
                .with_half_sizes(std::move(half_sizes))
                .with_centers(std::move(centers));
        }

        /// Creates new `Boxes2D` with `half_sizes` created from (full) sizes.
        ///
        /// TODO(#3285): Does *not* preserve data as-is and instead creates half-sizes from the
        /// input data.
        static Boxes2D from_sizes(const std::vector<datatypes::Vec2D>& sizes);

        /// Creates new `Boxes2D` with `centers` and `half_sizes` created from centers and (full)
        /// sizes.
        ///
        /// TODO(#3285): Does *not* preserve data as-is and instead creates centers and half-sizes
        /// from the input data.
        static Boxes2D from_centers_and_sizes(
            Collection<components::Position2D> centers, const std::vector<datatypes::Vec2D>& sizes
        ) {
            return from_sizes(std::move(sizes)).with_centers(std::move(centers));
        }

        /// Creates new `Boxes2D` with `half_sizes` and `centers` created from minimums and (full)
        /// sizes.
        ///
        /// TODO(#3285): Does *not* preserve data as-is and instead creates centers and half-sizes
        /// from the input data.
        static Boxes2D from_mins_and_sizes(
            const std::vector<datatypes::Vec2D>& mins, const std::vector<datatypes::Vec2D>& sizes
        );

        // END of extensions from boxes2d_ext.cpp, start of generated code:

      public:
        Boxes2D() = default;
        Boxes2D(Boxes2D&& other) = default;
        Boxes2D(const Boxes2D& other) = default;
        Boxes2D& operator=(const Boxes2D& other) = default;
        Boxes2D& operator=(Boxes2D&& other) = default;

        /// Update only some specific fields of a `Boxes2D`.
        static Boxes2D update_fields() {
            return Boxes2D();
        }

        /// Clear all the fields of a `Boxes2D`.
        static Boxes2D clear_fields();

        /// All half-extents that make up the batch of boxes.
        Boxes2D with_half_sizes(const Collection<rerun::components::HalfSize2D>& _half_sizes) && {
            half_sizes =
                ComponentBatch::from_loggable(_half_sizes, Descriptor_half_sizes).value_or_throw();
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optional center positions of the boxes.
        Boxes2D with_centers(const Collection<rerun::components::Position2D>& _centers) && {
            centers = ComponentBatch::from_loggable(_centers, Descriptor_centers).value_or_throw();
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optional colors for the boxes.
        Boxes2D with_colors(const Collection<rerun::components::Color>& _colors) && {
            colors = ComponentBatch::from_loggable(_colors, Descriptor_colors).value_or_throw();
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optional radii for the lines that make up the boxes.
        Boxes2D with_radii(const Collection<rerun::components::Radius>& _radii) && {
            radii = ComponentBatch::from_loggable(_radii, Descriptor_radii).value_or_throw();
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optional text labels for the boxes.
        ///
        /// If there's a single label present, it will be placed at the center of the entity.
        /// Otherwise, each instance will have its own label.
        Boxes2D with_labels(const Collection<rerun::components::Text>& _labels) && {
            labels = ComponentBatch::from_loggable(_labels, Descriptor_labels).value_or_throw();
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optional choice of whether the text labels should be shown by default.
        Boxes2D with_show_labels(const rerun::components::ShowLabels& _show_labels) && {
            show_labels = ComponentBatch::from_loggable(_show_labels, Descriptor_show_labels)
                              .value_or_throw();
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// An optional floating point value that specifies the 2D drawing order.
        ///
        /// Objects with higher values are drawn on top of those with lower values.
        ///
        /// The default for 2D boxes is 10.0.
        Boxes2D with_draw_order(const rerun::components::DrawOrder& _draw_order) && {
            draw_order =
                ComponentBatch::from_loggable(_draw_order, Descriptor_draw_order).value_or_throw();
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optional `components::ClassId`s for the boxes.
        ///
        /// The `components::ClassId` provides colors and labels if not specified explicitly.
        Boxes2D with_class_ids(const Collection<rerun::components::ClassId>& _class_ids) && {
            class_ids =
                ComponentBatch::from_loggable(_class_ids, Descriptor_class_ids).value_or_throw();
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }
    };

} // namespace rerun::archetypes

namespace rerun {
    /// \private
    template <typename T>
    struct AsComponents;

    /// \private
    template <>
    struct AsComponents<archetypes::Boxes2D> {
        /// Serialize all set component batches.
        static Result<std::vector<ComponentBatch>> serialize(const archetypes::Boxes2D& archetype);
    };
} // namespace rerun
