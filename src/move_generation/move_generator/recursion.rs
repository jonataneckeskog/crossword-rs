#![allow(dead_code)]

use crate::constants::BOARD_SIZE;
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

        let hori_buffer = gen_ctx.hori_buffers[row];
        let vert_buffer = gen_ctx.vert_buffers[col];

        // Root is already a borrow, so it's fine to pass it directly
        let root: &GaddagNode = self.gaddag.get_root();

        let mut horizontal_ctx =
            RecursionContext::new(anchor, root, rack, hori_buffer, col as i32, true, false);
        self.extend_backwards(gen_ctx, &mut horizontal_ctx);

        let mut vertical_ctx =
            RecursionContext::new(anchor, root, rack, vert_buffer, row as i32, false, false);
        self.extend_backwards(gen_ctx, &mut vertical_ctx);
    }

    pub fn extend_backwards(&self, gen_ctx: &mut GeneratorContext, ctx: &mut RecursionContext<'_>) {
        if ctx.out_of_bounds_backwards() {
            return;
        }

        // Encountered an already explored anchor, stop recursion
        let anchor = ctx.position_at_depth();
        if gen_ctx.explored_anchors[anchor] {
            return;
        }

        if ctx.is_current_empty() {
            return self.handle_empty_tile(gen_ctx, ctx); // Does not change the depth
        }

        if ctx.prev_tile_exists() {
            return self.follow_existing_tiles(gen_ctx, ctx);
        }

        // No more backwards extentions, and we have: prefix is complete -> try extend forward
        if let Some(pivot_node) = ctx.pivot_child() {
            let previous_node = ctx.node;
            let action = ExtendAction::TraversePivot();

            let old_depth = ctx.depth;

            // Jump to the starting square before generating forwards
            ctx.depth = ctx.starting_square() as i32;
            ctx.extend(&action, pivot_node);
            self.extend_forwards(gen_ctx, ctx);
            ctx.undo(&action, previous_node);
            ctx.depth = old_depth;
        }

        if ctx.rack.is_empty() {
            return;
        }

        // Go to the empty square before the word
        // Note: Node is never updated here
        let previous_node = ctx.node;
        let action = ExtendAction::TraverseExisting();
        ctx.extend(&action, previous_node);
        self.extend_backwards(gen_ctx, ctx);
        ctx.undo(&action, previous_node);
    }

    pub fn extend_forwards(&self, gen_ctx: &mut GeneratorContext, ctx: &mut RecursionContext<'_>) {
        if ctx.out_of_bounds_forwards() {
            return;
        }

        if ctx.is_current_empty() {
            return self.handle_empty_tile(gen_ctx, ctx);
        }

        if ctx.next_tile_exists() {
            return self.follow_existing_tiles(gen_ctx, ctx);
        }

        println!("Buffer: {:?}", ctx.buffer);

        // Record move if conditions are met
        if ctx.node.is_word() && ctx.current_move_len > 0 {
            println!("Move: {:?}, Buffer {:?}", ctx.current_positions, ctx.buffer);
            self.record_move(gen_ctx, ctx);
        }

        if ctx.rack.is_empty() {
            return;
        }

        // Go to the next (empty) square without updating node
        let previous_node = ctx.node;
        let action = ExtendAction::TraverseExisting();
        ctx.extend(&action, previous_node);
        self.extend_forwards(gen_ctx, ctx);
        ctx.undo(&action, previous_node);
    }
}
