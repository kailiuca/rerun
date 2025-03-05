# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/datatypes/vec2d.fbs".

# You can extend this class by creating a "Vec2DExt" class in "vec2d_ext.py".

from __future__ import annotations

from collections.abc import Sequence
from typing import TYPE_CHECKING, Any, Union

import numpy as np
import numpy.typing as npt
import pyarrow as pa
from attrs import define, field

from .._baseclasses import (
    BaseBatch,
)
from .._converters import (
    to_np_float32,
)
from .vec2d_ext import Vec2DExt

__all__ = ["Vec2D", "Vec2DArrayLike", "Vec2DBatch", "Vec2DLike"]


@define(init=False)
class Vec2D(Vec2DExt):
    """**Datatype**: A vector in 2D space."""

    def __init__(self: Any, xy: Vec2DLike):
        """Create a new instance of the Vec2D datatype."""

        # You can define your own __init__ function as a member of Vec2DExt in vec2d_ext.py
        self.__attrs_init__(xy=xy)

    xy: npt.NDArray[np.float32] = field(converter=to_np_float32)

    def __array__(self, dtype: npt.DTypeLike = None, copy: bool | None = None) -> npt.NDArray[Any]:
        # You can define your own __array__ function as a member of Vec2DExt in vec2d_ext.py
        return np.asarray(self.xy, dtype=dtype, copy=copy)


if TYPE_CHECKING:
    Vec2DLike = Union[Vec2D, npt.NDArray[Any], npt.ArrayLike, Sequence[float]]
else:
    Vec2DLike = Any

Vec2DArrayLike = Union[
    Vec2D, Sequence[Vec2DLike], npt.NDArray[Any], npt.ArrayLike, Sequence[Sequence[float]], Sequence[float]
]


class Vec2DBatch(BaseBatch[Vec2DArrayLike]):
    _ARROW_DATATYPE = pa.list_(pa.field("item", pa.float32(), nullable=False, metadata={}), 2)

    @staticmethod
    def _native_to_pa_array(data: Vec2DArrayLike, data_type: pa.DataType) -> pa.Array:
        return Vec2DExt.native_to_pa_array_override(data, data_type)
