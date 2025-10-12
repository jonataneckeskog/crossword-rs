#![allow(dead_code)]

use std::collections::HashSet;

use crate::constants::{BOARD_SIZE, BoardPosition, EMPTY_TILE, RACK_SIZE, TOTAL_SIZE};
use crate::core::{Board, CrosswordMove, Rack};
use crate::move_generation::gaddag::GaddagNode;

pub struct GeneratorContext<'a> {
    // Store values
    pub moves: HashSet<CrosswordMove>,
    pub explored_anchors: [bool; TOTAL_SIZE],

    // Start rack
    pub rack: &'a Rack,

    // Precomputer buffers
    pub hori_buffers: [[char; BOARD_SIZE]; BOARD_SIZE],
    pub vert_buffers: [[char; BOARD_SIZE]; BOARD_SIZE],
}

pub struct RecursionContext<'a> {
    // Quickly build moves
    pub current_letters: [char; RACK_SIZE],
    pub current_positions: [BoardPosition; RACK_SIZE],
    pub current_move_len: u8,

    // Logic
    pub node: &'a GaddagNode,
    pub buffer: [char; BOARD_SIZE],
    pub is_horizontal: bool,
}

impl<'a> GeneratorContext<'a> {
    pub fn new(board: &'a Board, rack: &'a Rack) -> Self {
        let moves = HashSet::new();
        let explored_anchors = [false; TOTAL_SIZE];

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
            moves,
            explored_anchors,
            rack,
            hori_buffers,
            vert_buffers,
        }
    }
}

impl<'a> RecursionContext<'a> {
    pub fn new(node: &'a GaddagNode, buffer: [char; BOARD_SIZE], is_horizontal: bool) -> Self {
        Self {
            current_letters: [EMPTY_TILE; RACK_SIZE],
            current_positions: [0; RACK_SIZE],
            current_move_len: 0,
            node,
            buffer,
            is_horizontal,
        }
    }
}
