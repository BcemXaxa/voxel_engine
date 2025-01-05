use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex};

use crate::{
    for_multi,
    modules::math::vec::{Vec3, VecAdd, VecMult},
};

type Color = [u8; 3];
type Voxel = Option<Color>;

const K: usize = 32;
pub struct Chunk {
    voxels: [[[Voxel; K]; K]; K],
}

impl Chunk {
    pub fn empty() -> Self {
        Self {
            voxels: [[[Voxel::None; K]; K]; K],
        }
    }

    pub fn random() -> Self {
        let mut random = Self::empty();
        for_multi!(0..K, 0..K, 0..K; |x: usize, y: usize, z: usize| {
            let color = [
                (255 - (x * 31) % 255) as u8,
                (255 - (y * 31) % 255) as u8,
                (255 - (z * 31) % 255) as u8
            ];
            random.voxels[z][y][x] = Some(color);
        });
        random
    }
}

impl Chunk {
    pub fn mesh(&self) -> Vec<ChunkMeshVertex> {
        let mut mesh = Vec::new();
        for_multi!(0..K, 0..K, 0..K; |x: usize, y: usize, z: usize| {
            if let Some(color) = self.voxels[z][y][x] {
                mesh.append(&mut Self::voxel_mesh((x, y, z), color));
            }
        });
        mesh
    }

    fn voxel_mesh((x, y, z): (usize, usize, usize), color: Color) -> Vec<ChunkMeshVertex> {
        use ChunkMeshVertex as V;
        let shift = [x as f32, y as f32, z as f32];
        let color = [color[0] as f32, color[1] as f32, color[2] as f32, 255.0].div(255.0);

        // TODO(optimize): voxel mesh
        VOXEL_MESH
            .iter()
            .map(|vert| V {
                pos: vert.add(shift),
                color,
            })
            .collect()
    }
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

#[derive(BufferContents, Vertex)]
#[repr(C)]
pub struct ChunkMeshVertex {
    #[format(R32G32B32_SFLOAT)]
    pub pos: [f32; 3],
    #[format(R32G32B32A32_SFLOAT)]
    pub color: [f32; 4],
}
