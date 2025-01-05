use std::collections::HashMap;

use crate::modules::{
    math::{
        cg::{Frustum, PerspectiveFrustum},
        mat::{Mat4x4, MatMult},
    },
    renderer::vertex_buffer::MyVertex,
};

use super::{
    camera::{Camera, TrackingCamera},
    chunk::{Chunk, ChunkMeshVertex},
    light::Light,
};

type ChunkIndex = [isize; 3];
pub struct Scene {
    chunks: HashMap<ChunkIndex, Chunk>,
    light: Light,
    view: View,
}

impl Default for Scene {
    fn default() -> Self {
        use std::f32::consts::PI;
        let mut chunks = HashMap::new();
        chunks.insert([0, 0, 0], Chunk::random());
        Self {
            chunks,
            light: Light::default(),
            view: View {
                camera: Box::new(TrackingCamera {
                    pos: [0.0, -5.0, 0.0],
                    target: [0.0, 0.0, 0.0],
                }),
                frustum: Box::new(PerspectiveFrustum {
                    near: 1e-1,
                    far: 1e5,
                    fov: PI / 3.0,
                    ar: 8.0 / 6.0,
                }),
            },
        }
    }
}

impl Scene {
    pub fn get_view_projection(&self) -> Mat4x4 {
        self.view.view_projection()
    }

    pub fn get_chunk_mesh(&self, idx: ChunkIndex) -> Vec<ChunkMeshVertex> {
        self.chunks.get(&idx).unwrap().mesh()
    }
}

struct View {
    camera: Box<dyn Camera>,
    frustum: Box<dyn Frustum>,
}

impl View {
    fn view_projection(&self) -> Mat4x4 {
        self.camera
            .view_matrix()
            .mult(self.frustum.projection_matrix())
    }
}
