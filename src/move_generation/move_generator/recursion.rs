#![allow(dead_code)]

use crate::constants::{BOARD_SIZE, EMPTY_TILE};
use crate::core::Rack;
use crate::move_generation::{gaddag::GaddagNode, move_context::*, move_generator::MoveGenerator};

impl<'a> MoveGenerator<'a> {
    // Recursion context is made and owned here
    pub fn generate_moves_for_anchor(
        &self,
        rack: &mut Rack,
        gen_ctx: &mut GeneratorContext,
        anchor: usize,
    ) {
        let row = anchor / BOARD_SIZE;
        let col = anchor % BOARD_SIZE;

        // Copy buffers (arrays implement Copy)
        // This should be O(BOARD_SIZE), but it's still extremely fast. To
        // get it to O(1) I could wrap the buffer in a Box "Box<[char]>",
        // but this is likely not faster for BOARD_SIZE = 15. Box forces it to be on
        // the heap, which is where the overhead comes from.
        let hori_buffer = gen_ctx.hori_buffers[row];
        let vert_buffer = gen_ctx.vert_buffers[col];

        // Root is already a borrow, so it's fine to pass it directly
        let root: &GaddagNode = self.gaddag.get_root();

        let mut horizontal_ctx = RecursionContext::new(anchor, root, rack, hori_buffer, row, true);
        self.extend_backwards(gen_ctx, &mut horizontal_ctx);

        let mut vertical_ctx = RecursionContext::new(anchor, root, rack, vert_buffer, col, false);
        self.extend_backwards(gen_ctx, &mut vertical_ctx);
    }

    fn extend_backwards(&self, gen_ctx: &mut GeneratorContext, ctx: &mut RecursionContext<'_>) {
        // Move generation logic starts here

        // Empty tile, try placing all letters
        if ctx.current_tile() == EMPTY_TILE {
            let tiles: Vec<_> = ctx.rack.available_tiles().collect(); // Cannot iterate over while changing
            for (idx, tile) in tiles {
                let valid = self.is_crossword_valid(gen_ctx, tile, ctx.depth, ctx.is_horizontal)
                    && ctx.node.get_child(tile).is_some();

                if valid {
                    let previous_node = ctx.node;
                    ctx.extend(idx, tile, previous_node.get_child(tile).unwrap(), false);
                    self.extend_forwards(gen_ctx, ctx);
                    ctx.undo(idx, previous_node, false);
                }
            }
            return;
        }

        // If the cell to the left is an occupied board cell, we must follow it.
        if ctx.depth > 0 && ctx.current_tile_with_mod(-1) != EMPTY_TILE {
            let tile = ctx.current_tile_with_mod(-1);
            if let Some(next_node) = ctx.node.get_child(tile) {
                let prev_node = ctx.node;
                // move left on board while moving forward on gaddag
                ctx.node = next_node;
                ctx.depth -= 1;
                self.extend_backwards(gen_ctx, ctx);
                // restore
                ctx.depth += 1;
                ctx.node = prev_node;
            }
            return;
        }

        // Full tile
        self.extend_forwards(gen_ctx, ctx);
    }

    fn extend_forwards(&self, gen_ctx: &mut GeneratorContext, ctx: &mut RecursionContext<'_>) {
        // And continues here
        if ctx.depth as usize >= BOARD_SIZE {
            return;
        }

        if ctx.node.is_word() {
            self.record_move(gen_ctx, ctx);
        }
    }
}
