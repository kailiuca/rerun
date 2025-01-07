//! Example components to be used for tests and docs

use std::sync::Arc;

use re_byte_size::SizeBytes;
use re_types_core::{Component, ComponentDescriptor, DeserializationError, Loggable};

// ----------------------------------------------------------------------------

#[derive(Debug)]
pub struct MyPoints;

impl MyPoints {
    pub const NUM_COMPONENTS: usize = 5;
}

impl re_types_core::Archetype for MyPoints {
    type Indicator = re_types_core::GenericIndicatorComponent<Self>;

    fn name() -> re_types_core::ArchetypeName {
        "example.MyPoints".into()
    }

    fn display_name() -> &'static str {
        "MyPoints"
    }

    fn required_components() -> ::std::borrow::Cow<'static, [re_types_core::ComponentDescriptor]> {
        vec![MyPoint::descriptor()].into()
    }

    fn recommended_components() -> std::borrow::Cow<'static, [re_types_core::ComponentDescriptor]> {
        vec![
            re_types_core::ComponentBatch::descriptor(&Self::Indicator::default()).into_owned(),
            MyColor::descriptor(),
            MyLabel::descriptor(),
        ]
        .into()
    }
}

// ----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct MyPoint {
    pub x: f32,
    pub y: f32,
}

impl MyPoint {
    #[allow(clippy::should_implement_trait)]
    #[inline]
    pub fn from_iter(it: impl IntoIterator<Item = u32>) -> Vec<Self> {
        it.into_iter()
            .map(|i| Self::new(i as f32, i as f32))
            .collect()
    }
}

impl MyPoint {
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

re_types_core::macros::impl_into_cow!(MyPoint);

impl SizeBytes for MyPoint {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        let Self { x: _, y: _ } = self;
        0
    }
}

impl Loggable for MyPoint {
    fn arrow_datatype() -> arrow::datatypes::DataType {
        use arrow::datatypes::DataType::Float32;
        arrow::datatypes::DataType::Struct(arrow::datatypes::Fields::from(vec![
            arrow::datatypes::Field::new("x", Float32, false),
            arrow::datatypes::Field::new("y", Float32, false),
        ]))
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<std::borrow::Cow<'a, Self>>>>,
    ) -> re_types_core::SerializationResult<arrow::array::ArrayRef>
    where
        Self: 'a,
    {
        use arrow::datatypes::DataType::Float32;

        let (xs, ys): (Vec<_>, Vec<_>) = data
            .into_iter()
            .map(Option::unwrap)
            .map(Into::into)
            .map(|p| (p.x, p.y))
            .unzip();

        let x_array = Arc::new(arrow::array::Float32Array::from(xs));
        let y_array = Arc::new(arrow::array::Float32Array::from(ys));

        Ok(Arc::new(arrow::array::StructArray::new(
            arrow::datatypes::Fields::from(vec![
                arrow::datatypes::Field::new("x", Float32, false),
                arrow::datatypes::Field::new("y", Float32, false),
            ]),
            vec![x_array, y_array],
            None,
        )))
    }

    fn from_arrow_opt(
        data: &dyn arrow::array::Array,
    ) -> re_types_core::DeserializationResult<Vec<Option<Self>>> {
        let array = data
            .as_any()
            .downcast_ref::<arrow::array::StructArray>()
            .ok_or(DeserializationError::downcast_error::<
                arrow::array::StructArray,
            >())?;

        let x_array = array.columns()[0].as_ref();
        let y_array = array.columns()[1].as_ref();

        let xs = x_array
            .as_any()
            .downcast_ref::<arrow::array::Float32Array>()
            .ok_or(DeserializationError::downcast_error::<
                arrow::array::Float32Array,
            >())?;
        let ys = y_array
            .as_any()
            .downcast_ref::<arrow::array::Float32Array>()
            .ok_or(DeserializationError::downcast_error::<
                arrow::array::Float32Array,
            >())?;

        Ok(xs
            .iter()
            .zip(ys.iter())
            .map(|(x, y)| {
                if let (Some(x), Some(y)) = (x, y) {
                    Some(Self { x, y })
                } else {
                    None
                }
            })
            .collect())
    }
}

