use crate::constants::{BoardPosition, RACK_SIZE};

pub struct CrosswordMove {
    tiles: [char; RACK_SIZE],
    positions: [BoardPosition; RACK_SIZE],
    len: u8,
}

impl CrosswordMove {
    pub fn from_arrays(tiles: [char; RACK_SIZE], positions: [BoardPosition; RACK_SIZE], len: u8) -> Self {
        Self {
            tiles,
            positions,
            len,
        }
    }

    pub fn iter(&self) -> MoveIterator<'_> {
        MoveIterator {
            move_ref: self,
            index: 0,
        }
    }
}

pub struct MoveIterator<'a> {
    move_ref: &'a CrosswordMove,
    index: usize,
}

impl<'a> Iterator for MoveIterator<'a> {
    type Item = (char, BoardPosition);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.move_ref.len as usize {
            let item = (
                self.move_ref.tiles[self.index],
                self.move_ref.positions[self.index],
            );
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}
