# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/datatypes/mat3x3.fbs".

# You can extend this class by creating a "Mat3x3Ext" class in "mat3x3_ext.py".

from __future__ import annotations

from typing import TYPE_CHECKING, Any, Sequence, Union

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
from .mat3x3_ext import Mat3x3Ext

__all__ = ["Mat3x3", "Mat3x3ArrayLike", "Mat3x3Batch", "Mat3x3Like"]


@define(init=False)
class Mat3x3(Mat3x3Ext):
    """
    **Datatype**: A 3x3 Matrix.

    Matrices in Rerun are stored as flat list of coefficients in column-major order:
    ```text
                column 0       column 1       column 2
           -------------------------------------------------
    row 0 | flat_columns[0] flat_columns[3] flat_columns[6]
    row 1 | flat_columns[1] flat_columns[4] flat_columns[7]
    row 2 | flat_columns[2] flat_columns[5] flat_columns[8]
    ```

    However, construction is done from a list of rows, which follows NumPy's convention:
    ```python
    np.testing.assert_array_equal(
        rr.datatypes.Mat3x3([1, 2, 3, 4, 5, 6, 7, 8, 9]).flat_columns, np.array([1, 4, 7, 2, 5, 8, 3, 6, 9], dtype=np.float32)
    )
    np.testing.assert_array_equal(
        rr.datatypes.Mat3x3([[1, 2, 3], [4, 5, 6], [7, 8, 9]]).flat_columns,
        np.array([1, 4, 7, 2, 5, 8, 3, 6, 9], dtype=np.float32),
    )
    ```
    If you want to construct a matrix from a list of columns instead, use the named `columns` parameter:
    ```python
    np.testing.assert_array_equal(
        rr.datatypes.Mat3x3(columns=[1, 2, 3, 4, 5, 6, 7, 8, 9]).flat_columns,
        np.array([1, 2, 3, 4, 5, 6, 7, 8, 9], dtype=np.float32),
    )
    np.testing.assert_array_equal(
        rr.datatypes.Mat3x3(columns=[[1, 2, 3], [4, 5, 6], [7, 8, 9]]).flat_columns,
        np.array([1, 2, 3, 4, 5, 6, 7, 8, 9], dtype=np.float32),
    )
    ```
    """

    # __init__ can be found in mat3x3_ext.py

    flat_columns: npt.NDArray[np.float32] = field(converter=to_np_float32)
    # Flat list of matrix coefficients in column-major order.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    def __array__(self, dtype: npt.DTypeLike = None, copy: bool | None = None) -> npt.NDArray[Any]:
        # You can define your own __array__ function as a member of Mat3x3Ext in mat3x3_ext.py
        return np.asarray(self.flat_columns, dtype=dtype, copy=copy)


if TYPE_CHECKING:
    Mat3x3Like = Union[Mat3x3, npt.ArrayLike]
else:
    Mat3x3Like = Any

Mat3x3ArrayLike = Union[Mat3x3, Sequence[Mat3x3Like], npt.ArrayLike]


class Mat3x3Batch(BaseBatch[Mat3x3ArrayLike]):
    _ARROW_DATATYPE = pa.list_(pa.field("item", pa.float32(), nullable=False, metadata={}), 9)

    @staticmethod
    def _native_to_pa_array(data: Mat3x3ArrayLike, data_type: pa.DataType) -> pa.Array:
        return Mat3x3Ext.native_to_pa_array_override(data, data_type)
