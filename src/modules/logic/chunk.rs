use crate::for_multi;

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
}