impl Component for MyPoint {
    fn descriptor() -> ComponentDescriptor {
        ComponentDescriptor::new("example.MyPoint")
    }
}

// ----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct MyPoint64 {
    pub x: f64,
    pub y: f64,
}

impl MyPoint64 {
    #[allow(clippy::should_implement_trait)]
    #[inline]
    pub fn from_iter(it: impl IntoIterator<Item = u64>) -> Vec<Self> {
        it.into_iter()
            .map(|i| Self::new(i as f64, i as f64))
            .collect()
    }
}

impl MyPoint64 {
    #[inline]
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

re_types_core::macros::impl_into_cow!(MyPoint64);

impl SizeBytes for MyPoint64 {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        let Self { x: _, y: _ } = self;
        0
    }
}

impl Loggable for MyPoint64 {
    fn arrow_datatype() -> arrow::datatypes::DataType {
        use arrow::datatypes::DataType::Float64;
        arrow::datatypes::DataType::Struct(arrow::datatypes::Fields::from(vec![
            arrow::datatypes::Field::new("x", Float64, false),
            arrow::datatypes::Field::new("y", Float64, false),
        ]))
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<std::borrow::Cow<'a, Self>>>>,
    ) -> re_types_core::SerializationResult<arrow::array::ArrayRef>
    where
        Self: 'a,
    {
        use arrow::datatypes::DataType::Float64;

        let (xs, ys): (Vec<_>, Vec<_>) = data
            .into_iter()
            .map(Option::unwrap)
            .map(Into::into)
            .map(|p| (p.x, p.y))
            .unzip();

        let x_array = Arc::new(arrow::array::Float64Array::from(xs));
        let y_array = Arc::new(arrow::array::Float64Array::from(ys));

        Ok(Arc::new(arrow::array::StructArray::new(
            arrow::datatypes::Fields::from(vec![
                arrow::datatypes::Field::new("x", Float64, false),
                arrow::datatypes::Field::new("y", Float64, false),
            ]),
            vec![x_array, y_array],
            None,
        )))
    }

    fn from_arrow_opt(
        data: &dyn arrow::array::Array,
    ) -> re_types_core::DeserializationResult<Vec<Option<Self>>> {
        let array = data
            .as_any()
            .downcast_ref::<arrow::array::StructArray>()
            .ok_or(DeserializationError::downcast_error::<
                arrow::array::StructArray,
            >())?;

        let x_array = array.columns()[0].as_ref();
        let y_array = array.columns()[1].as_ref();

        let xs = x_array
            .as_any()
            .downcast_ref::<arrow::array::Float64Array>()
            .ok_or(DeserializationError::downcast_error::<
                arrow::array::Float64Array,
            >())?;
        let ys = y_array
            .as_any()
            .downcast_ref::<arrow::array::Float64Array>()
            .ok_or(DeserializationError::downcast_error::<
                arrow::array::Float64Array,
            >())?;

        Ok(xs
            .iter()
            .zip(ys.iter())
            .map(|(x, y)| {
                if let (Some(x), Some(y)) = (x, y) {
                    Some(Self { x, y })
                } else {
                    None
                }
            })
            .collect())
    }
}

impl Component for MyPoint64 {
    fn descriptor() -> ComponentDescriptor {
        ComponentDescriptor::new("example.MyPoint64")
    }
}

// ----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(transparent)]
pub struct MyColor(pub u32);

impl MyColor {
    #[allow(clippy::should_implement_trait)]
    #[inline]
    pub fn from_iter(it: impl IntoIterator<Item = u32>) -> Vec<Self> {
        it.into_iter().map(Self).collect()
    }
}

impl MyColor {
    #[inline]
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self(u32::from_le_bytes([r, g, b, 255]))
    }
}

impl From<u32> for MyColor {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}

re_types_core::macros::impl_into_cow!(MyColor);

