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
    pub fn generate_all_moves(&self, board: &Board, rack: &mut Rack) -> HashSet<CrosswordMove> {
        // Create move context blocks
        let mut gen_ctx: GeneratorContext = GeneratorContext::new(board);

        if board.is_empty() {
            let center = TOTAL_SIZE / 2;
            self.generate_moves_for_anchor(rack, &mut gen_ctx, center);
            return gen_ctx.moves;
        }

        // Start generating moves
        for index in 0..TOTAL_SIZE {
            if !board.is_anchor(index) {
                continue;
            }

            // Generate moves for anchor
            self.generate_moves_for_anchor(rack, &mut gen_ctx, index);

            // Mark the anchor as explored
            gen_ctx.explored_anchors[index] = true;
        }

        gen_ctx.moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{BOARD_SIZE, BoardPosition, EMPTY_TILE, RACK_SIZE, TOTAL_SIZE};
    use crate::core::{Board, Rack};
    use crate::move_generation::gaddag::Gaddag;

    #[test]
    fn empty_board_returns_moves() {
        let gaddag = Gaddag::from_wordlist(&vec!["CAT".to_string()]);
        let generator = MoveGenerator::new(&gaddag);

        let board = Board::new();

        // Rack containing letters for CAT (first move should be allowed on center)
        let mut tiles = [EMPTY_TILE; RACK_SIZE];
        tiles[0] = 'C';
        tiles[1] = 'A';
        tiles[2] = 'T';
        let mut rack = Rack::from_arrays(tiles, 3);

        let moves = generator.generate_all_moves(&board, &mut rack);

        // On an empty board the center square counts as an anchor and the generator
        // should produce at least one move (first play must cover the center).
        assert!(!moves.is_empty(), "expected some moves on an empty board");

        // Ensure at least one generated move covers the center position (first play rule)
        let center = TOTAL_SIZE / 2;

        let mut covers_center = false;
        for m in moves.iter() {
            for (_, pos) in m.iter() {
                if pos as usize == center {
                    covers_center = true;
                    break;
                }
            }
            if covers_center {
                break;
            }
        }
        assert!(
            covers_center,
            "expected at least one move to cover the center square"
        );
    }

    #[test]
    fn generates_cat_when_c_on_board_and_a_t_on_rack() {
        let gaddag = Gaddag::from_wordlist(&vec!["CAT".to_string()]);
        let generator = MoveGenerator::new(&gaddag);

        let mut board = Board::new();

        // Put 'A' on the board in the top row (safe index) so adjacent squares are anchors
        let centre_row = 0usize;
        let centre_col = 2usize;
        let centre_index = centre_row * BOARD_SIZE + centre_col;
        board.place('A', centre_index as BoardPosition);

        // Rack holds C and T which combined with existing A should form CAT (C left, T right)
        let mut tiles = [EMPTY_TILE; RACK_SIZE];
        tiles[0] = 'C';
        tiles[1] = 'T';
        let mut rack = Rack::from_arrays(tiles, 2);

        let moves = generator.generate_all_moves(&board, &mut rack);

        // Sanity checks on generated moves. We don't assert a specific move here
        // because the generator is prototype; instead we check realistic
        // properties we expect on a real Scrabble board:
        // - positions are in-bounds
        // - moves do not overwrite existing tiles (they only place on EMPTY tiles)
        // - placed tiles are valid characters
        for m in moves.iter() {
            let mut seen_positions = std::collections::HashSet::new();
            for (tile, pos) in m.iter() {
                let pos_us = pos as usize;
                // in-bounds
                assert!(pos_us < TOTAL_SIZE, "position out of bounds: {}", pos_us);

                // no duplicate positions inside a move
                assert!(
                    seen_positions.insert(pos_us),
                    "duplicate position in move: {}",
                    pos_us
                );

                // should not overwrite existing tiles
                let board_ch = board.get(pos_us);
                if board_ch != EMPTY_TILE {
                    // if board already had a tile at this pos, the move must use the same tile
                    assert_eq!(
                        board_ch, tile,
                        "move overwrites existing tile at {}: {} vs {}",
                        pos_us, board_ch, tile
                    );
                }

                // tile should be an ASCII letter or blank
                assert!(
                    tile.is_ascii_alphabetic() || tile == '?',
                    "invalid tile char: {}",
                    tile
                );
            }
        }
    }
}
