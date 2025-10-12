#![allow(dead_code)]

use crate::constants::{BOARD_SIZE, BoardPosition, RACK_SIZE, TOTAL_SIZE};
use crate::core::{Board, Rack};

pub struct MoveContext<'a> {
    pub rack: &'a Rack,
    pub explored_anchors: [bool; TOTAL_SIZE],

    // Quickly build moves
    pub current_letters: [char; RACK_SIZE],
    pub current_positions: [BoardPosition; RACK_SIZE],
    pub current_move_len: u8,

    // Precomputer buffers
    pub hori_buffers: [[char; BOARD_SIZE]; BOARD_SIZE],
    pub vert_buffers: [[char; BOARD_SIZE]; BOARD_SIZE],
}

impl<'a> MoveContext<'a> {
    pub fn new(board: &'a Board, rack: &'a Rack) -> Self {
        let mut explored_anchors: [bool; TOTAL_SIZE] = [false; TOTAL_SIZE];

        let mut current_letters: [char; RACK_SIZE] = [' '; RACK_SIZE];
        let mut current_positions: [BoardPosition; RACK_SIZE] = [0; RACK_SIZE];
        let mut current_move_len: u8 = 0;

        let mut hori_buffers = [[' '; BOARD_SIZE]; BOARD_SIZE];
        let mut vert_buffers = [[' '; BOARD_SIZE]; BOARD_SIZE];

        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                let tile = board.get(y * BOARD_SIZE + x);
                hori_buffers[y][x] = tile;
                vert_buffers[x][y] = tile;
            }
        }

        Self {
            rack,
            explored_anchors,
            current_letters,
            current_positions,
            current_move_len,
            hori_buffers,
            vert_buffers,
        }
    }
}
