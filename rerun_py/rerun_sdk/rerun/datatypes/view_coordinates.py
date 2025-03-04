# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/datatypes/view_coordinates.fbs".

# You can extend this class by creating a "ViewCoordinatesExt" class in "view_coordinates_ext.py".

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
    to_np_uint8,
)
from .view_coordinates_ext import ViewCoordinatesExt

__all__ = ["ViewCoordinates", "ViewCoordinatesArrayLike", "ViewCoordinatesBatch", "ViewCoordinatesLike"]


@define(init=False)
class ViewCoordinates(ViewCoordinatesExt):
    """
    **Datatype**: How we interpret the coordinate system of an entity/space.

    For instance: What is "up"? What does the Z axis mean?

    The three coordinates are always ordered as [x, y, z].

    For example [Right, Down, Forward] means that the X axis points to the right, the Y axis points
    down, and the Z axis points forward.

    ⚠ [Rerun does not yet support left-handed coordinate systems](https://github.com/rerun-io/rerun/issues/5032).

    The following constants are used to represent the different directions:
     * Up = 1
     * Down = 2
     * Right = 3
     * Left = 4
     * Forward = 5
     * Back = 6
    """

    def __init__(self: Any, coordinates: ViewCoordinatesLike):
        """
        Create a new instance of the ViewCoordinates datatype.

        Parameters
        ----------
        coordinates:
            The directions of the [x, y, z] axes.

        """

        # You can define your own __init__ function as a member of ViewCoordinatesExt in view_coordinates_ext.py
        self.__attrs_init__(coordinates=coordinates)

    coordinates: npt.NDArray[np.uint8] = field(converter=to_np_uint8)
    # The directions of the [x, y, z] axes.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    def __array__(self, dtype: npt.DTypeLike = None, copy: bool | None = None) -> npt.NDArray[Any]:
        # You can define your own __array__ function as a member of ViewCoordinatesExt in view_coordinates_ext.py
        return np.asarray(self.coordinates, dtype=dtype, copy=copy)


if TYPE_CHECKING:
    ViewCoordinatesLike = Union[ViewCoordinates, npt.ArrayLike]
else:
    ViewCoordinatesLike = Any

ViewCoordinatesArrayLike = Union[ViewCoordinates, Sequence[ViewCoordinatesLike], npt.ArrayLike]


class ViewCoordinatesBatch(BaseBatch[ViewCoordinatesArrayLike]):
    _ARROW_DATATYPE = pa.list_(pa.field("item", pa.uint8(), nullable=False, metadata={}), 3)

    @staticmethod
    def _native_to_pa_array(data: ViewCoordinatesArrayLike, data_type: pa.DataType) -> pa.Array:
        return ViewCoordinatesExt.native_to_pa_array_override(data, data_type)
