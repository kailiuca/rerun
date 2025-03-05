# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/datatypes/color_model.fbs".

# You can extend this class by creating a "ColorModelExt" class in "color_model_ext.py".

from __future__ import annotations

from collections.abc import Sequence
from typing import Literal, Union

import pyarrow as pa

from .._baseclasses import (
    BaseBatch,
)

__all__ = ["ColorModel", "ColorModelArrayLike", "ColorModelBatch", "ColorModelLike"]


from enum import Enum


class ColorModel(Enum):
    """
    **Datatype**: Specified what color components are present in an [`archetypes.Image`][rerun.archetypes.Image].

    This combined with [`datatypes.ChannelDatatype`][rerun.datatypes.ChannelDatatype] determines the pixel format of an image.
    """

    L = 1
    """Grayscale luminance intencity/brightness/value, sometimes called `Y`"""

    RGB = 2
    """Red, Green, Blue"""

    RGBA = 3
    """Red, Green, Blue, Alpha"""

    BGR = 4
    """Blue, Green, Red"""

    BGRA = 5
    """Blue, Green, Red, Alpha"""

    @classmethod
    def auto(cls, val: str | int | ColorModel) -> ColorModel:
        """Best-effort converter, including a case-insensitive string matcher."""
        if isinstance(val, ColorModel):
            return val
        if isinstance(val, int):
            return cls(val)
        try:
            return cls[val]
        except KeyError:
            val_lower = val.lower()
            for variant in cls:
                if variant.name.lower() == val_lower:
                    return variant
        raise ValueError(f"Cannot convert {val} to {cls.__name__}")

    def __str__(self) -> str:
        """Returns the variant name."""
        return self.name


ColorModelLike = Union[ColorModel, Literal["BGR", "BGRA", "L", "RGB", "RGBA", "bgr", "bgra", "l", "rgb", "rgba"], int]
ColorModelArrayLike = Union[ColorModelLike, Sequence[ColorModelLike]]


class ColorModelBatch(BaseBatch[ColorModelArrayLike]):
    _ARROW_DATATYPE = pa.uint8()

    @staticmethod
    def _native_to_pa_array(data: ColorModelArrayLike, data_type: pa.DataType) -> pa.Array:
        if isinstance(data, (ColorModel, int, str)):
            data = [data]

        pa_data = [ColorModel.auto(v).value if v is not None else None for v in data]  # type: ignore[redundant-expr]

        return pa.array(pa_data, type=data_type)