impl SizeBytes for MyColor {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        let Self(_) = self;
        0
    }
}

impl Loggable for MyColor {
    fn arrow_datatype() -> arrow::datatypes::DataType {
        arrow::datatypes::DataType::UInt32
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<std::borrow::Cow<'a, Self>>>>,
    ) -> re_types_core::SerializationResult<arrow::array::ArrayRef>
    where
        Self: 'a,
    {
        use re_types_core::datatypes::UInt32;
        UInt32::to_arrow_opt(
            data.into_iter()
                .map(|opt| opt.map(Into::into).map(|c| UInt32(c.0))),
        )
    }

    fn from_arrow_opt(
        data: &dyn arrow::array::Array,
    ) -> re_types_core::DeserializationResult<Vec<Option<Self>>> {
        use re_types_core::datatypes::UInt32;
        Ok(UInt32::from_arrow_opt(data)?
            .into_iter()
            .map(|opt| opt.map(|v| Self(v.0)))
            .collect())
    }
}

impl Component for MyColor {
    fn descriptor() -> ComponentDescriptor {
        ComponentDescriptor::new("example.MyColor")
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct MyLabel(pub String);

re_types_core::macros::impl_into_cow!(MyLabel);

impl SizeBytes for MyLabel {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        let Self(s) = self;
        s.heap_size_bytes()
    }
}

impl Loggable for MyLabel {
    fn arrow_datatype() -> arrow::datatypes::DataType {
        re_types_core::datatypes::Utf8::arrow_datatype()
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<std::borrow::Cow<'a, Self>>>>,
    ) -> re_types_core::SerializationResult<arrow::array::ArrayRef>
    where
        Self: 'a,
    {
        use re_types_core::datatypes::Utf8;
        Utf8::to_arrow_opt(
            data.into_iter()
                .map(|opt| opt.map(Into::into).map(|l| Utf8(l.0.clone().into()))),
        )
    }

    fn from_arrow_opt(
        data: &dyn arrow::array::Array,
    ) -> re_types_core::DeserializationResult<Vec<Option<Self>>> {
        use re_types_core::datatypes::Utf8;
        Ok(Utf8::from_arrow_opt(data)?
            .into_iter()
            .map(|opt| opt.map(|v| Self(v.0.to_string())))
            .collect())
    }
}

impl Component for MyLabel {
    fn descriptor() -> ComponentDescriptor {
        ComponentDescriptor::new("example.MyLabel")
    }
}

// ----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(transparent)]
pub struct MyIndex(pub u64);

impl MyIndex {
    #[allow(clippy::should_implement_trait)]
    #[inline]
    pub fn from_iter(it: impl IntoIterator<Item = u64>) -> Vec<Self> {
        it.into_iter().map(Self).collect()
    }
}

re_types_core::macros::impl_into_cow!(MyIndex);

impl SizeBytes for MyIndex {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        let Self(_) = self;
        0
    }
}

impl Loggable for MyIndex {
    fn arrow_datatype() -> arrow::datatypes::DataType {
        arrow::datatypes::DataType::UInt64
    }

    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<std::borrow::Cow<'a, Self>>>>,
    ) -> re_types_core::SerializationResult<arrow::array::ArrayRef>
    where
        Self: 'a,
    {
        use re_types_core::datatypes::UInt64;
        UInt64::to_arrow_opt(
            data.into_iter()
                .map(|opt| opt.map(Into::into).map(|c| UInt64(c.0))),
        )
    }

    fn from_arrow_opt(
        data: &dyn arrow::array::Array,
    ) -> re_types_core::DeserializationResult<Vec<Option<Self>>> {
        use re_types_core::datatypes::UInt64;
        Ok(UInt64::from_arrow_opt(data)?
            .into_iter()
            .map(|opt| opt.map(|v| Self(v.0)))
            .collect())
    }
}

impl Component for MyIndex {
    fn descriptor() -> re_types_core::ComponentDescriptor {
        ComponentDescriptor::new("example.MyIndex")
    }
}
