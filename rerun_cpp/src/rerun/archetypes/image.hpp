// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/archetypes/image.fbs".

#pragma once

#include "../collection.hpp"
#include "../compiler_utils.hpp"
#include "../component_batch.hpp"
#include "../components/draw_order.hpp"
#include "../components/image_buffer.hpp"
#include "../components/image_format.hpp"
#include "../components/opacity.hpp"
#include "../image_utils.hpp"
#include "../indicator_component.hpp"
#include "../result.hpp"

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

namespace rerun::archetypes {
    /// **Archetype**: A monochrome or color image.
    ///
    /// See also `archetypes::DepthImage` and `archetypes::SegmentationImage`.
    ///
    /// Rerun also supports compressed images (JPEG, PNG, …), using `archetypes::EncodedImage`.
    /// For images that refer to video frames see `archetypes::VideoFrameReference`.
    /// Compressing images or using video data instead can save a lot of bandwidth and memory.
    ///
    /// The raw image data is stored as a single buffer of bytes in a `components::Blob`.
    /// The meaning of these bytes is determined by the `components::ImageFormat` which specifies the resolution
    /// and the pixel format (e.g. RGB, RGBA, …).
    ///
    /// The order of dimensions in the underlying `components::Blob` follows the typical
    /// row-major, interleaved-pixel image format.
    ///
    /// Since the underlying [rerun::components::Blob] uses `rerun::Collection` internally,
    /// data can be passed in without a copy from raw pointers or by reference from `std::vector`/`std::array`/c-arrays.
    /// If needed, this "borrow-behavior" can be extended by defining your own `rerun::CollectionAdapter`.
    ///
    /// ## Examples
    ///
    /// ### image_simple:
    /// ![image](https://static.rerun.io/image_simple/06ba7f8582acc1ffb42a7fd0006fad7816f3e4e4/full.png)
    ///
    /// ```cpp
    /// #include <rerun.hpp>
    ///
    /// #include <vector>
    ///
    /// int main() {
    ///     const auto rec = rerun::RecordingStream("rerun_example_image");
    ///     rec.spawn().exit_on_failure();
    ///
    ///     // Create a synthetic image.
    ///     const int HEIGHT = 200;
    ///     const int WIDTH = 300;
    ///     std::vector<uint8_t> data(WIDTH * HEIGHT * 3, 0);
    ///     for (size_t i = 0; i <data.size(); i += 3) {
    ///         data[i] = 255;
    ///     }
    ///     for (size_t y = 50; y <150; ++y) {
    ///         for (size_t x = 50; x <150; ++x) {
    ///             data[(y * WIDTH + x) * 3 + 0] = 0;
    ///             data[(y * WIDTH + x) * 3 + 1] = 255;
    ///             data[(y * WIDTH + x) * 3 + 2] = 0;
    ///         }
    ///     }
    ///
    ///     rec.log("image", rerun::Image::from_rgb24(data, {WIDTH, HEIGHT}));
    /// }
    /// ```
    ///
    /// ### Logging images with various formats
    /// ![image](https://static.rerun.io/image_formats/7b8a162fcfd266f303980439beea997dc8544c24/full.png)
    ///
    /// ```cpp
    /// #include <algorithm>
    /// #include <cstdint>
    /// #include <vector>
    ///
    /// #include <rerun.hpp>
    ///
    /// int main() {
    ///     const auto rec = rerun::RecordingStream("rerun_example_image_formats");
    ///     rec.spawn().exit_on_failure();
    ///
    ///     // Simple gradient image
    ///     std::vector<uint8_t> image(256 * 256 * 3);
    ///     for (size_t y = 0; y <256; ++y) {
    ///         for (size_t x = 0; x <256; ++x) {
    ///             image[(y * 256 + x) * 3 + 0] = static_cast<uint8_t>(x);
    ///             image[(y * 256 + x) * 3 + 1] = static_cast<uint8_t>(std::min<size_t>(255, x + y));
    ///             image[(y * 256 + x) * 3 + 2] = static_cast<uint8_t>(y);
    ///         }
    ///     }
    ///
    ///     // RGB image
    ///     rec.log("image_rgb", rerun::Image::from_rgb24(image, {256, 256}));
    ///
    ///     // Green channel only (Luminance)
    ///     std::vector<uint8_t> green_channel(256 * 256);
    ///     for (size_t i = 0; i <256 * 256; ++i) {
    ///         green_channel[i] = image[i * 3 + 1];
    ///     }
    ///     rec.log(
    ///         "image_green_only",
    ///         rerun::Image(rerun::borrow(green_channel), {256, 256}, rerun::ColorModel::L)
    ///     );
    ///
    ///     // BGR image
    ///     std::vector<uint8_t> bgr_image(256 * 256 * 3);
    ///     for (size_t i = 0; i <256 * 256; ++i) {
    ///         bgr_image[i * 3 + 0] = image[i * 3 + 2];
    ///         bgr_image[i * 3 + 1] = image[i * 3 + 1];
    ///         bgr_image[i * 3 + 2] = image[i * 3 + 0];
    ///     }
    ///     rec.log(
    ///         "image_bgr",
    ///         rerun::Image(rerun::borrow(bgr_image), {256, 256}, rerun::ColorModel::BGR)
    ///     );
    ///
    ///     // New image with Separate Y/U/V planes with 4:2:2 chroma downsampling
    ///     std::vector<uint8_t> yuv_bytes(256 * 256 + 128 * 256 * 2);
    ///     std::fill_n(yuv_bytes.begin(), 256 * 256, static_cast<uint8_t>(128)); // Fixed value for Y
    ///     size_t u_plane_offset = 256 * 256;
    ///     size_t v_plane_offset = u_plane_offset + 128 * 256;
    ///     for (size_t y = 0; y <256; ++y) {
    ///         for (size_t x = 0; x <128; ++x) {
    ///             auto coord = y * 128 + x;
    ///             yuv_bytes[u_plane_offset + coord] = static_cast<uint8_t>(x * 2); // Gradient for U
    ///             yuv_bytes[v_plane_offset + coord] = static_cast<uint8_t>(y);     // Gradient for V
    ///         }
    ///     }
    ///     rec.log(
    ///         "image_yuv422",
    ///         rerun::Image(rerun::borrow(yuv_bytes), {256, 256}, rerun::PixelFormat::Y_U_V16_FullRange)
    ///     );
    ///
    ///     return 0;
    /// }
    /// ```
    struct Image {
        /// The raw image data.
        std::optional<ComponentBatch> buffer;

