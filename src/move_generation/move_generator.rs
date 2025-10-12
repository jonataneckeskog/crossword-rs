#![allow(dead_code)]

mod helpers;
mod recursion;

use crate::constants::TOTAL_SIZE;
use crate::core::{Board, CrosswordMove, Rack};
use crate::move_generation::{gaddag::Gaddag, move_context::GeneratorContext};
use std::collections::HashSet;

pub struct MoveGenerator<'a> {
    gaddag: &'a Gaddag,
}

impl<'a> MoveGenerator<'a> {
    pub fn new(gaddag: &'a Gaddag) -> Self {
        Self { gaddag }
    }

    // Generator context is made and owned here
    pub fn generate_all_moves(&self, board: &Board, rack: &Rack) -> HashSet<CrosswordMove> {
        // Create move context blocks
        let mut gen_ctx: GeneratorContext = GeneratorContext::new(board, rack);

        // Start generating moves
        for index in 0..TOTAL_SIZE {
            if !board.is_anchor(index) {
                continue;
            }

            // Generate moves for anchor
            self.generate_moves_for_anchor(&mut gen_ctx, index);

            // Mark the anchor as explored
            gen_ctx.explored_anchors[index] = true;
        }

        gen_ctx.moves
    }
}
