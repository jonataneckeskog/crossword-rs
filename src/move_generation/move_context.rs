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
    // Anchor we are generating for
    pub anchor: usize,

    // Keep track of move
    pub current_tiles: [char; RACK_SIZE],
    pub current_positions: [BoardPosition; RACK_SIZE],
    pub current_move_len: u8,

    // Recursion logic
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
        anchor: usize,
        node: &'a GaddagNode,
        rack: &'a mut Rack,
        buffer: [char; BOARD_SIZE],
        depth: usize,
        is_horizontal: bool,
    ) -> Self {
        Self {
            anchor,
            current_tiles: [EMPTY_TILE; RACK_SIZE],
            current_positions: [0; RACK_SIZE],
            current_move_len: 0,
            node,
            rack,
            buffer,
            depth,
            is_horizontal,
        }
    }

    #[inline]
    pub fn current_tile(&self) -> char {
        self.buffer[self.depth]
    }

    #[inline]
    pub fn current_tile_with_mod(&self, modifyer: i32) -> char {
        self.buffer[((self.depth as i32) + modifyer) as usize]
    }

    #[inline]
    pub fn extend(&mut self, idx: usize, tile: char, new_node: &'a GaddagNode, is_forwards: bool) {
        // Move
        self.current_tiles[self.depth] = tile;
        self.current_positions[self.depth] = if self.is_horizontal {
            (self.anchor + self.depth) as BoardPosition
        } else {
            (self.anchor + BOARD_SIZE * self.depth) as BoardPosition
        };
        self.current_move_len += 1;

        // Recursion
        self.node = new_node;
        self.rack.mark_used(idx);
        self.buffer[self.depth] = tile;
        self.depth = if is_forwards {
            self.depth + 1
        } else {
            self.depth - 1
        }
    }

    #[inline]
    pub fn undo(&mut self, idx: usize, previous_node: &'a GaddagNode, is_forwards: bool) {
        // Recursion
        self.depth = if is_forwards {
            self.depth - 1
        } else {
            self.depth + 1
        };
        self.buffer[self.depth] = EMPTY_TILE;
        self.node = previous_node;

        // Move
        self.rack.unmark_used(idx);
        self.current_move_len -= 1;
        self.current_positions[self.depth] = 0;
        self.current_tiles[self.depth] = EMPTY_TILE;
    }
}
