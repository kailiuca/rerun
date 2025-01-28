# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/archetypes/image.fbs".

# You can extend this class by creating a "ImageExt" class in "image_ext.py".

from __future__ import annotations

import numpy as np
from attrs import define, field

from .. import components, datatypes
from .._baseclasses import (
    Archetype,
    ComponentColumnList,
    DescribedComponentBatch,
)
from ..error_utils import catch_and_log_exceptions
from .image_ext import ImageExt

__all__ = ["Image"]


@define(str=False, repr=False, init=False)
class Image(ImageExt, Archetype):
    """
    **Archetype**: A monochrome or color image.

    See also [`archetypes.DepthImage`][rerun.archetypes.DepthImage] and [`archetypes.SegmentationImage`][rerun.archetypes.SegmentationImage].

    Rerun also supports compressed images (JPEG, PNG, …), using [`archetypes.EncodedImage`][rerun.archetypes.EncodedImage].
    For images that refer to video frames see [`archetypes.VideoFrameReference`][rerun.archetypes.VideoFrameReference].
    Compressing images or using video data instead can save a lot of bandwidth and memory.

    The raw image data is stored as a single buffer of bytes in a [`components.Blob`][rerun.components.Blob].
    The meaning of these bytes is determined by the [`components.ImageFormat`][rerun.components.ImageFormat] which specifies the resolution
    and the pixel format (e.g. RGB, RGBA, …).

    The order of dimensions in the underlying [`components.Blob`][rerun.components.Blob] follows the typical
    row-major, interleaved-pixel image format.

    Examples
    --------
    ### `image_simple`:
    ```python
    import numpy as np
    import rerun as rr

    # Create an image with numpy
    image = np.zeros((200, 300, 3), dtype=np.uint8)
    image[:, :, 0] = 255
    image[50:150, 50:150] = (0, 255, 0)

    rr.init("rerun_example_image", spawn=True)

    rr.log("image", rr.Image(image))
    ```
    <center>
    <picture>
      <source media="(max-width: 480px)" srcset="https://static.rerun.io/image_simple/06ba7f8582acc1ffb42a7fd0006fad7816f3e4e4/480w.png">
      <source media="(max-width: 768px)" srcset="https://static.rerun.io/image_simple/06ba7f8582acc1ffb42a7fd0006fad7816f3e4e4/768w.png">
      <source media="(max-width: 1024px)" srcset="https://static.rerun.io/image_simple/06ba7f8582acc1ffb42a7fd0006fad7816f3e4e4/1024w.png">
      <source media="(max-width: 1200px)" srcset="https://static.rerun.io/image_simple/06ba7f8582acc1ffb42a7fd0006fad7816f3e4e4/1200w.png">
      <img src="https://static.rerun.io/image_simple/06ba7f8582acc1ffb42a7fd0006fad7816f3e4e4/full.png" width="640">
    </picture>
    </center>

    ### Logging images with various formats:
    ```python
    import numpy as np
    import rerun as rr

    rr.init("rerun_example_image_formats", spawn=True)

    # Simple gradient image, logged in different formats.
    image = np.array([[[x, min(255, x + y), y] for x in range(0, 256)] for y in range(0, 256)], dtype=np.uint8)
    rr.log("image_rgb", rr.Image(image))
    rr.log("image_green_only", rr.Image(image[:, :, 1], color_model="l"))  # Luminance only
    rr.log("image_bgr", rr.Image(image[:, :, ::-1], color_model="bgr"))  # BGR

    # New image with Separate Y/U/V planes with 4:2:2 chroma downsampling
    y = bytes([128 for y in range(0, 256) for x in range(0, 256)])
    u = bytes([x * 2 for y in range(0, 256) for x in range(0, 128)])  # Half horizontal resolution for chroma.
    v = bytes([y for y in range(0, 256) for x in range(0, 128)])
    rr.log("image_yuv422", rr.Image(bytes=y + u + v, width=256, height=256, pixel_format=rr.PixelFormat.Y_U_V16_FullRange))
    ```
    <center>
    <picture>
      <img src="https://static.rerun.io/image_formats/7b8a162fcfd266f303980439beea997dc8544c24/full.png" width="640">
    </picture>
    </center>

    """

    # __init__ can be found in image_ext.py

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            buffer=None,
            format=None,
            opacity=None,
            draw_order=None,
        )

    @classmethod
    def _clear(cls) -> Image:
        """Produce an empty Image, bypassing `__init__`."""
        inst = cls.__new__(cls)
        inst.__attrs_clear__()
        return inst

    @classmethod
    def from_fields(
        cls,
        *,
        clear_unset: bool = False,
        buffer: datatypes.BlobLike | None = None,
        format: datatypes.ImageFormatLike | None = None,
        opacity: datatypes.Float32Like | None = None,
        draw_order: datatypes.Float32Like | None = None,
    ) -> Image:
        """
        Update only some specific fields of a `Image`.

        Parameters
        ----------
        clear_unset:
            If true, all unspecified fields will be explicitly cleared.
        buffer:
            The raw image data.
        format:
            The format of the image.
        opacity:
            Opacity of the image, useful for layering several images.

            Defaults to 1.0 (fully opaque).
        draw_order:
            An optional floating point value that specifies the 2D drawing order.

            Objects with higher values are drawn on top of those with lower values.

        """

        inst = cls.__new__(cls)
        with catch_and_log_exceptions(context=cls.__name__):
            kwargs = {
                "buffer": buffer,
                "format": format,
                "opacity": opacity,
                "draw_order": draw_order,
            }

            if clear_unset:
                kwargs = {k: v if v is not None else [] for k, v in kwargs.items()}  # type: ignore[misc]

            inst.__attrs_init__(**kwargs)
            return inst

        inst.__attrs_clear__()
        return inst

    @classmethod
    def cleared(cls) -> Image:
        """Clear all the fields of a `Image`."""
        return cls.from_fields(clear_unset=True)

    @classmethod
    def columns(
        cls,
        *,
        buffer: datatypes.BlobArrayLike | None = None,
        format: datatypes.ImageFormatArrayLike | None = None,
        opacity: datatypes.Float32ArrayLike | None = None,
        draw_order: datatypes.Float32ArrayLike | None = None,
    ) -> ComponentColumnList:
        """
        Construct a new column-oriented component bundle.

        This makes it possible to use `rr.send_columns` to send columnar data directly into Rerun.

        The returned columns will be partitioned into unit-length sub-batches by default.
        Use `ComponentColumnList.partition` to repartition the data as needed.

        Parameters
        ----------
        buffer:
            The raw image data.
        format:
            The format of the image.
        opacity:
            Opacity of the image, useful for layering several images.

            Defaults to 1.0 (fully opaque).
        draw_order:
            An optional floating point value that specifies the 2D drawing order.

            Objects with higher values are drawn on top of those with lower values.

        """

        inst = cls.__new__(cls)
        with catch_and_log_exceptions(context=cls.__name__):
            inst.__attrs_init__(
                buffer=buffer,
                format=format,
                opacity=opacity,
                draw_order=draw_order,
            )

        batches = [batch for batch in inst.as_component_batches() if isinstance(batch, DescribedComponentBatch)]
        if len(batches) == 0:
            return ComponentColumnList([])

        lengths = np.ones(len(batches[0]._batch.as_arrow_array()))
        columns = [batch.partition(lengths) for batch in batches]

        indicator_batch = DescribedComponentBatch(cls.indicator(), cls.indicator().component_descriptor())
        indicator_column = indicator_batch.partition(np.zeros(len(lengths)))

        return ComponentColumnList([indicator_column] + columns)

    buffer: components.ImageBufferBatch | None = field(
        metadata={"component": True},
        default=None,
        converter=components.ImageBufferBatch._converter,  # type: ignore[misc]
    )
    # The raw image data.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    format: components.ImageFormatBatch | None = field(
        metadata={"component": True},
        default=None,
        converter=components.ImageFormatBatch._converter,  # type: ignore[misc]
    )
    # The format of the image.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    opacity: components.OpacityBatch | None = field(
        metadata={"component": True},
        default=None,
        converter=components.OpacityBatch._converter,  # type: ignore[misc]
    )
    # Opacity of the image, useful for layering several images.
    #
    # Defaults to 1.0 (fully opaque).
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    draw_order: components.DrawOrderBatch | None = field(
        metadata={"component": True},
        default=None,
        converter=components.DrawOrderBatch._converter,  # type: ignore[misc]
    )
    # An optional floating point value that specifies the 2D drawing order.
    #
    # Objects with higher values are drawn on top of those with lower values.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__  # type: ignore[assignment]
