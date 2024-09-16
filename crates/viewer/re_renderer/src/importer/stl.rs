use std::sync::Arc;

use itertools::Itertools;
use smallvec::smallvec;
use tinystl::StlData;

use crate::{
    mesh::{self, GpuMesh},
    renderer::MeshInstance,
    RenderContext,
};
use re_types::archetypes::Asset3D;

#[derive(thiserror::Error, Debug)]
pub enum StlImportError {
    #[error("Error loading STL mesh: {0}")]
    TinyStl(tinystl::Error),

    #[error(transparent)]
    MeshError(#[from] mesh::MeshError),
}

/// Load a [STL .stl file](https://en.wikipedia.org/wiki/STL_(file_format)) into the mesh manager.
pub fn load_stl_from_buffer(
    asset3d: &Asset3D,
    ctx: &RenderContext,
    _texture_key: u64,
) -> Result<Vec<MeshInstance>, StlImportError> {
    re_tracing::profile_function!();

    let Asset3D {
        blob,
        albedo_factor,
        ..
    } = asset3d;

    let buffer = blob.as_slice();

    let cursor = std::io::Cursor::new(buffer);
    let StlData {
        name,
        triangles,
        normals,
        ..
    } = StlData::read_buffer(std::io::BufReader::new(cursor)).map_err(StlImportError::TinyStl)?;

    let num_vertices = triangles.len() * 3;

    let material = mesh::Material {
        label: name.clone().into(),
        index_range: 0..num_vertices as u32,
        albedo: ctx.texture_manager_2d.white_texture_unorm_handle().clone(),
        albedo_factor: albedo_factor.map_or(crate::Rgba::WHITE, |c| c.0.into()),
    };

    let mesh = mesh::Mesh {
        label: name.into(),
        triangle_indices: (0..num_vertices as u32)
            .tuples::<(_, _, _)>()
            .map(glam::UVec3::from)
            .collect::<Vec<_>>(),
        vertex_positions: bytemuck::cast_slice(&triangles).to_vec(),

        // Normals on STL are per triangle, not per vertex.
        // Yes, this makes STL always look faceted.
        vertex_normals: normals
            .into_iter()
            .flat_map(|n| {
                let n = glam::Vec3::from_array(n);
                [n, n, n]
            })
            .collect(),

        // STL has neither colors nor texcoords.
        vertex_colors: vec![crate::Rgba32Unmul::WHITE; num_vertices],
        vertex_texcoords: vec![glam::Vec2::ZERO; num_vertices],

        materials: smallvec![material],
    };

    mesh.sanity_check()?;

    Ok(vec![MeshInstance::new_with_cpu_mesh(
        Arc::new(GpuMesh::new(ctx, &mesh)?),
        Some(Arc::new(mesh)),
    )])
}
