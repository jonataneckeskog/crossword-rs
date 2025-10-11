mod board;
mod constants;
mod crossword_move;
mod gaddag;
mod movegenerator;
mod rack;

use crate::board::Board;
use crate::constants::BoardPosition;
use crate::crossword_move::CrosswordMove;

// Currently just a simple make a move example
fn main() {
    let mut board = Board::new();

    // Example
    let tiles = ['H', 'E', 'L', 'L', 'O', ' ', ' '];
    let positions: [BoardPosition; 7] = [49, 50, 51, 52, 53, 0, 0];

    let crossword_move = CrosswordMove::from_arrays(tiles, positions, 5);

    board.make_move(&crossword_move);

    // Print the board row by row
    for row in 0..crate::constants::BOARD_SIZE {
        for col in 0..crate::constants::BOARD_SIZE {
            let idx = row * crate::constants::BOARD_SIZE + col;
            print!("{}", board.get(idx));
        }
        println!();
    }
}
