// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/components/apply_latest_at.fbs".

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

use ::re_types_core::try_serialize_field;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, SerializedComponentBatch};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Component**: Whether empty cells in a dataframe should be filled with a latest-at query.
#[derive(Clone, Debug, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct ApplyLatestAt(pub crate::datatypes::Bool);

impl ::re_types_core::Component for ApplyLatestAt {
    #[inline]
    fn descriptor() -> ComponentDescriptor {
        ComponentDescriptor::new("rerun.blueprint.components.ApplyLatestAt")
    }
}

::re_types_core::macros::impl_into_cow!(ApplyLatestAt);

impl ::re_types_core::Loggable for ApplyLatestAt {
    #[inline]
    fn arrow_datatype() -> arrow::datatypes::DataType {
        crate::datatypes::Bool::arrow_datatype()
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> SerializationResult<arrow::array::ArrayRef>
    where
        Self: Clone + 'a,
    {
        crate::datatypes::Bool::to_arrow_opt(data.into_iter().map(|datum| {
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
        crate::datatypes::Bool::from_arrow_opt(arrow_data)
            .map(|v| v.into_iter().map(|v| v.map(Self)).collect())
    }
}

impl<T: Into<crate::datatypes::Bool>> From<T> for ApplyLatestAt {
    fn from(v: T) -> Self {
        Self(v.into())
    }
}

impl std::borrow::Borrow<crate::datatypes::Bool> for ApplyLatestAt {
    #[inline]
    fn borrow(&self) -> &crate::datatypes::Bool {
        &self.0
    }
}

impl std::ops::Deref for ApplyLatestAt {
    type Target = crate::datatypes::Bool;

    #[inline]
    fn deref(&self) -> &crate::datatypes::Bool {
        &self.0
    }
}

impl std::ops::DerefMut for ApplyLatestAt {
    #[inline]
    fn deref_mut(&mut self) -> &mut crate::datatypes::Bool {
        &mut self.0
    }
}

impl ::re_byte_size::SizeBytes for ApplyLatestAt {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.0.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::datatypes::Bool>::is_pod()
    }
}
