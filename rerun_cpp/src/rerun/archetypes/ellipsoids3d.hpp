// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/ellipsoids.fbs".

#pragma once

#include "../collection.hpp"
#include "../compiler_utils.hpp"
#include "../component_batch.hpp"
#include "../components/class_id.hpp"
#include "../components/color.hpp"
#include "../components/fill_mode.hpp"
#include "../components/half_size3d.hpp"
#include "../components/leaf_rotation_axis_angle.hpp"
#include "../components/leaf_rotation_quat.hpp"
#include "../components/leaf_translation3d.hpp"
#include "../components/radius.hpp"
#include "../components/text.hpp"
#include "../indicator_component.hpp"
#include "../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::archetypes {
    /// **Archetype**: 3D ellipsoids or spheres.
    ///
    /// This archetype is for ellipsoids or spheres whose size is a key part of the data
    /// (e.g. a bounding sphere).
    /// For points whose radii are for the sake of visualization, use `archetypes::Points3D` instead.
    ///
    /// Note that orienting and placing the ellipsoids/spheres is handled via `[archetypes.LeafTransforms3D]`.
    /// Some of its component are repeated here for convenience.
    struct Ellipsoids3D {
        /// For each ellipsoid, half of its size on its three axes.
        ///
        /// If all components are equal, then it is a sphere with that radius.
        Collection<rerun::components::HalfSize3D> half_sizes;

        /// Optional center positions of the ellipsoids.
        ///
        /// If not specified, the centers will be at (0, 0, 0).
        /// Note that this uses a `components::LeafTranslation3D` which is also used by `archetypes::LeafTransforms3D`.
        std::optional<Collection<rerun::components::LeafTranslation3D>> centers;

        /// Rotations via axis + angle.
        ///
        /// If no rotation is specified, the axes of the ellipsoid align with the axes of the local coordinate system.
        /// Note that this uses a `components::LeafRotationAxisAngle` which is also used by `archetypes::LeafTransforms3D`.
        std::optional<Collection<rerun::components::LeafRotationAxisAngle>> rotation_axis_angles;

        /// Rotations via quaternion.
        ///
        /// If no rotation is specified, the axes of the ellipsoid align with the axes of the local coordinate system.
        /// Note that this uses a `components::LeafRotationQuat` which is also used by `archetypes::LeafTransforms3D`.
        std::optional<Collection<rerun::components::LeafRotationQuat>> quaternions;

        /// Optional colors for the ellipsoids.
        std::optional<Collection<rerun::components::Color>> colors;

        /// Optional radii for the lines used when the ellipsoid is rendered as a wireframe.
        std::optional<Collection<rerun::components::Radius>> line_radii;

        /// Optionally choose whether the ellipsoids are drawn with lines or solid.
        std::optional<rerun::components::FillMode> fill_mode;

        /// Optional text labels for the ellipsoids.
        std::optional<Collection<rerun::components::Text>> labels;

        /// Optional class ID for the ellipsoids.
        ///
        /// The class ID provides colors and labels if not specified explicitly.
        std::optional<Collection<rerun::components::ClassId>> class_ids;

      public:
        static constexpr const char IndicatorComponentName[] =
            "rerun.components.Ellipsoids3DIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;

      public: // START of extensions from ellipsoids3d_ext.cpp:
        /// Creates new `Ellipsoids3D` that are spheres, with `half_sizes` created from radii.
        //
        // TODO(andreas): This should not take an std::vector.
        static Ellipsoids3D from_radii(const std::vector<float>& sizes);

        /// Creates new `Ellipsoids3D` that are spheres, with `half_sizes` and `centers` created
        /// from centers and radii.
        //
        // TODO(andreas): This should not take an std::vector.
        static Ellipsoids3D from_centers_and_radii(
            const std::vector<datatypes::Vec3D>& centers, const std::vector<float>& radii
        );

        /// Creates new `Ellipsoids3D` with `half_sizes` centered around the local origin.
        static Ellipsoids3D from_half_sizes(Collection<components::HalfSize3D> half_sizes) {
            Ellipsoids3D ellipsoids;
            ellipsoids.half_sizes = std::move(half_sizes);
            return ellipsoids;
        }

        /// Creates new `Ellipsoids3D` with `centers` and `half_sizes`.
        static Ellipsoids3D from_centers_and_half_sizes(
            Collection<components::LeafTranslation3D> centers,
            Collection<components::HalfSize3D> half_sizes
        ) {
            Ellipsoids3D ellipsoids;
            ellipsoids.half_sizes = std::move(half_sizes);
            ellipsoids.centers = std::move(centers);
            return ellipsoids;
        }

        // END of extensions from ellipsoids3d_ext.cpp, start of generated code:

      public:
        Ellipsoids3D() = default;
        Ellipsoids3D(Ellipsoids3D&& other) = default;

        /// Optional center positions of the ellipsoids.
        ///
        /// If not specified, the centers will be at (0, 0, 0).
        /// Note that this uses a `components::LeafTranslation3D` which is also used by `archetypes::LeafTransforms3D`.
        Ellipsoids3D with_centers(Collection<rerun::components::LeafTranslation3D> _centers) && {
            centers = std::move(_centers);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Rotations via axis + angle.
        ///
        /// If no rotation is specified, the axes of the ellipsoid align with the axes of the local coordinate system.
        /// Note that this uses a `components::LeafRotationAxisAngle` which is also used by `archetypes::LeafTransforms3D`.
        Ellipsoids3D with_rotation_axis_angles(
            Collection<rerun::components::LeafRotationAxisAngle> _rotation_axis_angles
        ) && {
            rotation_axis_angles = std::move(_rotation_axis_angles);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Rotations via quaternion.
        ///
        /// If no rotation is specified, the axes of the ellipsoid align with the axes of the local coordinate system.
        /// Note that this uses a `components::LeafRotationQuat` which is also used by `archetypes::LeafTransforms3D`.
        Ellipsoids3D with_quaternions(Collection<rerun::components::LeafRotationQuat> _quaternions
        ) && {
            quaternions = std::move(_quaternions);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optional colors for the ellipsoids.
        Ellipsoids3D with_colors(Collection<rerun::components::Color> _colors) && {
            colors = std::move(_colors);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optional radii for the lines used when the ellipsoid is rendered as a wireframe.
        Ellipsoids3D with_line_radii(Collection<rerun::components::Radius> _line_radii) && {
            line_radii = std::move(_line_radii);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optionally choose whether the ellipsoids are drawn with lines or solid.
        Ellipsoids3D with_fill_mode(rerun::components::FillMode _fill_mode) && {
            fill_mode = std::move(_fill_mode);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optional text labels for the ellipsoids.
        Ellipsoids3D with_labels(Collection<rerun::components::Text> _labels) && {
            labels = std::move(_labels);
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Optional class ID for the ellipsoids.
        ///
        /// The class ID provides colors and labels if not specified explicitly.
        Ellipsoids3D with_class_ids(Collection<rerun::components::ClassId> _class_ids) && {
            class_ids = std::move(_class_ids);
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
    struct AsComponents<archetypes::Ellipsoids3D> {
        /// Serialize all set component batches.
        static Result<std::vector<ComponentBatch>> serialize(
            const archetypes::Ellipsoids3D& archetype
        );
    };
} // namespace rerun
