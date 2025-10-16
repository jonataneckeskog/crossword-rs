#![allow(dead_code)]

use crate::CrosswordMove;
use crate::constants::{BOARD_SIZE, BoardPosition, EMPTY_TILE, TOTAL_SIZE};

pub struct Board {
    tiles: [char; TOTAL_SIZE],
    tile_count: usize,
}

impl Board {
    pub fn new() -> Self {
        Self {
            tiles: [EMPTY_TILE; TOTAL_SIZE],
            tile_count: 0,
        }
    }

    #[inline]
    pub fn is_cell_empty(&self, index: usize) -> bool {
        self.tiles[index] == EMPTY_TILE
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.tile_count == 0
    }

    pub fn place(&mut self, tile: char, index: BoardPosition) {
        self.tiles[index as usize] = tile;
    }

    pub fn get(&self, index: usize) -> char {
        self.tiles[index]
    }

    pub fn make_move(&mut self, crossword_move: &CrosswordMove) {
        for placement in crossword_move.iter() {
            self.place(placement.0, placement.1);
        }
        self.tile_count += crossword_move.len();
    }

    pub fn undo_move(&mut self, crossword_move: &CrosswordMove) {
        for placement in crossword_move.iter() {
            self.place(EMPTY_TILE, placement.1);
        }
        self.tile_count -= crossword_move.len();
    }

    pub fn is_anchor(&self, index: usize) -> bool {
        if !self.is_cell_empty(index as usize) {
            return false;
        }

        let row = index / BOARD_SIZE;
        let col = index % BOARD_SIZE;

        // Check left
        if col > 0 && !self.is_cell_empty(index - 1) {
            return true;
        }

        // Check right
        if col + 1 < BOARD_SIZE && !self.is_cell_empty(index + 1) {
            return true;
        }

        // Check up
        if row > 0 && !self.is_cell_empty(index - BOARD_SIZE) {
            return true;
        }

        // Check down
        if row + 1 < BOARD_SIZE && !self.is_cell_empty(index + BOARD_SIZE) {
            return true;
        }

        false
    }

    pub fn increase_tile_count(&mut self) {
        self.tile_count += 1;
    }
}
