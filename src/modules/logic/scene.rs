use std::{cell::{Cell, RefCell}, collections::HashMap};

use super::{
    camera::{Camera, OrientedCamera, TrackingCamera},
    chunk::Chunk,
    light::Light,
};
use crate::modules::math::vec::*;

type ChunkIndex = [isize; 3];
pub struct Scene {
    chunks: HashMap<ChunkIndex, Chunk>,
    light: Light,
    pub camera: RefCell<OrientedCamera>,
}

impl Default for Scene {
    fn default() -> Self {
        let mut chunks = HashMap::new();
        chunks.insert([0, 0, 0], Chunk::random());
        Self {
            chunks,
            light: Light::default(),
            camera: RefCell::new(OrientedCamera {
                pos: [0.0, -5.0, 0.0],
                orientation: [0.0, 0.5, 0.5, 0.0].norm(),
            }),
        }
    }
}

impl Scene {
    pub fn get_chunk(&self, idx: ChunkIndex) -> Option<&Chunk> {
        self.chunks.get(&idx)
    }
}