use crate::for_multi;

type Color = u32;

const K: usize = 32;
pub struct Chunk {
    voxels: [[[Voxel; K]; K]; K],
}

impl Chunk {
    pub fn rand() -> Self {
        let mut voxels = [[[Voxel::None; K]; K]; K];
        for_multi!(0..K, 0..K, 0..K; |i: usize, j: usize, k: usize| {
            let choice = i + j + k;
            if choice % 2 == 0 {
                voxels[i][j][k] = Voxel::from(choice as u32);
            }
        });
        Self { voxels }
    }
}

#[derive(Clone, Copy)]
pub enum Voxel {
    None,
    White,
    Black,
    Red,
    Green,
    Blue,
}

impl Voxel {
    pub fn from(i: u32) -> Self {
        use Voxel::*;
        match i % 6 {
            0 => None,
            1 => White,
            2 => Black,
            3 => Red,
            4 => Green,
            5 => Blue,
            _ => unreachable!(),
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Voxel::None => Self::color_from_bytes(0, 0, 0, 0),
            Voxel::White => Self::color_from_bytes(255, 255, 255, 255),
            Voxel::Black => Self::color_from_bytes(0, 0, 0, 255),
            Voxel::Red => Self::color_from_bytes(255, 0, 0, 255),
            Voxel::Green => Self::color_from_bytes(0, 255, 0, 255),
            Voxel::Blue => Self::color_from_bytes(0, 0, 255, 255),
        }
    }

    const fn color_from_bytes(r: u8, g: u8, b: u8, a: u8) -> Color {
        let mut color = 0_u32;
        color |= r as u32;
        color <<= 8;
        color |= g as u32;
        color <<= 8;
        color |= b as u32;
        color <<= 8;
        color |= a as u32;
        color
    }
}
