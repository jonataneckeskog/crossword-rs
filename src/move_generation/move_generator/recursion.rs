#![allow(dead_code)]

use crate::constants::BOARD_SIZE;
use crate::move_generation::{gaddag::GaddagNode, move_context::*, move_generator::MoveGenerator};

impl<'a> MoveGenerator<'a> {
    // Recursion context is made and owned here
    pub fn generate_moves_for_anchor(&self, gen_ctx: &mut GeneratorContext<'_>, anchor: usize) {
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
}
