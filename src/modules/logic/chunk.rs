use crate::for_multi;

use super::voxel::Voxel;

pub struct Chunk {
    pub voxels: [[[Voxel; Self::DIMENSIONS]; Self::DIMENSIONS]; Self::DIMENSIONS],
}

impl Chunk {
    pub const DIMENSIONS: usize = 32;
}

impl Chunk {
    pub fn empty() -> Self {
        const D: usize = Chunk::DIMENSIONS;
        Self {
            voxels: [[[Voxel::None; D]; D]; D],
        }
    }

    pub fn random() -> Self {
        const D: usize = Chunk::DIMENSIONS;
        let mut random = Self::empty();
        for_multi!(0..D, 0..D, 0..D; |x: usize, y: usize, z: usize| {
            let color = [
                (255 - (x * 31) % 255) as u8,
                (255 - (y * 31) % 255) as u8,
                (255 - (z * 31) % 255) as u8,
                255
            ];
            random.voxels[z][y][x] = Some(color);
        });
        random
    }
}
