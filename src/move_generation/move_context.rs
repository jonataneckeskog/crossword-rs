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
    pub depth: i32,
    pub is_horizontal: bool,
    pub is_forwards: bool,
}

#[derive(Debug)]
pub enum ExtendAction {
    PlaceFromRack(usize, char),
    TraverseExisting(),
    TraversePivot(),
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
        depth: i32,
        is_horizontal: bool,
        is_forwards: bool,
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
            is_forwards,
        }
    }

    #[inline]
    // Returns depth as a usize (for convenience)
    pub fn depth(&self) -> usize {
        self.depth as usize
    }

    #[inline]
    pub fn position_at_depth(&self) -> usize {
        if self.is_horizontal {
            self.anchor + self.depth()
        } else {
            self.anchor + BOARD_SIZE * self.depth()
        }
    }

    #[inline]
    pub fn current_tile(&self) -> char {
        self.buffer[self.depth()]
    }

    #[inline]
    pub fn current_tile_with_mod(&self, modifyer: i32) -> char {
        self.buffer[(self.depth + modifyer) as usize]
    }

    pub fn extend(&mut self, action: &ExtendAction, new_node: &'a GaddagNode) {
        // Node always updates
        self.node = new_node;

        match action {
            ExtendAction::PlaceFromRack(idx, tile) => {
                // Update buffer, rack and move
                self.rack.mark_used(*idx);
                self.buffer[self.depth()] = *tile;

                self.current_tiles[self.current_move_len as usize] = *tile;

                self.current_positions[self.current_move_len as usize] =
                    self.position_at_depth() as BoardPosition;

                self.current_move_len += 1;
            }
            ExtendAction::TraverseExisting() => {}
            ExtendAction::TraversePivot() => {
                self.is_forwards = !self.is_forwards;
                return;
            }
        }

        // Update depth
        self.depth = if self.is_forwards {
            self.depth + 1
        } else {
            self.depth.saturating_sub(1)
        };
    }

    pub fn undo(&mut self, action: &ExtendAction, previous_node: &'a GaddagNode) {
        // Update node
        self.node = previous_node;

        // Update buffer
        match action {
            ExtendAction::TraversePivot() => {
                self.is_forwards = !self.is_forwards;
                return;
            }
            _ => {
                self.depth = if self.is_forwards {
                    self.depth.saturating_sub(1)
                } else {
                    self.depth + 1
                };
            }
        }

        // Handle cleanup for PlaceFromRack
        if let ExtendAction::PlaceFromRack(idx, _) = action {
            self.rack.unmark_used(*idx);
            self.buffer[self.depth()] = EMPTY_TILE;
            self.current_move_len -= 1;
            self.current_positions[self.current_move_len as usize] = 0;
            self.current_tiles[self.current_move_len as usize] = EMPTY_TILE;
        }
    }

    fn debug_log(ctx: &RecursionContext, action: &ExtendAction) {
        println!(
            "[DEBUG] action={:?}, depth={}, anchor={}, is_horizontal={}, is_forwards={}",
            action, ctx.depth, ctx.anchor, ctx.is_horizontal, ctx.is_forwards
        );

        // Print current buffer row/column
        println!("Buffer: {:?}", ctx.buffer);

        // Print current move so far
        if ctx.current_move_len > 0 {
            let tiles: Vec<_> = ctx.current_tiles[..ctx.current_move_len as usize].to_vec();
            let positions: Vec<_> = ctx.current_positions[..ctx.current_move_len as usize].to_vec();
            println!("Move so far: {:?} at positions {:?}", tiles, positions);
        }
    }
}