        /// The format of the image.
        std::optional<ComponentBatch> format;

        /// Opacity of the image, useful for layering several images.
        ///
        /// Defaults to 1.0 (fully opaque).
        std::optional<ComponentBatch> opacity;

        /// An optional floating point value that specifies the 2D drawing order.
        ///
        /// Objects with higher values are drawn on top of those with lower values.
        std::optional<ComponentBatch> draw_order;

      public:
        static constexpr const char IndicatorComponentName[] = "rerun.components.ImageIndicator";

        /// Indicator component, used to identify the archetype when converting to a list of components.
        using IndicatorComponent = rerun::components::IndicatorComponent<IndicatorComponentName>;
        /// The name of the archetype as used in `ComponentDescriptor`s.
        static constexpr const char ArchetypeName[] = "rerun.archetypes.Image";

        /// `ComponentDescriptor` for the `buffer` field.
        static constexpr auto Descriptor_buffer = ComponentDescriptor(
            ArchetypeName, "buffer",
            Loggable<rerun::components::ImageBuffer>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `format` field.
        static constexpr auto Descriptor_format = ComponentDescriptor(
            ArchetypeName, "format",
            Loggable<rerun::components::ImageFormat>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `opacity` field.
        static constexpr auto Descriptor_opacity = ComponentDescriptor(
            ArchetypeName, "opacity",
            Loggable<rerun::components::Opacity>::Descriptor.component_name
        );
        /// `ComponentDescriptor` for the `draw_order` field.
        static constexpr auto Descriptor_draw_order = ComponentDescriptor(
            ArchetypeName, "draw_order",
            Loggable<rerun::components::DrawOrder>::Descriptor.component_name
        );

      public: // START of extensions from image_ext.cpp:
        /// Construct an image from bytes and image format.
        ///
        /// @param bytes The raw image data as bytes.
        /// If the data does not outlive the image, use `std::move` or create the `rerun::Collection`
        /// explicitly ahead of time with `rerun::Collection::take_ownership`.
        /// The length of the data should be `W * H * image_format.bytes_per_pixel`.
        /// @param format_ How the data should be interpreted.
        Image(Collection<uint8_t> bytes, components::ImageFormat format_) {
            if (bytes.size() != format_.image_format.num_bytes()) {
                Error(
                    ErrorCode::InvalidTensorDimension,
                    "Image buffer has the wrong size. Got " + std::to_string(bytes.size()) +
                        " bytes, expected " + std::to_string(format_.image_format.num_bytes())
                )
                    .handle();
            }
            *this = std::move(*this).with_buffer(bytes).with_format(format_);
        }

        /// Construct an image from resolution, pixel format and bytes.
        ///
        /// @param bytes The raw image data as bytes.
        /// If the data does not outlive the image, use `std::move` or create the `rerun::Collection`
        /// explicitly ahead of time with `rerun::Collection::take_ownership`.
        /// The length of the data should be `W * H * pixel_format.bytes_per_pixel`.
        /// @param resolution The resolution of the image as {width, height}.
        /// @param pixel_format How the data should be interpreted.
        Image(
            Collection<uint8_t> bytes, WidthHeight resolution, datatypes::PixelFormat pixel_format
        )
            : Image{std::move(bytes), datatypes::ImageFormat{resolution, pixel_format}} {}

        /// Construct an image from resolution, color model, channel datatype and bytes.
        ///
        /// @param bytes The raw image data.
        /// If the data does not outlive the image, use `std::move` or create the `rerun::Collection`
        /// explicitly ahead of time with `rerun::Collection::take_ownership`.
        /// The length of the data should be `W * H * datatype.bytes * color_model.num_channels`.
        /// @param resolution The resolution of the image as {width, height}.
        /// @param color_model The color model of the pixel data.
        /// @param datatype Datatype of the individual channels of the color model.
        Image(
            Collection<uint8_t> bytes, WidthHeight resolution, datatypes::ColorModel color_model,
            datatypes::ChannelDatatype datatype
        )
            : Image(std::move(bytes), datatypes::ImageFormat(resolution, color_model, datatype)) {}

        /// Construct an image from resolution, color model and elements,
        /// inferring the channel datatype from the element type.
        ///
        /// @param elements Pixel data as a `rerun::Collection`.
        /// If the data does not outlive the image, use `std::move` or create the `rerun::Collection`
        /// explicitly ahead of time with `rerun::Collection::take_ownership`.
        /// The length of the data should be `W * H * color_model.num_channels`.
        /// @param resolution The resolution of the image as {width, height}.
        /// @param color_model The color model of the pixel data.
        /// Each element in elements is interpreted as a single channel of the color model.
        template <typename T>
        Image(Collection<T> elements, WidthHeight resolution, datatypes::ColorModel color_model)
            : Image(elements.to_uint8(), resolution, color_model, get_datatype(elements.data())) {}

        /// Construct an image from resolution, color model and element pointer,
        /// inferring the channel datatype from the element type.
        ///
        /// @param elements The raw image data.
        /// ⚠️ Does not take ownership of the data, the caller must ensure the data outlives the image.
        /// The number of elements is assumed to be `W * H * color_model.num_channels`.
        /// @param resolution The resolution of the image as {width, height}.
        /// @param color_model The color model of the pixel data.
        /// Each element in elements is interpreted as a single channel of the color model.
        template <typename T>
        Image(const T* elements, WidthHeight resolution, datatypes::ColorModel color_model)
            : Image(
                  rerun::Collection<uint8_t>::borrow(
                      reinterpret_cast<const uint8_t*>(elements),
                      resolution.width * resolution.height * color_model_channel_count(color_model)
                  ),
                  resolution, color_model, get_datatype(elements)
              ) {}

        /// Assumes single channel greyscale/luminance with 8-bit per value.
        ///
        /// @param bytes Pixel data as a `rerun::Collection`.
        /// If the data does not outlive the image, use `std::move` or create the `rerun::Collection`
        /// explicitly ahead of time with `rerun::Collection::take_ownership`.
        /// The length of the data should be `W * H`.
        /// @param resolution The resolution of the image as {width, height}.
        static Image from_greyscale8(Collection<uint8_t> bytes, WidthHeight resolution) {
            return Image(
                bytes,
                resolution,
                datatypes::ColorModel::L,
                datatypes::ChannelDatatype::U8
            );
        }

        /// Assumes RGB, 8-bit per channel, packed as `RGBRGBRGB…`.
        ///
        /// @param bytes Pixel data as a `rerun::Collection`.
        /// If the data does not outlive the image, use `std::move` or create the `rerun::Collection`
        /// explicitly ahead of time with `rerun::Collection::take_ownership`.
        /// The length of the data should be `W * H * 3`.
        /// @param resolution The resolution of the image as {width, height}.
        static Image from_rgb24(Collection<uint8_t> bytes, WidthHeight resolution) {
            return Image(
                bytes,
                resolution,
                datatypes::ColorModel::RGB,
                datatypes::ChannelDatatype::U8
            );
        }

        /// Assumes RGBA, 8-bit per channel, with separate alpha.
        ///
        /// @param bytes Pixel data as a `rerun::Collection`.
        /// If the data does not outlive the image, use `std::move` or create the `rerun::Collection`
        /// explicitly ahead of time with `rerun::Collection::take_ownership`.
        /// The length of the data should be `W * H * 4`.
        /// @param resolution The resolution of the image as {width, height}.
        static Image from_rgba32(Collection<uint8_t> bytes, WidthHeight resolution) {
            return Image(
                bytes,
                resolution,
                datatypes::ColorModel::RGBA,
                datatypes::ChannelDatatype::U8
            );
        }

        // END of extensions from image_ext.cpp, start of generated code:

      public:
        Image() = default;
        Image(Image&& other) = default;
        Image(const Image& other) = default;
        Image& operator=(const Image& other) = default;
        Image& operator=(Image&& other) = default;

        /// Update only some specific fields of a `Image`.
        static Image update_fields() {
            return Image();
        }

        /// Clear all the fields of a `Image`.
        static Image clear_fields();

        /// The raw image data.
        Image with_buffer(const rerun::components::ImageBuffer& _buffer) && {
            buffer = ComponentBatch::from_loggable(_buffer, Descriptor_buffer).value_or_throw();
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// The format of the image.
        Image with_format(const rerun::components::ImageFormat& _format) && {
            format = ComponentBatch::from_loggable(_format, Descriptor_format).value_or_throw();
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// Opacity of the image, useful for layering several images.
        ///
        /// Defaults to 1.0 (fully opaque).
        Image with_opacity(const rerun::components::Opacity& _opacity) && {
            opacity = ComponentBatch::from_loggable(_opacity, Descriptor_opacity).value_or_throw();
            // See: https://github.com/rerun-io/rerun/issues/4027
            RR_WITH_MAYBE_UNINITIALIZED_DISABLED(return std::move(*this);)
        }

        /// An optional floating point value that specifies the 2D drawing order.
        ///
        /// Objects with higher values are drawn on top of those with lower values.
        Image with_draw_order(const rerun::components::DrawOrder& _draw_order) && {
            draw_order =
                ComponentBatch::from_loggable(_draw_order, Descriptor_draw_order).value_or_throw();
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
    struct AsComponents<archetypes::Image> {
        /// Serialize all set component batches.
        static Result<std::vector<ComponentBatch>> serialize(const archetypes::Image& archetype);
    };
} // namespace rerun
