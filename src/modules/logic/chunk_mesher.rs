use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex};

use crate::{for_multi, modules::{logic::chunk::Chunk, math::vec::{Vec3, VecAdd}}};

use super::voxel::Color;


#[derive(BufferContents, Vertex)]
#[repr(C)]
pub struct ChunkMeshVertex {
    #[format(R32G32B32_SFLOAT)]
    pub pos: [f32; 3],
    #[format(R8G8B8A8_UNORM)]
    pub color: [u8; 4],
}

pub fn mesh(chunk: &Chunk) -> Vec<ChunkMeshVertex> {
    const D: usize = Chunk::DIMENSIONS;
    let mut mesh = Vec::new();
    for_multi!(0..D, 0..D, 0..D; |x: usize, y: usize, z: usize| {
        if let Some(color) = chunk.voxels[z][y][x] {
            mesh.append(&mut voxel_mesh((x, y, z), color));
        }
    });
    mesh
}

fn voxel_mesh((x, y, z): (usize, usize, usize), color: Color) -> Vec<ChunkMeshVertex> {
    let shift = [x as f32, y as f32, z as f32];
    // TODO(optimize): voxel mesh
    VOXEL_MESH
        .iter()
        .map(|vert| ChunkMeshVertex {
            pos: vert.add(shift),
            color,
        })
        .collect()
}

const VOXEL_MESH: [Vec3; 36] = [
    // xz
    [0.0, 0.0, 0.0],
    [1.0, 0.0, 0.0],
    [1.0, 0.0, 1.0],
    [1.0, 0.0, 1.0],
    [0.0, 0.0, 1.0],
    [0.0, 0.0, 0.0],
    // xy
    [0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [1.0, 1.0, 0.0],
    [1.0, 1.0, 0.0],
    [1.0, 0.0, 0.0],
    [0.0, 0.0, 0.0],
    // yz
    [0.0, 0.0, 0.0],
    [0.0, 0.0, 1.0],
    [0.0, 1.0, 1.0],
    [0.0, 1.0, 1.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0],
    // xz +y
    [0.0, 1.0, 0.0],
    [0.0, 1.0, 1.0],
    [1.0, 1.0, 1.0],
    [1.0, 1.0, 1.0],
    [1.0, 1.0, 0.0],
    [0.0, 1.0, 0.0],
    // xy +z
    [0.0, 0.0, 1.0],
    [1.0, 0.0, 1.0],
    [1.0, 1.0, 1.0],
    [1.0, 1.0, 1.0],
    [0.0, 1.0, 1.0],
    [0.0, 0.0, 1.0],
    // yz +x
    [1.0, 0.0, 0.0],
    [1.0, 1.0, 0.0],
    [1.0, 1.0, 1.0],
    [1.0, 1.0, 1.0],
    [1.0, 0.0, 1.0],
    [1.0, 0.0, 0.0],
];