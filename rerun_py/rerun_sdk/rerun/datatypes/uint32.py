# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/datatypes/uint32.fbs".

# You can extend this class by creating a "UInt32Ext" class in "uint32_ext.py".

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

__all__ = ["UInt32", "UInt32ArrayLike", "UInt32Batch", "UInt32Like"]


@define(init=False)
class UInt32:
    """**Datatype**: A 32bit unsigned integer."""

    def __init__(self: Any, value: UInt32Like):
        """Create a new instance of the UInt32 datatype."""

        # You can define your own __init__ function as a member of UInt32Ext in uint32_ext.py
        self.__attrs_init__(value=value)

    value: int = field(converter=int)

    def __array__(self, dtype: npt.DTypeLike = None, copy: bool | None = None) -> npt.NDArray[Any]:
        # You can define your own __array__ function as a member of UInt32Ext in uint32_ext.py
        return np.asarray(self.value, dtype=dtype, copy=copy)

    def __int__(self) -> int:
        return int(self.value)

    def __hash__(self) -> int:
        return hash(self.value)


if TYPE_CHECKING:
    UInt32Like = Union[UInt32, int]
else:
    UInt32Like = Any

UInt32ArrayLike = Union[UInt32, Sequence[UInt32Like], int, npt.NDArray[np.uint32]]


class UInt32Batch(BaseBatch[UInt32ArrayLike]):
    _ARROW_DATATYPE = pa.uint32()

    @staticmethod
    def _native_to_pa_array(data: UInt32ArrayLike, data_type: pa.DataType) -> pa.Array:
        array = np.asarray(data, dtype=np.uint32).flatten()
        return pa.array(array, type=data_type)
