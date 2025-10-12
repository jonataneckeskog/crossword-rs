#![allow(dead_code)]

use std::collections::HashSet;

use crate::constants::{BOARD_SIZE, EMPTY_TILE, TOTAL_SIZE};
use crate::core::{Board, CrosswordMove, Rack};
use crate::move_generation::{gaddag::*, move_context::*, step::Step};

pub struct MoveGenerator<'a> {
    gaddag: &'a Gaddag,
}

impl<'a> MoveGenerator<'a> {
    pub fn new(gaddag: &'a Gaddag) -> Self {
        Self { gaddag }
    }

    pub fn generate_all_moves(&self, board: &Board, rack: &Rack) -> HashSet<CrosswordMove> {
        // Initialize the empty moves set
        let mut moves: HashSet<CrosswordMove> = HashSet::new();

        // Store explored anchors
        let mut explored_anchors: [bool; TOTAL_SIZE] = [false; TOTAL_SIZE];

        // Create Immutable move context block
        let move_context: GeneratorContext<'_> = GeneratorContext::new(board, rack);

        // Start generating moves
        for index in 0..TOTAL_SIZE {
            if !board.is_anchor(index) {
                continue;
            }

            // Generate moves for anchor
            self.generate_moves_for_anchor(&mut moves, &mut explored_anchors, &move_context, index);

            // Mark the anchor as explored
            explored_anchors[index] = true;
        }

        moves
    }

    fn generate_moves_for_anchor(
        &self,
        moves: &mut HashSet<CrosswordMove>,
        explored_anchors: &mut [bool; TOTAL_SIZE],
        generator_ctx: &GeneratorContext,
        anchor: usize,
    ) {
        let row = anchor / BOARD_SIZE;
        let col = anchor % BOARD_SIZE;

        let mut hori_buffer = generator_ctx.hori_buffers[row].clone();
        let mut vert_buffer = generator_ctx.vert_buffers[col].clone();

        let mut horizontal_ctx: GenerationContext =
            GenerationContext::new(hori_buffer, Step::LEFT, true);
        let mut vertical_ctx: GenerationContext =
            GenerationContext::new(hori_buffer, Step::LEFT, true);

        self.extend_backwards(
            generator_ctx,
            self.gaddag.get_root(),
            hori_buffer,
            row,
            generator_ctx.rack.len,
            &Step::LEFT,
            true,
        );
        self.extend_backwards(
            generator_ctx,
            self.gaddag.get_root(),
            vert_buffer,
            col,
            generator_ctx.rack.len,
            &Step::UP,
            false,
        );
    }

    fn extend_backwards(
        &self,
        move_context: &GeneratorContext,
        node: &GaddagNode,
        buffer: [char; BOARD_SIZE],
        depth: usize,
        limit: usize,
        step: &Step,
        is_horizontal: bool,
    ) {
        // Move generation logic starts here
    }

    fn extend_forwards(
        &self,
        move_context: &GeneratorContext,
        node: &GaddagNode,
        buffer: &mut [usize; BOARD_SIZE],
        depth: usize,
        limit: usize,
        step: &Step,
    ) {
        // And continues here
    }

    // Helper functions
    fn get_cross_line(
        &self,
        move_context: &'a GeneratorContext,
        depth: usize,
        is_horizontal: bool,
    ) -> &'a [char; BOARD_SIZE] {
        if is_horizontal {
            &move_context.hori_buffers[depth]
        } else {
            &move_context.vert_buffers[depth]
        }
    }

    fn is_crossword_valid(
        &self,
        move_context: &GeneratorContext,
        placed_tile: char,
        depth: usize,
        is_horizontal: bool,
    ) -> bool {
        let crossline = self.get_cross_line(move_context, depth, is_horizontal);

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
}
