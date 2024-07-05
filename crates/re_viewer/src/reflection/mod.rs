// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/reflection.rs

#![allow(clippy::too_many_lines)]
#![allow(clippy::wildcard_imports)]
#![allow(unused_imports)]
use re_types::blueprint::components::*;
use re_types::components::*;
use re_types_blueprint::blueprint::components::*;
use re_types_core::components::*;
use re_types_core::{
    reflection::{
        ArchetypeFieldReflection, ArchetypeReflection, ArchetypeReflectionMap, ComponentReflection,
        ComponentReflectionMap, Reflection,
    },
    ArchetypeName, ComponentName, Loggable, LoggableBatch as _, SerializationError,
};

/// Generates reflection about all known components.
///
/// Call only once and reuse the results.

pub fn generate_reflection() -> Result<Reflection, SerializationError> {
    re_tracing::profile_function!();
    Ok(Reflection {
        components: generate_component_reflection()?,
        archetypes: generate_archetype_reflection(),
    })
}

/// Generates reflection about all known components.
///
/// Call only once and reuse the results.

fn generate_component_reflection() -> Result<ComponentReflectionMap, SerializationError> {
    re_tracing::profile_function!();
    let array = [
        (
            <ActiveTab as Loggable>::name(),
            ComponentReflection {
                docstring_md: "The active tab in a tabbed container.",
                placeholder: Some(ActiveTab::default().to_arrow()?),
            },
        ),
        (
            <AutoLayout as Loggable>::name(),
            ComponentReflection {
                docstring_md: "Whether the viewport layout is determined automatically.",
                placeholder: Some(AutoLayout::default().to_arrow()?),
            },
        ),
        (
            <AutoSpaceViews as Loggable>::name(),
            ComponentReflection {
                docstring_md: "Whether or not space views should be created automatically.",
                placeholder: Some(AutoSpaceViews::default().to_arrow()?),
            },
        ),
        (
            <BackgroundKind as Loggable>::name(),
            ComponentReflection {
                docstring_md: "The type of the background in a view.",
                placeholder: Some(BackgroundKind::default().to_arrow()?),
            },
        ),
        (
            <ColumnShare as Loggable>::name(),
            ComponentReflection {
                docstring_md: "The layout share of a column in the container.",
                placeholder: Some(ColumnShare::default().to_arrow()?),
            },
        ),
        (
            <ContainerKind as Loggable>::name(),
            ComponentReflection {
                docstring_md: "The kind of a blueprint container (tabs, grid, …).",
                placeholder: Some(ContainerKind::default().to_arrow()?),
            },
        ),
        (
            <Corner2D as Loggable>::name(),
            ComponentReflection {
                docstring_md: "One of four 2D corners, typically used to align objects.",
                placeholder: Some(Corner2D::default().to_arrow()?),
            },
        ),
        (
            <GridColumns as Loggable>::name(),
            ComponentReflection {
                docstring_md: "How many columns a grid container should have.",
                placeholder: Some(GridColumns::default().to_arrow()?),
            },
        ),
        (
            <IncludedContent as Loggable>::name(),
            ComponentReflection {
                docstring_md: "All the contents in the container.",
                placeholder: Some(IncludedContent::default().to_arrow()?),
            },
        ),
        (
            <IncludedSpaceView as Loggable>::name(),
            ComponentReflection {
                docstring_md: "The unique id of a space view, used to refer to views in containers.",
                placeholder: Some(IncludedSpaceView::default().to_arrow()?),
            },
        ),
        (
            <Interactive as Loggable>::name(),
            ComponentReflection {
                docstring_md: "Whether the entity can be interacted with.\n\nNon interactive components are still visible, but mouse interactions in the view are disabled.",
                placeholder: Some(Interactive::default().to_arrow()?),
            },
        ),
        (
            <LockRangeDuringZoom as Loggable>::name(),
            ComponentReflection {
                docstring_md: "Indicate whether the range should be locked when zooming in on the data.\n\nDefault is `false`, i.e. zoom will change the visualized range.",
                placeholder: Some(LockRangeDuringZoom::default().to_arrow()?),
            },
        ),
        (
            <PanelState as Loggable>::name(),
            ComponentReflection {
                docstring_md: "Tri-state for panel controls.",
                placeholder: Some(PanelState::default().to_arrow()?),
            },
        ),
        (
            <QueryExpression as Loggable>::name(),
            ComponentReflection {
                docstring_md: "An individual `QueryExpression` used to filter a set of `EntityPath`s.\n\nEach expression is either an inclusion or an exclusion expression.\nInclusions start with an optional `+` and exclusions must start with a `-`.\n\nMultiple expressions are combined together as part of `SpaceViewContents`.\n\nThe `/**` suffix matches the whole subtree, i.e. self and any child, recursively\n(`/world/**` matches both `/world` and `/world/car/driver`).\nOther uses of `*` are not (yet) supported.",
                placeholder: Some(QueryExpression::default().to_arrow()?),
            },
        ),
        (
            <RootContainer as Loggable>::name(),
            ComponentReflection {
                docstring_md: "The container that sits at the root of a viewport.",
                placeholder: Some(RootContainer::default().to_arrow()?),
            },
        ),
        (
            <RowShare as Loggable>::name(),
            ComponentReflection {
                docstring_md: "The layout share of a row in the container.",
                placeholder: Some(RowShare::default().to_arrow()?),
            },
        ),
        (
            <SpaceViewClass as Loggable>::name(),
            ComponentReflection {
                docstring_md: "The class of a `SpaceView`.",
                placeholder: Some(SpaceViewClass::default().to_arrow()?),
            },
        ),
        (
            <SpaceViewMaximized as Loggable>::name(),
            ComponentReflection {
                docstring_md: "Whether a space view is maximized.",
                placeholder: Some(SpaceViewMaximized::default().to_arrow()?),
            },
        ),
        (
            <SpaceViewOrigin as Loggable>::name(),
            ComponentReflection {
                docstring_md: "The origin of a `SpaceView`.",
                placeholder: Some(SpaceViewOrigin::default().to_arrow()?),
            },
        ),
        (
            <TensorDimensionIndexSlider as Loggable>::name(),
            ComponentReflection {
                docstring_md: "Show a slider for the index of some dimension of a slider.",
                placeholder: Some(TensorDimensionIndexSlider::default().to_arrow()?),
            },
        ),
        (
            <ViewFit as Loggable>::name(),
            ComponentReflection {
                docstring_md: "Determines whether an image or texture should be scaled to fit the viewport.",
                placeholder: Some(ViewFit::default().to_arrow()?),
            },
        ),
        (
            <ViewerRecommendationHash as Loggable>::name(),
            ComponentReflection {
                docstring_md: "Hash of a viewer recommendation.\n\nThe formation of this hash is considered an internal implementation detail of the viewer.",
                placeholder: Some(ViewerRecommendationHash::default().to_arrow()?),
            },
        ),
        (
            <Visible as Loggable>::name(),
            ComponentReflection {
                docstring_md: "Whether the container, view, entity or instance is currently visible.",
                placeholder: Some(Visible::default().to_arrow()?),
            },
        ),
        (
            <VisibleTimeRange as Loggable>::name(),
            ComponentReflection {
                docstring_md: "The range of values on a given timeline that will be included in a view's query.\n\nRefer to `VisibleTimeRanges` archetype for more information.",
                placeholder: Some(VisibleTimeRange::default().to_arrow()?),
            },
        ),
        (
            <VisualBounds2D as Loggable>::name(),
            ComponentReflection {
                docstring_md: "Visual bounds in 2D space used for `Spatial2DView`.",
                placeholder: Some(VisualBounds2D::default().to_arrow()?),
            },
        ),
        (
            <VisualizerOverrides as Loggable>::name(),
            ComponentReflection {
                docstring_md: "Override the visualizers for an entity.\n\nThis component is a stop-gap mechanism based on the current implementation details\nof the visualizer system. It is not intended to be a long-term solution, but provides\nenough utility to be useful in the short term.\n\nThe long-term solution is likely to be based off: <https://github.com/rerun-io/rerun/issues/6626>\n\nThis can only be used as part of blueprints. It will have no effect if used\nin a regular entity.",
                placeholder: Some(VisualizerOverrides::default().to_arrow()?),
            },
        ),
        (
            <AggregationPolicy as Loggable>::name(),
            ComponentReflection {
                docstring_md: "Policy for aggregation of multiple scalar plot values.\n\nThis is used for lines in plots when the X axis distance of individual points goes below a single pixel,\ni.e. a single pixel covers more than one tick worth of data. It can greatly improve performance\n(and readability) in such situations as it prevents overdraw.",
                placeholder: Some(AggregationPolicy::default().to_arrow()?),
            },
        ),
        (
            <AnnotationContext as Loggable>::name(),
            ComponentReflection {
                docstring_md: "The `AnnotationContext` provides additional information on how to display entities.\n\nEntities can use `ClassId`s and `KeypointId`s to provide annotations, and\nthe labels and colors will be looked up in the appropriate\n`AnnotationContext`. We use the *first* annotation context we find in the\npath-hierarchy when searching up through the ancestors of a given entity\npath.",
                placeholder: Some(AnnotationContext::default().to_arrow()?),
            },
        ),
        (
            <AxisLength as Loggable>::name(),
            ComponentReflection {
                docstring_md: "The length of an axis in local units of the space.",
                placeholder: Some(AxisLength::default().to_arrow()?),
            },
        ),
        (
            <Blob as Loggable>::name(),
            ComponentReflection {
                docstring_md: "A binary blob of data.",
                placeholder: Some(Blob::default().to_arrow()?),
            },
        ),
        (
            <ClassId as Loggable>::name(),
            ComponentReflection {
                docstring_md: "A 16-bit ID representing a type of semantic class.\n\nUsed to look up a [`crate::datatypes::ClassDescription`] within the [`crate::components::AnnotationContext`].",
                placeholder: Some(ClassId::default().to_arrow()?),
            },
        ),
        (
            <ClearIsRecursive as Loggable>::name(),
            ComponentReflection {
                docstring_md: "Configures how a clear operation should behave - recursive or not.",
                placeholder: Some(ClearIsRecursive::default().to_arrow()?),
            },
        ),
        (
            <Color as Loggable>::name(),
            ComponentReflection {
                docstring_md: "An RGBA color with unmultiplied/separate alpha, in sRGB gamma space with linear alpha.\n\nThe color is stored as a 32-bit integer, where the most significant\nbyte is `R` and the least significant byte is `A`.",
                placeholder: Some(Color::default().to_arrow()?),
            },
        ),
        (
            <Colormap as Loggable>::name(),
            ComponentReflection {
                docstring_md: "Colormap for mapping scalar values within a given range to a color.\n\nThis provides a number of popular pre-defined colormaps.\nIn the future, the Rerun Viewer will allow users to define their own colormaps,\nbut currently the Viewer is limited to the types defined here.",
                placeholder: Some(Colormap::default().to_arrow()?),
            },
        ),
        (
            <DepthMeter as Loggable>::name(),
            ComponentReflection {
                docstring_md: "The world->depth map scaling factor.\n\nThis measures how many depth map units are in a world unit.\nFor instance, if a depth map uses millimeters and the world uses meters,\nthis value would be `1000`.\n\nNote that the only effect on 2D views is the physical depth values shown when hovering the image.\nIn 3D views on the other hand, this affects where the points of the point cloud are placed.",
                placeholder: Some(DepthMeter::default().to_arrow()?),
            },
        ),
        (
            <DisconnectedSpace as Loggable>::name(),
            ComponentReflection {
                docstring_md: "Spatially disconnect this entity from its parent.\n\nSpecifies that the entity path at which this is logged is spatially disconnected from its parent,\nmaking it impossible to transform the entity path into its parent's space and vice versa.\nIt *only* applies to space views that work with spatial transformations, i.e. 2D & 3D space views.\nThis is useful for specifying that a subgraph is independent of the rest of the scene.",
                placeholder: Some(DisconnectedSpace::default().to_arrow()?),
            },
        ),
        (
            <DrawOrder as Loggable>::name(),
            ComponentReflection {
                docstring_md: "Draw order of 2D elements. Higher values are drawn on top of lower values.\n\nAn entity can have only a single draw order component.\nWithin an entity draw order is governed by the order of the components.\n\nDraw order for entities with the same draw order is generally undefined.",
                placeholder: Some(DrawOrder::default().to_arrow()?),
            },
        ),
        (
            <FillRatio as Loggable>::name(),
            ComponentReflection {
                docstring_md: "How much a primitive fills out the available space.\n\nUsed for instance to scale the points of the point cloud created from `DepthImage` projection in 3D views.\nValid range is from 0 to max float although typically values above 1.0 are not useful.\n\nDefaults to 1.0.",
                placeholder: Some(FillRatio::default().to_arrow()?),
            },
        ),
        (
            <GammaCorrection as Loggable>::name(),
            ComponentReflection {
                docstring_md: "A gamma correction value to be used with a scalar value or color.\n\nUsed to adjust the gamma of a color or scalar value between 0 and 1 before rendering.\n`new_value = old_value ^ gamma`\n\nValid range is from 0 (excluding) to max float.\nDefaults to 1.0 unless otherwise specified.",
                placeholder: Some(GammaCorrection::default().to_arrow()?),
            },
        ),
        (
            <HalfSize2D as Loggable>::name(),
            ComponentReflection {
                docstring_md: "Half-size (radius) of a 2D box.\n\nMeasured in its local coordinate system.\n\nThe box extends both in negative and positive direction along each axis.\nNegative sizes indicate that the box is flipped along the respective axis, but this has no effect on how it is displayed.",
                placeholder: Some(HalfSize2D::default().to_arrow()?),
            },
        ),
        (
            <HalfSize3D as Loggable>::name(),
            ComponentReflection {
                docstring_md: "Half-size (radius) of a 3D box.\n\nMeasured in its local coordinate system.\n\nThe box extends both in negative and positive direction along each axis.\nNegative sizes indicate that the box is flipped along the respective axis, but this has no effect on how it is displayed.",
                placeholder: Some(HalfSize3D::default().to_arrow()?),
            },
        ),
        (
            <ImagePlaneDistance as Loggable>::name(),
            ComponentReflection {
                docstring_md: "The distance from the camera origin to the image plane when the projection is shown in a 3D viewer.\n\nThis is only used for visualization purposes, and does not affect the projection itself.",
                placeholder: Some(ImagePlaneDistance::default().to_arrow()?),
            },
        ),
        (
            <KeypointId as Loggable>::name(),
            ComponentReflection {
                docstring_md: "A 16-bit ID representing a type of semantic keypoint within a class.\n\n`KeypointId`s are only meaningful within the context of a [`crate::datatypes::ClassDescription`].\n\nUsed to look up an [`crate::datatypes::AnnotationInfo`] for a Keypoint within the [`crate::components::AnnotationContext`].",
                placeholder: Some(KeypointId::default().to_arrow()?),
            },
        ),
        (
            <LineStrip2D as Loggable>::name(),
            ComponentReflection {
                docstring_md: "A line strip in 2D space.\n\nA line strip is a list of points connected by line segments. It can be used to draw\napproximations of smooth curves.\n\nThe points will be connected in order, like so:\n```text\n       2------3     5\n      /        \\   /\n0----1          \\ /\n                 4\n```",
                placeholder: Some(LineStrip2D::default().to_arrow()?),
            },
        ),
        (
            <LineStrip3D as Loggable>::name(),
            ComponentReflection {
                docstring_md: "A line strip in 3D space.\n\nA line strip is a list of points connected by line segments. It can be used to draw\napproximations of smooth curves.\n\nThe points will be connected in order, like so:\n```text\n       2------3     5\n      /        \\   /\n0----1          \\ /\n                 4\n```",
                placeholder: Some(LineStrip3D::default().to_arrow()?),
            },
        ),
        (
            <MagnificationFilter as Loggable>::name(),
            ComponentReflection {
                docstring_md: "Filter used when magnifying an image/texture such that a single pixel/texel is displayed as multiple pixels on screen.",
                placeholder: Some(MagnificationFilter::default().to_arrow()?),
            },
        ),
        (
            <MarkerShape as Loggable>::name(),
            ComponentReflection {
                docstring_md: "The visual appearance of a point in e.g. a 2D plot.",
                placeholder: Some(MarkerShape::default().to_arrow()?),
            },
        ),
        (
            <MarkerSize as Loggable>::name(),
            ComponentReflection {
                docstring_md: "Radius of a marker of a point in e.g. a 2D plot, measured in UI points.",
                placeholder: Some(MarkerSize::default().to_arrow()?),
            },
        ),
        (
            <Material as Loggable>::name(),
            ComponentReflection {
                docstring_md: "Material properties of a mesh, e.g. its color multiplier.",
                placeholder: Some(Material::default().to_arrow()?),
            },
        ),
        (
            <MediaType as Loggable>::name(),
            ComponentReflection {
                docstring_md: "A standardized media type (RFC2046, formerly known as MIME types), encoded as a utf8 string.\n\nThe complete reference of officially registered media types is maintained by the IANA and can be\nconsulted at <https://www.iana.org/assignments/media-types/media-types.xhtml>.",
                placeholder: Some(MediaType::default().to_arrow()?),
            },
        ),
        (
            <Name as Loggable>::name(),
            ComponentReflection {
                docstring_md: "A display name, typically for an entity or a item like a plot series.",
                placeholder: Some(Name::default().to_arrow()?),
            },
        ),
        (
            <Opacity as Loggable>::name(),
            ComponentReflection {
                docstring_md: "Degree of transparency ranging from 0.0 (fully transparent) to 1.0 (fully opaque).\n\nThe final opacity value may be a result of multiplication with alpha values as specified by other color sources.\nUnless otherwise specified, the default value is 1.",
                placeholder: Some(Opacity::default().to_arrow()?),
            },
        ),
        (
            <OutOfTreeTransform3D as Loggable>::name(),
            ComponentReflection {
                docstring_md: "An out-of-tree affine transform between two 3D spaces, represented in a given direction.\n\n\"Out-of-tree\" means that the transform only affects its own entity: children don't inherit from it.",
                placeholder: Some(OutOfTreeTransform3D::default().to_arrow()?),
            },
        ),
        (
            <PinholeProjection as Loggable>::name(),
            ComponentReflection {
                docstring_md: "Camera projection, from image coordinates to view coordinates.\n\nChild from parent.\nImage coordinates from camera view coordinates.\n\nExample:\n```text\n1496.1     0.0  980.5\n   0.0  1496.1  744.5\n   0.0     0.0    1.0\n```",
                placeholder: Some(PinholeProjection::default().to_arrow()?),
            },
        ),
        (
            <Position2D as Loggable>::name(),
            ComponentReflection {
                docstring_md: "A position in 2D space.",
                placeholder: Some(Position2D::default().to_arrow()?),
            },
        ),
        (
            <Position3D as Loggable>::name(),
            ComponentReflection {
                docstring_md: "A position in 3D space.",
                placeholder: Some(Position3D::default().to_arrow()?),
            },
        ),
        (
            <Radius as Loggable>::name(),
            ComponentReflection {
                docstring_md: "The radius of something, e.g. a point.\n\nInternally, positive values indicate scene units, whereas negative values\nare interpreted as UI points.\n\nUI points are independent of zooming in Views, but are sensitive to the application UI scaling.\nat 100% UI scaling, UI points are equal to pixels\nThe Viewer's UI scaling defaults to the OS scaling which typically is 100% for full HD screens and 200% for 4k screens.",
                placeholder: Some(Radius::default().to_arrow()?),
            },
        ),
        (
            <Range1D as Loggable>::name(),
            ComponentReflection {
                docstring_md: "A 1D range, specifying a lower and upper bound.",
                placeholder: Some(Range1D::default().to_arrow()?),
            },
        ),
        (
            <Resolution as Loggable>::name(),
            ComponentReflection {
                docstring_md: "Pixel resolution width & height, e.g. of a camera sensor.\n\nTypically in integer units, but for some use cases floating point may be used.",
                placeholder: Some(Resolution::default().to_arrow()?),
            },
        ),
        (
            <Rotation3D as Loggable>::name(),
            ComponentReflection {
                docstring_md: "A 3D rotation, represented either by a quaternion or a rotation around axis.",
                placeholder: Some(Rotation3D::default().to_arrow()?),
            },
        ),
        (
            <Scalar as Loggable>::name(),
            ComponentReflection {
                docstring_md: "A scalar value, encoded as a 64-bit floating point.\n\nUsed for time series plots.",
                placeholder: Some(Scalar::default().to_arrow()?),
            },
        ),
        (
            <StrokeWidth as Loggable>::name(),
            ComponentReflection {
                docstring_md: "The width of a stroke specified in UI points.",
                placeholder: Some(StrokeWidth::default().to_arrow()?),
            },
        ),
        (
            <TensorData as Loggable>::name(),
            ComponentReflection {
                docstring_md: "An N-dimensional array of numbers.\n\nThe number of dimensions and their respective lengths is specified by the `shape` field.\nThe dimensions are ordered from outermost to innermost. For example, in the common case of\na 2D RGB Image, the shape would be `[height, width, channel]`.\n\nThese dimensions are combined with an index to look up values from the `buffer` field,\nwhich stores a contiguous array of typed values.\n\nNote that the buffer may be encoded in a compressed format such as `jpeg` or\nin a format with downsampled chroma, such as NV12 or YUY2.\nFor file formats, the shape is used as a hint, for chroma downsampled format\nthe shape has to be the shape of the decoded image.",
                placeholder: Some(TensorData::default().to_arrow()?),
            },
        ),
        (
            <TensorDimensionIndexSelection as Loggable>::name(),
            ComponentReflection {
                docstring_md: "Specifies a concrete index on a tensor dimension.",
                placeholder: Some(TensorDimensionIndexSelection::default().to_arrow()?),
            },
        ),
        (
            <TensorHeightDimension as Loggable>::name(),
            ComponentReflection {
                docstring_md: "Specifies which dimension to use for height.",
                placeholder: Some(TensorHeightDimension::default().to_arrow()?),
            },
        ),
        (
            <TensorWidthDimension as Loggable>::name(),
            ComponentReflection {
                docstring_md: "Specifies which dimension to use for width.",
                placeholder: Some(TensorWidthDimension::default().to_arrow()?),
            },
        ),
        (
            <Texcoord2D as Loggable>::name(),
            ComponentReflection {
                docstring_md: "A 2D texture UV coordinate.\n\nTexture coordinates specify a position on a 2D texture.\nA range from 0-1 covers the entire texture in the respective dimension.\nUnless configured otherwise, the texture repeats outside of this range.\nRerun uses top-left as the origin for UV coordinates.\n\n  0     U     1\n0 + --------- →\n  |           .\nV |           .\n  |           .\n1 ↓ . . . . . .\n\nThis is the same convention as in Vulkan/Metal/DX12/WebGPU, but (!) unlike OpenGL,\nwhich places the origin at the bottom-left.",
                placeholder: Some(Texcoord2D::default().to_arrow()?),
            },
        ),
        (
            <Text as Loggable>::name(),
            ComponentReflection {
                docstring_md: "A string of text, e.g. for labels and text documents.",
                placeholder: Some(Text::default().to_arrow()?),
            },
        ),
        (
            <TextLogLevel as Loggable>::name(),
            ComponentReflection {
                docstring_md: "The severity level of a text log message.\n\nRecommended to be one of:\n* `\"CRITICAL\"`\n* `\"ERROR\"`\n* `\"WARN\"`\n* `\"INFO\"`\n* `\"DEBUG\"`\n* `\"TRACE\"`",
                placeholder: Some(TextLogLevel::default().to_arrow()?),
            },
        ),
        (
            <Transform3D as Loggable>::name(),
            ComponentReflection {
                docstring_md: "An affine transform between two 3D spaces, represented in a given direction.",
                placeholder: Some(Transform3D::default().to_arrow()?),
            },
        ),
        (
            <TriangleIndices as Loggable>::name(),
            ComponentReflection {
                docstring_md: "The three indices of a triangle in a triangle mesh.",
                placeholder: Some(TriangleIndices::default().to_arrow()?),
            },
        ),
        (
            <Vector2D as Loggable>::name(),
            ComponentReflection {
                docstring_md: "A vector in 2D space.",
                placeholder: Some(Vector2D::default().to_arrow()?),
            },
        ),
        (
            <Vector3D as Loggable>::name(),
            ComponentReflection {
                docstring_md: "A vector in 3D space.",
                placeholder: Some(Vector3D::default().to_arrow()?),
            },
        ),
        (
            <ViewCoordinates as Loggable>::name(),
            ComponentReflection {
                docstring_md: "How we interpret the coordinate system of an entity/space.\n\nFor instance: What is \"up\"? What does the Z axis mean? Is this right-handed or left-handed?\n\nThe three coordinates are always ordered as [x, y, z].\n\nFor example [Right, Down, Forward] means that the X axis points to the right, the Y axis points\ndown, and the Z axis points forward.\n\nThe following constants are used to represent the different directions:\n * Up = 1\n * Down = 2\n * Right = 3\n * Left = 4\n * Forward = 5\n * Back = 6",
                placeholder: Some(ViewCoordinates::default().to_arrow()?),
            },
        ),
    ];
    Ok(ComponentReflectionMap::from_iter(array))
}

