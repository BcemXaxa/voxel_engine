use std::{
    cell::{Cell, RefCell},
    collections::{hash_map::Iter, HashMap},
};

use super::{
    camera::{Camera, OrientedCamera, TrackingCamera},
    chunk::Chunk,
    light::Light,
};
use crate::modules::math::{angle::Angle, cg::Orientation, quaternion::Quaternion, vec::*};

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
        chunks.insert([1, 0, 0], Chunk::cat());
        Self {
            chunks,
            light: Light::default(),
            // camera: RefCell::new(TrackingCamera {
            //     pos: [0.0, -5.0, 0.0],
            //     target: [32.0, 32.0, 32.0].div(2.0),
            // }),
            camera: RefCell::new(OrientedCamera {
                pos: [0.0, -5.0, 0.0],
                orientation: Quaternion::default(),
            }),
        }
    }
}

impl Scene {
    pub fn get_chunk(&self, idx: ChunkIndex) -> Option<&Chunk> {
        self.chunks.get(&idx)
    }

    pub fn get_chunks(&self) -> Iter<ChunkIndex, Chunk>{
        self.chunks.iter()
    }
}
