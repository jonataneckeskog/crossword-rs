#![allow(dead_code)]

use std::collections::HashSet;

use crate::constants::{BOARD_SIZE, BoardPosition, EMPTY_TILE, RACK_SIZE, TOTAL_SIZE};
use crate::core::{Board, CrosswordMove, Rack};
use crate::move_generation::gaddag::GaddagNode;

pub struct GeneratorContext {
    // Store values
    pub moves: HashSet<CrosswordMove>,
    pub explored_anchors: [bool; TOTAL_SIZE],

    // Precomputer buffers
    pub hori_buffers: [[char; BOARD_SIZE]; BOARD_SIZE],
    pub vert_buffers: [[char; BOARD_SIZE]; BOARD_SIZE],
}

pub struct RecursionContext<'a> {
    // Keep track of move
    pub current_tiles: [char; RACK_SIZE],
    pub current_positions: [BoardPosition; RACK_SIZE],
    pub current_move_len: u8,

    // Word building
    pub node: &'a GaddagNode,
    pub rack: &'a mut Rack,
    pub buffer: [char; BOARD_SIZE],
    pub depth: usize,
    pub is_horizontal: bool,
}

impl GeneratorContext {
    pub fn new(board: &Board) -> Self {
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
            hori_buffers,
            vert_buffers,
        }
    }
}

impl<'a> RecursionContext<'a> {
    pub fn new(
        node: &'a GaddagNode,
        rack: &'a mut Rack,
        buffer: [char; BOARD_SIZE],
        depth: usize,
        is_horizontal: bool,
    ) -> Self {
        Self {
            current_tiles: [EMPTY_TILE; RACK_SIZE],
            rack,
            current_positions: [0; RACK_SIZE],
            current_move_len: 0,
            node,
            buffer,
            depth,
            is_horizontal,
        }
    }
}
