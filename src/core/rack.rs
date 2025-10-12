#![allow(dead_code)]

use crate::constants::{EMPTY_TILE, RACK_SIZE};

pub struct Rack {
    pub tiles: [char; RACK_SIZE],
    pub len: usize,
    pub used_mask: u8,
}

impl Rack {
    pub fn from_arrays(tiles: [char; RACK_SIZE], len: usize) -> Self {
        let mut used_mask: u8 = 0;
        for (i, tile) in tiles.iter().enumerate() {
            if *tile != EMPTY_TILE {
                used_mask |= 1 << i;
            }
        }
        Self {
            tiles,
            len,
            used_mask,
        }
    }

    #[inline]
    pub fn is_used(&self, idx: usize) -> bool {
        (self.used_mask & (1 << idx)) != 0
    }

    #[inline]
    pub fn mark_used(&mut self, idx: usize) {
        self.used_mask |= 1 << idx;
        self.len -= 1;
    }

    #[inline]
    pub fn unmark_used(&mut self, idx: usize) {
        self.used_mask &= !(1 << idx);
        self.len += 1;
    }

    /// Iterate over available tiles with their indices.
    pub fn available_tiles(&self) -> impl Iterator<Item = (usize, char)> + '_ {
        self.tiles
            .iter()
            .enumerate()
            .filter(|(i, _)| !self.is_used(*i))
            .map(|(i, &c)| (i, c))
    }
}
