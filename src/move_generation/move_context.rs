#![allow(dead_code)]

use crate::constants::{BOARD_SIZE, BoardPosition, EMPTY_TILE, RACK_SIZE, TOTAL_SIZE};
use crate::core::{Board, Rack};
use crate::move_generation::step::Step;

pub struct GeneratorContext<'a> {
    // Start rack
    pub rack: &'a Rack,

    // Precomputer buffers
    pub hori_buffers: [[char; BOARD_SIZE]; BOARD_SIZE],
    pub vert_buffers: [[char; BOARD_SIZE]; BOARD_SIZE],
}

pub struct GenerationContext {
    // Quickly build moves
    pub current_letters: [char; RACK_SIZE],
    pub current_positions: [BoardPosition; RACK_SIZE],
    pub current_move_len: u8,

    // Logic
    pub buffer: [char; BOARD_SIZE],
    pub step: Step,
    pub is_horizontal: bool,
}

impl<'a> GeneratorContext<'a> {
    pub fn new(board: &'a Board, rack: &'a Rack) -> Self {
        let mut hori_buffers = [[EMPTY_TILE; BOARD_SIZE]; BOARD_SIZE];
        let mut vert_buffers = [[EMPTY_TILE; BOARD_SIZE]; BOARD_SIZE];

        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                let tile = board.get(y * BOARD_SIZE + x);
                hori_buffers[y][x] = tile;
                vert_buffers[x][y] = tile;
            }
        }

        Self {
            rack,
            hori_buffers,
            vert_buffers,
        }
    }
}

impl GenerationContext {
    pub fn new(buffer: [char; BOARD_SIZE], step: Step, is_horizontal: bool) -> Self {
        Self {
            current_letters: [EMPTY_TILE; RACK_SIZE],
            current_positions: [0; RACK_SIZE],
            current_move_len: 0,
            buffer,
            step,
            is_horizontal,
        }
    }
}
