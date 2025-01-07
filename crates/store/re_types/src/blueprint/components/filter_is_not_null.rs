// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/components/filter_is_not_null.fbs".

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

/// **Component**: Configuration for the filter is not null feature of the dataframe view.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct FilterIsNotNull(pub crate::blueprint::datatypes::FilterIsNotNull);

impl ::re_types_core::Component for FilterIsNotNull {
    #[inline]
    fn descriptor() -> ComponentDescriptor {
        ComponentDescriptor::new("rerun.blueprint.components.FilterIsNotNull")
    }
}

::re_types_core::macros::impl_into_cow!(FilterIsNotNull);

impl ::re_types_core::Loggable for FilterIsNotNull {
    #[inline]
    fn arrow_datatype() -> arrow::datatypes::DataType {
        crate::blueprint::datatypes::FilterIsNotNull::arrow_datatype()
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<arrow::array::ArrayRef>
    where
        Self: Clone + 'a,
    {
        crate::blueprint::datatypes::FilterIsNotNull::to_arrow_opt(data.into_iter().map(|datum| {
            datum.map(|datum| match datum.into() {
                ::std::borrow::Cow::Borrowed(datum) => ::std::borrow::Cow::Borrowed(&datum.0),
                ::std::borrow::Cow::Owned(datum) => ::std::borrow::Cow::Owned(datum.0),
            })
        }))
    }

    fn from_arrow_opt(
        arrow_data: &dyn arrow::array::Array,
    ) -> DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        crate::blueprint::datatypes::FilterIsNotNull::from_arrow_opt(arrow_data)
            .map(|v| v.into_iter().map(|v| v.map(Self)).collect())
    }
}

impl<T: Into<crate::blueprint::datatypes::FilterIsNotNull>> From<T> for FilterIsNotNull {
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl std::borrow::Borrow<crate::blueprint::datatypes::FilterIsNotNull> for FilterIsNotNull {
    #[inline]
    fn borrow(&self) -> &crate::blueprint::datatypes::FilterIsNotNull {
        &self.0
    }
}

impl std::ops::Deref for FilterIsNotNull {
    type Target = crate::blueprint::datatypes::FilterIsNotNull;

    #[inline]
    fn deref(&self) -> &crate::blueprint::datatypes::FilterIsNotNull {
        &self.0
    }
}

impl std::ops::DerefMut for FilterIsNotNull {
    #[inline]
    fn deref_mut(&mut self) -> &mut crate::blueprint::datatypes::FilterIsNotNull {
        &mut self.0
    }
}

impl ::re_byte_size::SizeBytes for FilterIsNotNull {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.0.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::blueprint::datatypes::FilterIsNotNull>::is_pod()
    }
}
