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
            let seed = 8*x + 2*y + 3*z;
            let condition = seed % 27 == 0;
            random.voxels[z][y][x] = if condition {
                let color = [
                    (x * 23 % 255) as u8,
                    (y * 224 % 255) as u8,
                    (z * 25 % 255) as u8,
                    100
                ];
                Some(color)
            }
            else {
                None
            }
        });
        random
    }
}
