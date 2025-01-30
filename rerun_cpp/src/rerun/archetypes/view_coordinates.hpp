// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/view_coordinates.fbs".

#pragma once

#include "../collection.hpp"
#include "../component_batch.hpp"
#include "../component_column.hpp"
#include "../components/view_coordinates.hpp"
#include "../indicator_component.hpp"
#include "../rerun_sdk_export.hpp"
#include "../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::archetypes {
    /// **Archetype**: How we interpret the coordinate system of an entity/space.
    ///
    /// For instance: What is "up"? What does the Z axis mean?
    ///
    /// The three coordinates are always ordered as [x, y, z].
    ///
    /// For example [Right, Down, Forward] means that the X axis points to the right, the Y axis points
    /// down, and the Z axis points forward.
    ///
    /// Make sure that this archetype is logged at or above the origin entity path of your 3D views.
    ///
    /// ⚠ [Rerun does not yet support left-handed coordinate systems](https://github.com/rerun-io/rerun/issues/5032).
    ///
    /// ## Example
    ///
    /// ### View coordinates for adjusting the eye camera
    /// ![image](https://static.rerun.io/viewcoordinates/0833f0dc8616a676b7b2c566f2a6f613363680c5/full.png)
    ///
    /// ```cpp
    /// #include <rerun.hpp>
    ///
    /// int main() {
    ///     const auto rec = rerun::RecordingStream("rerun_example_view_coordinates");
    ///     rec.spawn().exit_on_failure();
    ///
    ///     rec.log_static("world", rerun::ViewCoordinates::RIGHT_HAND_Z_UP); // Set an up-axis
    ///     rec.log(
    ///         "world/xyz",
    ///         rerun::Arrows3D::from_vectors({{1.0, 0.0, 0.0}, {0.0, 1.0, 0.0}, {0.0, 0.0, 1.0}}
    ///         ).with_colors({{255, 0, 0}, {0, 255, 0}, {0, 0, 255}})
    ///     );
    /// }
    /// ```
    struct ViewCoordinates {
        /// The directions of the [x, y, z] axes.
        std::optional<ComponentBatch> xyz;

      public:
        static constexpr const char IndicatorComponentName[] =
            "rerun.components.ViewCoordinatesIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;
        /// The name of the archetype as used in `ComponentDescriptor`s.
        static constexpr const char ArchetypeName[] = "rerun.archetypes.ViewCoordinates";

        /// `ComponentDescriptor` for the `xyz` field.
        static constexpr auto Descriptor_xyz = ComponentDescriptor(
            ArchetypeName, "xyz",
            Loggable<rerun::components::ViewCoordinates>::Descriptor.component_name
        );

      public: // START of extensions from view_coordinates_ext.cpp:
        /// Construct Vec3D from x/y/z values.
        ViewCoordinates(uint8_t axis0, uint8_t axis1, uint8_t axis2)
            : ViewCoordinates(rerun::components::ViewCoordinates(axis0, axis1, axis2)) {}

        // <BEGIN_GENERATED:declarations>
        // This section is generated by running `scripts/generate_view_coordinate_defs.py --cpp`
        /// X=Up, Y=Left, Z=Forward
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates ULF;

        /// X=Up, Y=Forward, Z=Left
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates UFL;

        /// X=Left, Y=Up, Z=Forward
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates LUF;

        /// X=Left, Y=Forward, Z=Up
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates LFU;

        /// X=Forward, Y=Up, Z=Left
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates FUL;

        /// X=Forward, Y=Left, Z=Up
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates FLU;

        /// X=Up, Y=Left, Z=Back
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates ULB;

        /// X=Up, Y=Back, Z=Left
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates UBL;

        /// X=Left, Y=Up, Z=Back
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates LUB;

        /// X=Left, Y=Back, Z=Up
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates LBU;

        /// X=Back, Y=Up, Z=Left
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates BUL;

        /// X=Back, Y=Left, Z=Up
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates BLU;

        /// X=Up, Y=Right, Z=Forward
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates URF;

        /// X=Up, Y=Forward, Z=Right
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates UFR;

        /// X=Right, Y=Up, Z=Forward
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates RUF;

        /// X=Right, Y=Forward, Z=Up
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates RFU;

        /// X=Forward, Y=Up, Z=Right
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates FUR;

        /// X=Forward, Y=Right, Z=Up
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates FRU;

        /// X=Up, Y=Right, Z=Back
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates URB;

        /// X=Up, Y=Back, Z=Right
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates UBR;

        /// X=Right, Y=Up, Z=Back
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates RUB;

        /// X=Right, Y=Back, Z=Up
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates RBU;

        /// X=Back, Y=Up, Z=Right
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates BUR;

        /// X=Back, Y=Right, Z=Up
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates BRU;

        /// X=Down, Y=Left, Z=Forward
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates DLF;

        /// X=Down, Y=Forward, Z=Left
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates DFL;

        /// X=Left, Y=Down, Z=Forward
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates LDF;

        /// X=Left, Y=Forward, Z=Down
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates LFD;

        /// X=Forward, Y=Down, Z=Left
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates FDL;

        /// X=Forward, Y=Left, Z=Down
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates FLD;

        /// X=Down, Y=Left, Z=Back
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates DLB;

        /// X=Down, Y=Back, Z=Left
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates DBL;

        /// X=Left, Y=Down, Z=Back
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates LDB;

        /// X=Left, Y=Back, Z=Down
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates LBD;

        /// X=Back, Y=Down, Z=Left
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates BDL;

        /// X=Back, Y=Left, Z=Down
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates BLD;

        /// X=Down, Y=Right, Z=Forward
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates DRF;

        /// X=Down, Y=Forward, Z=Right
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates DFR;

        /// X=Right, Y=Down, Z=Forward
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates RDF;

        /// X=Right, Y=Forward, Z=Down
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates RFD;

        /// X=Forward, Y=Down, Z=Right
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates FDR;

        /// X=Forward, Y=Right, Z=Down
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates FRD;

        /// X=Down, Y=Right, Z=Back
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates DRB;

        /// X=Down, Y=Back, Z=Right
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates DBR;

        /// X=Right, Y=Down, Z=Back
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates RDB;

        /// X=Right, Y=Back, Z=Down
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates RBD;

        /// X=Back, Y=Down, Z=Right
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates BDR;

        /// X=Back, Y=Right, Z=Down
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates BRD;

        /// X=Up, Y=Right, Z=Forward
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates RIGHT_HAND_X_UP;

        /// X=Down, Y=Right, Z=Back
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates RIGHT_HAND_X_DOWN;

        /// X=Right, Y=Up, Z=Back
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates RIGHT_HAND_Y_UP;

        /// X=Right, Y=Down, Z=Forward
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates RIGHT_HAND_Y_DOWN;

        /// X=Right, Y=Forward, Z=Up
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates RIGHT_HAND_Z_UP;

        /// X=Right, Y=Back, Z=Down
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates RIGHT_HAND_Z_DOWN;

        /// X=Up, Y=Right, Z=Back
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates LEFT_HAND_X_UP;

        /// X=Down, Y=Right, Z=Forward
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates LEFT_HAND_X_DOWN;

        /// X=Right, Y=Up, Z=Forward
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates LEFT_HAND_Y_UP;

        /// X=Right, Y=Down, Z=Back
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates LEFT_HAND_Y_DOWN;

        /// X=Right, Y=Back, Z=Up
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates LEFT_HAND_Z_UP;

        /// X=Right, Y=Forward, Z=Down
        ///
        /// ⚠️ This is a left-handed coordinate system, which is [not yet supported by Rerun](https://github.com/rerun-io/rerun/issues/5032).
        RERUN_SDK_EXPORT static const rerun::archetypes::ViewCoordinates LEFT_HAND_Z_DOWN;

        // <END_GENERATED:declarations>

        // END of extensions from view_coordinates_ext.cpp, start of generated code:

      public:
        ViewCoordinates() = default;
        ViewCoordinates(ViewCoordinates&& other) = default;
        ViewCoordinates(const ViewCoordinates& other) = default;
        ViewCoordinates& operator=(const ViewCoordinates& other) = default;
        ViewCoordinates& operator=(ViewCoordinates&& other) = default;

        explicit ViewCoordinates(rerun::components::ViewCoordinates _xyz)
            : xyz(ComponentBatch::from_loggable(std::move(_xyz), Descriptor_xyz).value_or_throw()) {
        }

        /// Update only some specific fields of a `ViewCoordinates`.
        static ViewCoordinates update_fields() {
            return ViewCoordinates();
        }

        /// Clear all the fields of a `ViewCoordinates`.
        static ViewCoordinates clear_fields();

        /// The directions of the [x, y, z] axes.
        ViewCoordinates with_xyz(const rerun::components::ViewCoordinates& _xyz) && {
            xyz = ComponentBatch::from_loggable(_xyz, Descriptor_xyz).value_or_throw();
            return std::move(*this);
        }

        /// This method makes it possible to pack multiple `xyz` in a single component batch.
        ///
        /// This only makes sense when used in conjunction with `columns`. `with_xyz` should
        /// be used when logging a single row's worth of data.
        ViewCoordinates with_many_xyz(const Collection<rerun::components::ViewCoordinates>& _xyz
        ) && {
            xyz = ComponentBatch::from_loggable(_xyz, Descriptor_xyz).value_or_throw();
            return std::move(*this);
        }

        /// Partitions the component data into multiple sub-batches.
        ///
        /// Specifically, this transforms the existing `ComponentBatch` data into `ComponentColumn`s
        /// instead, via `ComponentColumn::from_batch_with_lengths`.
        ///
        /// This makes it possible to use `RecordingStream::send_columns` to send columnar data directly into Rerun.
        ///
        /// The specified `lengths` must sum to the total length of the component batch.
        Collection<ComponentColumn> columns(const Collection<uint32_t>& lengths_);

        /// Partitions the component data into unit-length sub-batches.
        ///
        /// This is semantically similar to calling `columns` with `std::vector<uint32_t>(n, 1)`,
        /// where `n` is automatically guessed.
        Collection<ComponentColumn> columns();
    };

} // namespace rerun::archetypes

namespace rerun {
    /// \private
    template <typename T>
    struct AsComponents;

    /// \private
    template <>
    struct AsComponents<archetypes::ViewCoordinates> {
        /// Serialize all set component batches.
        static Result<std::vector<ComponentBatch>> serialize(
            const archetypes::ViewCoordinates& archetype
        );
    };
} // namespace rerun
