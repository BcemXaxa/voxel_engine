use crate::{for_multi, modules::logic::voxel::Color};

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

    pub fn cat() -> Self {
        const D: usize = Chunk::DIMENSIONS;
        let mut cat = Self::empty();

        let w = Some([255, 255, 255, 255]);
        let g = Some([204, 207, 221, 255]);
        let d = Some([67, 74, 103, 255]);
        let b = Some([20, 20, 20, 255]);
        let c = Some([3, 157, 227, 255]);
        let o = Some([234, 117, 17, 255]);
        let p = Some([235, 128, 193, 255]);
        const N: Option<Color> = None;

        let pixel_art: [[Voxel; 18]; 17] = [
            [N, b, b, b, N, N, N, N, N, b, b, b, N, N, N, N, N, N],
            [N, b, w, w, b, b, b, b, b, w, w, b, N, N, N, N, N, N],
            [N, b, g, g, w, w, w, w, w, g, g, b, N, N, N, N, N, N],
            [N, b, g, w, w, w, w, w, w, w, g, b, N, N, N, N, N, N],
            [N, b, w, w, w, w, w, w, w, w, w, b, N, N, N, N, N, N],
            [b, b, w, w, b, w, w, w, b, w, w, b, b, N, N, N, N, N],
            [N, b, w, w, c, w, w, w, o, w, w, b, N, N, N, N, N, N],
            [b, b, g, w, w, w, p, w, w, w, g, b, b, N, N, N, N, N],
            [N, N, b, g, w, w, w, w, w, g, b, N, N, N, N, N, N, N],
            [N, b, g, w, w, w, w, w, w, w, g, b, N, N, N, b, b, N],
            [N, b, w, w, w, w, w, w, w, w, w, b, N, N, b, w, w, b],
            [b, g, w, w, w, w, w, w, w, w, w, g, b, N, b, w, w, b],
            [b, g, w, w, w, w, w, w, w, w, w, g, b, N, b, w, w, b],
            [b, g, d, w, w, d, w, d, w, w, d, g, b, b, w, w, w, b],
            [b, g, d, w, g, d, w, d, g, w, d, g, b, g, w, w, b, N],
            [N, b, g, d, g, d, g, d, g, d, g, b, g, g, g, b, N, N],
            [N, N, b, b, b, b, b, b, b, b, b, b, b, b, b, N, N, N],
        ];

        for y in 0..pixel_art.len() {
            for x in 0..pixel_art[y].len() {
                cat.voxels[D - y - 10][D / 2][10 + x] = pixel_art[y][x];
            }
        }
        cat
    }
}
