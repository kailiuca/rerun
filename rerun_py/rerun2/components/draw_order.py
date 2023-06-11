# NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Optional, Sequence, Union

import numpy as np
import numpy.typing as npt
import pyarrow as pa


@dataclass
class DrawOrder:
    """
    Draw order used for the display order of 2D elements.

    Higher values are drawn on top of lower values.
    An entity can have only a single draw order component.
    Within an entity draw order is governed by the order of the components.

    Draw order for entities with the same draw order is generally undefined.
    """

    value: float

    def __array__(self):
        return np.asarray(self.value)


DrawOrderLike = Union[DrawOrder, float]

DrawOrderArrayLike = Union[DrawOrderLike, Sequence[DrawOrderLike], npt.NDArray[np.float32]]


# --- Arrow support ---

from rerun2.components.draw_order_ext import DrawOrderArrayExt  # noqa: E402


class DrawOrderType(pa.ExtensionType):
    def __init__(self: type[pa.ExtensionType]) -> None:
        pa.ExtensionType.__init__(self, pa.float32(), "rerun.components.DrawOrder")

    def __arrow_ext_serialize__(self: type[pa.ExtensionType]) -> bytes:
        # since we don't have a parameterized type, we don't need extra metadata to be deserialized
        return b""

    @classmethod
    def __arrow_ext_deserialize__(
        cls: type[pa.ExtensionType], storage_type: Any, serialized: Any
    ) -> type[pa.ExtensionType]:
        # return an instance of this subclass given the serialized metadata.
        return DrawOrderType()

    def __arrow_ext_class__(self: type[pa.ExtensionType]) -> type[pa.ExtensionArray]:
        return DrawOrderArray


pa.register_extension_type(DrawOrderType())


class DrawOrderArray(pa.ExtensionArray, DrawOrderArrayExt):  # type: ignore[misc]
    @staticmethod
    def from_similar(data: Optional[DrawOrderArrayLike]):
        if data is None:
            return DrawOrderType().wrap_array(pa.array([], type=DrawOrderType().storage_type))
        else:
            return DrawOrderArrayExt.from_similar(
                data,
                mono=DrawOrder,
                mono_aliases=DrawOrderLike,
                many=DrawOrderArray,
                many_aliases=DrawOrderArrayLike,
                arrow=DrawOrderType,
            )
