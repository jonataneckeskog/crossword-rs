#![allow(dead_code)]

use std::collections::HashSet;

use crate::constants::{BOARD_SIZE, EMPTY_TILE, TOTAL_SIZE};
use crate::core::{Board, CrosswordMove, Rack};
use crate::move_generation::{gaddag::*, move_context::*};

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

    // Recursion context is made and owned here
    fn generate_moves_for_anchor(&self, gen_ctx: &mut GeneratorContext<'_>, anchor: usize) {
        let row = anchor / BOARD_SIZE;
        let col = anchor % BOARD_SIZE;

        // Copy buffers (arrays implement Copy)
        // This should be O(BOARD_SIZE), but it's still extremely fast. To
        // get it to O(1) I could wrap the buffer in a Box "Box<[char, BOARD_SIZE]>",
        // but this is likely not faster for BOARD_SIZE = 15. Box forces it to be on
        // the heap, which is where the overhead comes from.
        let hori_buffer = gen_ctx.hori_buffers[row];
        let vert_buffer = gen_ctx.vert_buffers[col];

        // Root is already a borrow, so it's fine to pass it directly
        let root: &GaddagNode = self.gaddag.get_root();
        let mut horizontal_ctx = RecursionContext::new(root, hori_buffer, true);
        let mut vertical_ctx = RecursionContext::new(root, vert_buffer, false);

        self.extend_backwards(gen_ctx, &mut horizontal_ctx);
        self.extend_backwards(gen_ctx, &mut vertical_ctx);
    }

    fn extend_backwards(
        &self,
        gen_context: &mut GeneratorContext<'_>,
        rec_context: &mut RecursionContext<'_>,
    ) {
        // Move generation logic starts here
    }

    fn extend_forwards(
        &self,
        gen_context: &mut GeneratorContext<'_>,
        rec_context: &mut RecursionContext<'_>,
    ) {
        // And continues here
    }

    // Helper functions
    fn get_cross_line<'b>(
        &self,
        gen_ctx: &'b GeneratorContext<'b>,
        depth: usize,
        is_horizontal: bool,
    ) -> &'b [char; BOARD_SIZE] {
        if is_horizontal {
            &gen_ctx.hori_buffers[depth]
        } else {
            &gen_ctx.vert_buffers[depth]
        }
    }

    fn is_crossword_valid(
        &self,
        gen_ctx: &GeneratorContext,
        placed_tile: char,
        depth: usize,
        is_horizontal: bool,
    ) -> bool {
        let crossline = self.get_cross_line(gen_ctx, depth, is_horizontal);

        let mut start = depth;
        let mut end = depth;
        while (start > 0 && crossline[start - 1] != EMPTY_TILE) {
            start -= 1;
        }
        while (end + 1 < BOARD_SIZE && crossline[end + 1] != EMPTY_TILE) {
            end += 1;
        }

        let length = end - start + 1;
        if (length == 1) {
            return true;
        }

        let mut current_node: Option<&GaddagNode> = Some(self.gaddag.get_root());
        for i in start..=end {
            let tile = if i == depth {
                placed_tile
            } else {
                crossline[i]
            };
            if let Some(node) = current_node {
                current_node = node.get_child(tile);
            } else {
                return false;
            }
        }

        current_node.map_or(false, |node| node.is_word())
    }

    fn record_move(&self, gen_ctx: &mut GeneratorContext, rec_ctx: &RecursionContext) {
        // Once again, just copying
        let crossword_move = CrosswordMove::from_arrays(
            rec_ctx.current_tiles,
            rec_ctx.current_positions,
            rec_ctx.current_move_len,
        );
        gen_ctx.moves.insert(crossword_move);
    }
}
