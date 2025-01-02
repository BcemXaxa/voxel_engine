use std::collections::HashMap;

use crate::modules::{math::{
    cg::Frustum,
    mat::{Mat4x4, MatMult},
}, renderer::vertex_buffer::MyVertex};

use super::{camera::Camera, chunk::Chunk, light::Light};

type ChunkIndex = [isize; 3];
struct Scene {
    chunks: HashMap<ChunkIndex, Chunk>,
    light: Light,
    view: View,
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

impl Scene {
    pub fn get_view_projection(&self) -> Mat4x4 {
        self.view.view_projection()
    }

    pub fn get_chunk_mesh(&self, idx: ChunkIndex) -> Vec<MyVertex>{
        todo!()
    }
}
