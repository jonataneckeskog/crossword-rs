use crate::constants::{BOARD_SIZE, EMPTY_TILE};
use crate::core::CrosswordMove;
use crate::move_generation::{MoveGenerator, move_context::*};

impl<'a> MoveGenerator<'a> {
    pub fn get_cross_line<'b>(
        &self,
        gen_ctx: &'b GeneratorContext,
        depth: usize,
        is_horizontal: bool,
    ) -> &'b [char; BOARD_SIZE] {
        // Return the line perpendicular to the move direction. If the move is
        // horizontal, crosswords run vertically (columns) and vice versa.
        if is_horizontal {
            &gen_ctx.vert_buffers[depth]
        } else {
            &gen_ctx.hori_buffers[depth]
        }
    }

    pub fn is_crossword_valid(
        &self,
        gen_ctx: &GeneratorContext,
        placed_tile: char,
        board_pos: usize,
        is_horizontal: bool,
    ) -> bool {
        // Determine which line (row/column) to check and the index within that line
        let line_idx = if is_horizontal {
            // For a horizontal move, crosswords run vertically -> column index
            board_pos % BOARD_SIZE
        } else {
            // For a vertical move, crosswords run horizontally -> row index
            board_pos / BOARD_SIZE
        };

        let idx_in_line = if is_horizontal {
            // index in column = row
            board_pos / BOARD_SIZE
        } else {
            // index in row = column
            board_pos % BOARD_SIZE
        };

        let crossline = self.get_cross_line(gen_ctx, line_idx, is_horizontal);

        let mut start = idx_in_line;
        let mut end = idx_in_line;
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

        // Build the crossword word (with the placed tile at `idx_in_line`) and use
        // the GADDAG's is_word method to check validity
        let mut word = String::with_capacity(length);
        for i in start..=end {
            let tile = if i == idx_in_line {
                placed_tile
            } else {
                crossline[i]
            };
            word.push(tile);
        }

        // DEBUG: show what word we're checking in tests
        println!(
            "[DEBUG is_crossword_valid] board_pos={}, is_horizontal={}, line_idx={}, start={}, end={}, word={}",
            board_pos, is_horizontal, line_idx, start, end, word
        );

        self.gaddag.is_word(&word)
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

    pub fn handle_empty_tile(&self, gen_ctx: &mut GeneratorContext, ctx: &'a mut RecursionContext) {
        let tiles: Vec<_> = ctx.rack.available_tiles().collect(); // Cannot iterate over while changing
        for (idx, tile) in tiles {
            if !self.is_crossword_valid(gen_ctx, tile, ctx.position_at_depth(), ctx.is_horizontal) {
                continue;
            }

            if let Some(next_node) = ctx.node.get_child(tile) {
                let previous_node = ctx.node;
                let action = ExtendAction::PlaceFromRack(idx, tile);
                ctx.extend(&action, next_node);
                if ctx.is_forwards {
                    self.extend_forwards(gen_ctx, ctx);
                } else {
                    self.extend_backwards(gen_ctx, ctx);
                }
                ctx.undo(&action, previous_node);
            }
        }
    }

    pub fn follow_existing_tiles(
        &self,
        gen_ctx: &mut GeneratorContext,
        ctx: &'a mut RecursionContext,
    ) {
        let step = if ctx.is_forwards { 1 } else { -1 };
        let tile = ctx.current_tile_with_mod(step);
        if let Some(next_node) = ctx.node.get_child(tile) {
            let previous_node = ctx.node;
            let action: ExtendAction = ExtendAction::TraverseExisting();
            ctx.extend(&action, next_node);
            if ctx.is_forwards {
                    self.extend_forwards(gen_ctx, ctx);
            } else {
                self.extend_backwards(gen_ctx, ctx);
            }
            ctx.undo(&action, previous_node);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{BoardPosition, RACK_SIZE, TOTAL_SIZE};
    use crate::core::{Board, Rack};
    use crate::move_generation::gaddag::Gaddag;

    struct Context {
        gaddag: Gaddag,
        board: Board,
        rack: Rack,
        gen_ctx: GeneratorContext,
    }

    fn default_setup() -> Context {
        return setup(Vec::new(), "", "");
    }

    fn setup(valid_words: Vec<&str>, rack_letters: &str, placed_word: &str) -> Context {
        let gaddag = Gaddag::from_wordlist(
            &valid_words
                .into_iter()
                .map(String::from)
                .collect::<Vec<String>>(),
        );
        let mut board = Board::new();
        let start = TOTAL_SIZE / 2;
        for (i, tile) in placed_word.chars().enumerate() {
            board.place(tile, (start + i) as BoardPosition);
        }

        let mut tiles = [EMPTY_TILE; RACK_SIZE];
        for (i, tile) in rack_letters.chars().enumerate() {
            tiles[i] = tile;
        }
        let rack = Rack::from_arrays(tiles, rack_letters.len());
        let gen_ctx = GeneratorContext::new(&board);

        Context {
            gaddag: gaddag,
            board: board,
            rack: rack,
            gen_ctx: gen_ctx,
        }
    }

    #[test]
    fn test_get_cross_line() {
        let setup = setup(vec!["CAT", "CATS"], "CATS", "CAT");
        let generator = MoveGenerator::new(&setup.gaddag);

        let crossline_1 = generator.get_cross_line(&setup.gen_ctx, 1, false);
        for tile in crossline_1.into_iter() {
            assert!(*tile == EMPTY_TILE);
        }

        let crossline_2 = generator.get_cross_line(&setup.gen_ctx, 7, false);
        assert!(
            crossline_2
                .into_iter()
                .filter(|&tile| *tile != EMPTY_TILE)
                .count()
                == 3
        );
    }

    #[test]
    fn test_is_crossword_valid() {
        let setup = setup(vec!["CAT", "CATS"], "CATS", "CAT");
        let generator = MoveGenerator::new(&setup.gaddag);

        let start = TOTAL_SIZE / 2;
        // Place 'S' just after the existing placed word horizontally (row)
        let pos_after = (start + 3) as usize; // one past the placed "CAT"
        // Check placing 'S' vertically at the middle of the placed word's column
        let pos_middle = (start + 1) as usize;

        assert!(generator.is_crossword_valid(&setup.gen_ctx, 'S', pos_after, false));
        assert!(generator.is_crossword_valid(&setup.gen_ctx, 'S', pos_middle, true));
        assert!(!generator.is_crossword_valid(&setup.gen_ctx, 'C', pos_after, false));
    }
}
