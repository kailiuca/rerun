// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/datatypes/view_coordinates.fbs".

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::external::arrow;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, ComponentBatchCowWithDescriptor};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Datatype**: How we interpret the coordinate system of an entity/space.
///
/// For instance: What is "up"? What does the Z axis mean?
///
/// The three coordinates are always ordered as [x, y, z].
///
/// For example [Right, Down, Forward] means that the X axis points to the right, the Y axis points
/// down, and the Z axis points forward.
///
/// ⚠ [Rerun does not yet support left-handed coordinate systems](https://github.com/rerun-io/rerun/issues/5032).
///
/// The following constants are used to represent the different directions:
///  * Up = 1
///  * Down = 2
///  * Right = 3
///  * Left = 4
///  * Forward = 5
///  * Back = 6
#[derive(Clone, Debug, Copy, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(transparent)]
pub struct ViewCoordinates(
    /// The directions of the [x, y, z] axes.
    pub [u8; 3usize],
);

::re_types_core::macros::impl_into_cow!(ViewCoordinates);

impl ::re_types_core::Loggable for ViewCoordinates {
    #[inline]
    fn arrow_datatype() -> arrow::datatypes::DataType {
        #![allow(clippy::wildcard_imports)]
        use arrow::datatypes::*;
        DataType::FixedSizeList(
            std::sync::Arc::new(Field::new("item", DataType::UInt8, false)),
            3,
        )
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<arrow::array::ArrayRef>
    where
        Self: Clone + 'a,
    {
        #![allow(clippy::wildcard_imports)]
        #![allow(clippy::manual_is_variant_and)]
        use ::re_types_core::{arrow_helpers::as_array_ref, Loggable as _, ResultExt as _};
        use arrow::{array::*, buffer::*, datatypes::*};
        Ok({
            let (somes, data0): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    let datum = datum.map(|datum| datum.into_owned().0);
                    (datum.is_some(), datum)
                })
                .unzip();
            let data0_validity: Option<arrow::buffer::NullBuffer> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            {
                let data0_inner_data: Vec<_> = data0
                    .into_iter()
                    .flat_map(|v| match v {
                        Some(v) => itertools::Either::Left(v.into_iter()),
                        None => itertools::Either::Right(
                            std::iter::repeat(Default::default()).take(3usize),
                        ),
                    })
                    .collect();
                let data0_inner_validity: Option<arrow::buffer::NullBuffer> =
                    data0_validity.as_ref().map(|validity| {
                        validity
                            .iter()
                            .map(|b| std::iter::repeat(b).take(3usize))
                            .flatten()
                            .collect::<Vec<_>>()
                            .into()
                    });
                as_array_ref(FixedSizeListArray::new(
                    std::sync::Arc::new(Field::new("item", DataType::UInt8, false)),
                    3,
                    as_array_ref(PrimitiveArray::<UInt8Type>::new(
                        ScalarBuffer::from(data0_inner_data.into_iter().collect::<Vec<_>>()),
                        data0_inner_validity,
                    )),
                    data0_validity,
                ))
            }
        })
    }

    fn from_arrow_opt(
        arrow_data: &dyn arrow::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        #![allow(clippy::wildcard_imports)]
        use ::re_types_core::{arrow_zip_validity::ZipValidity, Loggable as _, ResultExt as _};
        use arrow::{array::*, buffer::*, datatypes::*};
        Ok({
            let arrow_data = arrow_data
                .as_any()
                .downcast_ref::<arrow::array::FixedSizeListArray>()
                .ok_or_else(|| {
                    let expected = Self::arrow_datatype();
                    let actual = arrow_data.data_type().clone();
                    DeserializationError::datatype_mismatch(expected, actual)
                })
                .with_context("rerun.datatypes.ViewCoordinates#coordinates")?;
            if arrow_data.is_empty() {
                Vec::new()
            } else {
                let offsets = (0..)
                    .step_by(3usize)
                    .zip((3usize..).step_by(3usize).take(arrow_data.len()));
                let arrow_data_inner = {
                    let arrow_data_inner = &**arrow_data.values();
                    arrow_data_inner
                        .as_any()
                        .downcast_ref::<UInt8Array>()
                        .ok_or_else(|| {
                            let expected = DataType::UInt8;
                            let actual = arrow_data_inner.data_type().clone();
                            DeserializationError::datatype_mismatch(expected, actual)
                        })
                        .with_context("rerun.datatypes.ViewCoordinates#coordinates")?
                        .into_iter()
                        .collect::<Vec<_>>()
                };
                ZipValidity::new_with_validity(offsets, arrow_data.nulls())
                    .map(|elem| {
                        elem.map(|(start, end): (usize, usize)| {
                            debug_assert!(end - start == 3usize);
                            if arrow_data_inner.len() < end {
                                return Err(DeserializationError::offset_slice_oob(
                                    (start, end),
                                    arrow_data_inner.len(),
                                ));
                            }

                            #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
                            let data = unsafe { arrow_data_inner.get_unchecked(start..end) };
                            let data = data.iter().cloned().map(Option::unwrap_or_default);

                            // NOTE: Unwrapping cannot fail: the length must be correct.
                            #[allow(clippy::unwrap_used)]
                            Ok(array_init::from_iter(data).unwrap())
                        })
                        .transpose()
                    })
                    .collect::<DeserializationResult<Vec<Option<_>>>>()?
            }
            .into_iter()
        }
        .map(|v| v.ok_or_else(DeserializationError::missing_data))
        .map(|res| res.map(|v| Some(Self(v))))
        .collect::<DeserializationResult<Vec<Option<_>>>>()
        .with_context("rerun.datatypes.ViewCoordinates#coordinates")
        .with_context("rerun.datatypes.ViewCoordinates")?)
    }

    #[inline]
    fn from_arrow(arrow_data: &dyn arrow::array::Array) -> DeserializationResult<Vec<Self>>
    where
        Self: Sized,
    {
        #![allow(clippy::wildcard_imports)]
        use ::re_types_core::{arrow_zip_validity::ZipValidity, Loggable as _, ResultExt as _};
        use arrow::{array::*, buffer::*, datatypes::*};
        if let Some(nulls) = arrow_data.nulls() {
            if nulls.null_count() != 0 {
                return Err(DeserializationError::missing_data());
            }
        }
        Ok({
            let slice = {
                let arrow_data = arrow_data
                    .as_any()
                    .downcast_ref::<arrow::array::FixedSizeListArray>()
                    .ok_or_else(|| {
                        let expected = DataType::FixedSizeList(
                            std::sync::Arc::new(Field::new("item", DataType::UInt8, false)),
                            3,
                        );
                        let actual = arrow_data.data_type().clone();
                        DeserializationError::datatype_mismatch(expected, actual)
                    })
                    .with_context("rerun.datatypes.ViewCoordinates#coordinates")?;
                let arrow_data_inner = &**arrow_data.values();
                bytemuck::cast_slice::<_, [_; 3usize]>(
                    arrow_data_inner
                        .as_any()
                        .downcast_ref::<UInt8Array>()
                        .ok_or_else(|| {
                            let expected = DataType::UInt8;
                            let actual = arrow_data_inner.data_type().clone();
                            DeserializationError::datatype_mismatch(expected, actual)
                        })
                        .with_context("rerun.datatypes.ViewCoordinates#coordinates")?
                        .values()
                        .as_ref(),
                )
            };
            {
                slice.iter().copied().map(Self).collect::<Vec<_>>()
            }
        })
    }
}

impl From<[u8; 3usize]> for ViewCoordinates {
    #[inline]
    fn from(coordinates: [u8; 3usize]) -> Self {
        Self(coordinates)
    }
}

impl From<ViewCoordinates> for [u8; 3usize] {
    #[inline]
    fn from(value: ViewCoordinates) -> Self {
        value.0
    }
}

impl ::re_byte_size::SizeBytes for ViewCoordinates {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.0.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <[u8; 3usize]>::is_pod()
    }
}
