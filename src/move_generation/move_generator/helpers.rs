use crate::constants::{BOARD_SIZE, EMPTY_TILE};
use crate::core::CrosswordMove;
use crate::move_generation::{MoveGenerator, gaddag::*, move_context::*};

impl<'a> MoveGenerator<'a> {
    pub fn get_cross_line<'b>(
        &self,
        gen_ctx: &'b GeneratorContext,
        depth: usize,
        is_horizontal: bool,
    ) -> &'b [char; BOARD_SIZE] {
        if is_horizontal {
            &gen_ctx.hori_buffers[depth]
        } else {
            &gen_ctx.vert_buffers[depth]
        }
    }

    pub fn is_crossword_valid(
        &self,
        gen_ctx: &GeneratorContext,
        placed_tile: char,
        depth: usize,
        is_horizontal: bool,
    ) -> bool {
        let crossline = self.get_cross_line(gen_ctx, depth, is_horizontal);

        let mut start = depth;
        let mut end = depth;
        while start > 0 && crossline[start - 1] != EMPTY_TILE {
            start -= 1;
        }
        while end + 1 < BOARD_SIZE && crossline[end + 1] != EMPTY_TILE {
            end += 1;
        }

        let length = end - start + 1;
        if length == 1 {
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

    pub fn record_move(&self, gen_ctx: &mut GeneratorContext, rec_ctx: &RecursionContext) {
        // Once again, just copying
        let crossword_move = CrosswordMove::from_arrays(
            rec_ctx.current_tiles,
            rec_ctx.current_positions,
            rec_ctx.current_move_len,
        );
        gen_ctx.moves.insert(crossword_move);
    }
}