/// Generates reflection about all known archetypes.
///
/// Call only once and reuse the results.

fn generate_archetype_reflection() -> ArchetypeReflectionMap {
    re_tracing::profile_function!();
    let array = [
        (
            ArchetypeName::new("rerun.blueprint.archetypes.Background"),
            ArchetypeReflection {
                display_name: "Background",
                docstring_md: "Configuration for the background of a view.",
                fields: vec![
                    ArchetypeFieldReflection { component_name :
                    "rerun.blueprint.components.BackgroundKind".into(), display_name :
                    "Kind", docstring_md : "The type of the background.", },
                    ArchetypeFieldReflection { component_name : "rerun.components.Color"
                    .into(), display_name : "Color", docstring_md :
                    "Color used for the `SolidColor` background type.", },
                ],
            },
        ),
        (
            ArchetypeName::new("rerun.blueprint.archetypes.PlotLegend"),
            ArchetypeReflection {
                display_name: "Plot legend",
                docstring_md: "Configuration for the legend of a plot.",
                fields: vec![
                    ArchetypeFieldReflection { component_name :
                    "rerun.blueprint.components.Corner2D".into(), display_name :
                    "Corner", docstring_md :
                    "To what corner the legend is aligned.\n\nDefaults to the right bottom corner.",
                    }, ArchetypeFieldReflection { component_name :
                    "rerun.blueprint.components.Visible".into(), display_name :
                    "Visible", docstring_md :
                    "Whether the legend is shown at all.\n\nTrue by default.", },
                ],
            },
        ),
        (
            ArchetypeName::new("rerun.blueprint.archetypes.ScalarAxis"),
            ArchetypeReflection {
                display_name: "Scalar axis",
                docstring_md: "Configuration for the scalar axis of a plot.",
                fields: vec![
                    ArchetypeFieldReflection { component_name :
                    "rerun.components.Range1D".into(), display_name : "Range",
                    docstring_md :
                    "The range of the axis.\n\nIf unset, the range well be automatically determined based on the queried data.",
                    }, ArchetypeFieldReflection { component_name :
                    "rerun.blueprint.components.LockRangeDuringZoom".into(), display_name
                    : "Zoom lock", docstring_md :
                    "If enabled, the Y axis range will remain locked to the specified range when zooming.",
                    },
                ],
            },
        ),
        (
            ArchetypeName::new("rerun.blueprint.archetypes.TensorScalarMapping"),
            ArchetypeReflection {
                display_name: "Tensor scalar mapping",
                docstring_md: "Configures how tensor scalars are mapped to color.",
                fields: vec![
                    ArchetypeFieldReflection { component_name :
                    "rerun.components.MagnificationFilter".into(), display_name :
                    "Mag filter", docstring_md :
                    "Filter used when zooming in on the tensor.\n\nNote that the filter is applied to the scalar values *before* they are mapped to color.",
                    }, ArchetypeFieldReflection { component_name :
                    "rerun.components.Colormap".into(), display_name : "Colormap",
                    docstring_md : "How scalar values map to colors.", },
                    ArchetypeFieldReflection { component_name :
                    "rerun.components.GammaCorrection".into(), display_name : "Gamma",
                    docstring_md :
                    "Gamma exponent applied to normalized values before mapping to color.\n\nRaises the normalized values to the power of this value before mapping to color.\nActs like an inverse brightness. Defaults to 1.0.",
                    },
                ],
            },
        ),
        (
            ArchetypeName::new("rerun.blueprint.archetypes.TensorSliceSelection"),
            ArchetypeReflection {
                display_name: "Tensor slice selection",
                docstring_md: "Specifies a 2D slice of a tensor.",
                fields: vec![
                    ArchetypeFieldReflection { component_name :
                    "rerun.components.TensorWidthDimension".into(), display_name :
                    "Width", docstring_md :
                    "Which dimension to map to width.\n\nIf not specified, the height will be determined automatically based on the name and index of the dimension.",
                    }, ArchetypeFieldReflection { component_name :
                    "rerun.components.TensorHeightDimension".into(), display_name :
                    "Height", docstring_md :
                    "Which dimension to map to height.\n\nIf not specified, the height will be determined automatically based on the name and index of the dimension.",
                    }, ArchetypeFieldReflection { component_name :
                    "rerun.components.TensorDimensionIndexSelection".into(), display_name
                    : "Indices", docstring_md :
                    "Selected indices for all other dimensions.\n\nIf any of the here listed dimensions is equal to `width` or `height`, it will be ignored.",
                    }, ArchetypeFieldReflection { component_name :
                    "rerun.blueprint.components.TensorDimensionIndexSlider".into(),
                    display_name : "Slider", docstring_md :
                    "Any dimension listed here will have a slider for the index.\n\nEdits to the sliders will directly manipulate dimensions on the `indices` list.\nIf any of the here listed dimensions is equal to `width` or `height`, it will be ignored.\nIf not specified, adds slides for any dimension in `indices`.",
                    },
                ],
            },
        ),
        (
            ArchetypeName::new("rerun.blueprint.archetypes.TensorViewFit"),
            ArchetypeReflection {
                display_name: "Tensor view fit",
                docstring_md: "Configures how a selected tensor slice is shown on screen.",
                fields: vec![
                    ArchetypeFieldReflection { component_name :
                    "rerun.blueprint.components.ViewFit".into(), display_name :
                    "Scaling", docstring_md : "How the image is scaled to fit the view.",
                    },
                ],
            },
        ),
        (
            ArchetypeName::new("rerun.blueprint.archetypes.VisualBounds2D"),
            ArchetypeReflection {
                display_name: "Visual bounds 2D",
                docstring_md: "Controls the visual bounds of a 2D view.\n\nEverything within these bounds are guaranteed to be visible.\nSomethings outside of these bounds may also be visible due to letterboxing.\n\nIf no visual bounds are set, it will be determined automatically,\nbased on the bounding-box of the data or other camera information present in the view.",
                fields: vec![
                    ArchetypeFieldReflection { component_name :
                    "rerun.blueprint.components.VisualBounds2D".into(), display_name :
                    "Range", docstring_md :
                    "Controls the visible range of a 2D view.\n\nUse this to control pan & zoom of the view.",
                    },
                ],
            },
        ),
    ];
    ArchetypeReflectionMap::from_iter(array)
}
