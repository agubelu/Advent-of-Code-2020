use bit_reverse::LookupReverse;
use crate::etc::utils::{MatNum, MatPosition};
use Rotation::*;

///////////////////////////////////////////////////////////////////////////////

pub type TileElem = u8;
pub type BorderInfo = [u16; 4];
pub type FullBorderInfo = [u16; 8];
pub type Transformation = (Rotation, bool);

///////////////////////////////////////////////////////////////////////////////

pub struct Tile {
    tile_id: usize,
    side: usize,
    mat: MatNum<TileElem>,
}


pub enum Rotation { Rot0, Rot90, Rot180, Rot270 }

impl Tile {
    pub fn from_mat(tile_id: usize, mat: MatNum<TileElem>) -> Self {
        assert!(mat.width() == mat.height(), "The tile must contain a square matrix,");

        Tile {
            tile_id,
            side: mat.width(),
            mat,
        }
    }

    pub fn side(&self) -> usize {
        self.side
    }

    pub fn id(&self) -> usize {
        self.tile_id
    }

    pub fn mat(&self) -> &MatNum<TileElem> {
        &self.mat
    }

    pub fn flat(&self) -> &[TileElem] {
        self.mat.flat()
    }

    // Represents each border as an u16 integer
    // This will fail miserably if the tiles have a side > 16
    pub fn border_info(&self) -> BorderInfo {
        let last = self.side - 1;
        let (mut top, mut bottom, mut left, mut right) = (0, 0, 0, 0);

        for x in 0..self.side {
            top = (top << 1) | self.mat.get(x, 0) as u16;
            bottom = (bottom << 1) | self.mat.get(x, last) as u16;
        }

        for y in 0..self.side {
            left = (left << 1) | self.mat.get(0, y) as u16;
            right = (right << 1) | self.mat.get(last, y) as u16;
        }

        return [top, right, bottom, left];
    }

    // Returns the same information as the previous method, plus how the borders
    // would look like if they were flipped
    pub fn full_border_info(&self) -> FullBorderInfo {
        let [top, right, bottom, left] = self.border_info();
        let diff = 16 - self.side;

        let top_flip = top.swap_bits() >> diff;
        let bottom_flip = bottom.swap_bits() >> diff;
        let left_flip = left.swap_bits() >> diff;
        let right_flip = right.swap_bits() >> diff;

        return [top, right, bottom, left, top_flip, right_flip, bottom_flip, left_flip];
    }

    pub fn transform(&self, transform: Transformation) -> Self {
        let side = self.side;
        let max = side - 1;

        let tfunc = match transform {
            (Rot0,   false) => return self.clone(),
            (Rot90,  false) => rot90_nof,
            (Rot180, false) => rot180_nof,
            (Rot270, false) => rot270_nof,
            (Rot0,    true) => rot0_f,
            (Rot90,   true) => rot90_f,
            (Rot180,  true) => rot180_f,
            (Rot270,  true) => rot270_f,
        };

        let mut mat = MatNum::new(side, side, 0);
        
        for y in 0..side {
            for x in 0..side {
                let coords_org = (x, y);
                let coords_dst = tfunc(coords_org, max);
                mat[coords_dst] = self.mat[coords_org];
            }
        }

        return Self::from_mat(self.tile_id, mat);
    }
}

impl Clone for Tile {
    fn clone(&self) -> Self {
        let mat = self.mat.clone();
        Tile::from_mat(self.tile_id, mat)
    }
}

///////////////////////////////////////////////////////////////////////////////
/// Bunch of functions that transform a point in an original matrix to a point
/// in a transformed matrix

fn rot90_nof(pos: MatPosition, max: usize) -> MatPosition { (pos.1, max - pos.0) }
fn rot180_nof(pos: MatPosition, max: usize) -> MatPosition { (max - pos.0, max - pos.1) }
fn rot270_nof(pos: MatPosition, max: usize) -> MatPosition { (max - pos.1, pos.0) }
fn rot0_f(pos: MatPosition, max: usize) -> MatPosition { (max - pos.0, pos.1) }
fn rot90_f(pos: MatPosition, max: usize) -> MatPosition { (max - pos.1, max - pos.0) }
fn rot180_f(pos: MatPosition, max: usize) -> MatPosition { (pos.0, max - pos.1) }
fn rot270_f(pos: MatPosition, _: usize) -> MatPosition { (pos.1, pos.0) }