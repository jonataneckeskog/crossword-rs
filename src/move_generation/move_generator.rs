#![allow(dead_code)]

use std::collections::HashSet;

use crate::constants::{BOARD_SIZE, TOTAL_SIZE};
use crate::core::{Board, CrosswordMove, Rack};
use crate::move_generation::{gaddag::*, move_context::MoveContext, step::Step};

pub struct MoveGenerator<'a> {
    gaddag: &'a Gaddag,
}

impl<'a> MoveGenerator<'a> {
    pub fn new(gaddag: &'a Gaddag) -> Self {
        Self { gaddag }
    }

    pub fn generate_all_moves(&self, board: &Board, rack: &Rack) -> HashSet<CrosswordMove> {
        // Create the MoveContext and initialize the empty moves set
        let mut move_context: MoveContext = MoveContext::new(board, rack);
        let mut moves: HashSet<CrosswordMove> = HashSet::new();

        // Start generating moves
        for index in 0..TOTAL_SIZE {
            if !board.is_anchor(index) {
                continue;
            }

            // Generate moves for anchor
            self.generate_moves_for_anchor(&mut moves, index, &mut move_context);

            // Mark the anchor as explored
            move_context.explored_anchors[index] = true;
        }

        moves
    }

    fn generate_moves_for_anchor(
        &self,
        moves: &mut HashSet<CrosswordMove>,
        anchor: usize,
        ctx: &mut MoveContext,
    ) {
        let row = anchor / BOARD_SIZE;
        let col = anchor % BOARD_SIZE;

        let mut hori_buffer = ctx.hori_buffers[row].clone();
        let mut vert_buffer = ctx.vert_buffers[col].clone();

        self.extend_backwards(hori_buffer, row, ctx.rack.len, &Step::LEFT);
        self.extend_backwards(vert_buffer, col, ctx.rack.len, &Step::UP);
    }

    fn extend_backwards(
        &self,
        buffer: [char; BOARD_SIZE],
        depth: usize,
        limit: usize,
        step: &Step,
    ) {
        // Move generation logic starts here
    }

    fn extend_forwards(
        &self,
        buffer: &mut [usize; BOARD_SIZE],
        depth: usize,
        limit: usize,
        step: &Step,
    ) {
        // And continues here
    }
}
