use std::collections::HashSet;
use std::ops::Neg;

use crate::{
    board::Board,
    constants::{BOARD_SIZE, BoardPosition, RACK_SIZE, TOTAL_SIZE},
    crossword_move::CrosswordMove,
    gaddag::Gaddag,
    rack::Rack,
};

pub struct MoveGenerator<'a> {
    gaddag: &'a Gaddag,
}

struct MoveContext<'a> {
    rack: &'a Rack,

    // Quickly build moves
    current_letters: [char; RACK_SIZE],
    current_positions: [BoardPosition; RACK_SIZE],
    current_move_len: u8,

    // Precomputer buffers
    hori_buffers: [[char; BOARD_SIZE]; BOARD_SIZE],
    vert_buffers: [[char; BOARD_SIZE]; BOARD_SIZE],
}

pub struct Step(pub isize);

impl<'a> MoveGenerator<'a> {
    pub fn new(gaddag: &'a Gaddag) -> Self {
        Self { gaddag }
    }

    pub fn generate_all_moves(&self, board: &Board, rack: &Rack) -> HashSet<CrosswordMove> {
        // Create the MoveContext and initialize the empty moves set
        let mut move_context: MoveContext = MoveContext::new(board, rack);
        let mut moves: HashSet<CrosswordMove> = HashSet::new();

        // Start generating moves
        for index in 0..TOTAL_SIZE {
            if !board.is_anchor(index) {
                continue;
            }

            // Generate moves for anchor
            self.generate_moves_for_anchor(&mut moves, index, &mut move_context);
        }

        moves
    }

    fn generate_moves_for_anchor(
        &self,
        moves: &mut HashSet<CrosswordMove>,
        anchor: usize,
        ctx: &mut MoveContext,
    ) {
        let row = anchor / BOARD_SIZE;
        let col = anchor % BOARD_SIZE;

        let mut hori_buffer = ctx.hori_buffers[row].clone();
        let mut vert_buffer = ctx.vert_buffers[col].clone();

        self.extend_backwards(hori_buffer, row, ctx.rack.len, &Step::LEFT);
        self.extend_backwards(vert_buffer, col, ctx.rack.len, &Step::UP);
    }

    fn extend_backwards(
        &self,
        buffer: [char; BOARD_SIZE],
        depth: usize,
        limit: usize,
        step: &Step,
    ) {
        // Move generation logic starts here
    }

    fn extend_forwards(
        &self,
        buffer: &mut [usize; BOARD_SIZE],
        depth: usize,
        limit: usize,
        step: &Step,
    ) {
        // And continues here
    }
}

impl<'a> MoveContext<'a> {
    fn new(board: &'a Board, rack: &'a Rack) -> Self {
        let mut current_letters: [char; RACK_SIZE] = [' '; RACK_SIZE];
        let mut current_positions: [BoardPosition; RACK_SIZE] = [0; RACK_SIZE];
        let mut current_move_len: u8 = 0;
        let mut hori_buffers = [[' '; BOARD_SIZE]; BOARD_SIZE];
        let mut vert_buffers = [[' '; BOARD_SIZE]; BOARD_SIZE];

        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                let tile = board.get(y * BOARD_SIZE + x);
                hori_buffers[y][x] = tile;
                vert_buffers[x][y] = tile;
            }
        }

        Self {
            rack,
            current_letters,
            current_positions,
            current_move_len,
            hori_buffers,
            vert_buffers,
        }
    }
}

impl Step {
    pub const RIGHT: Step = Step(1);
    pub const LEFT: Step = Step(-1);
    pub const DOWN: Step = Step(BOARD_SIZE as isize);
    pub const UP: Step = Step(-(BOARD_SIZE as isize));
}

impl Neg for Step {
    type Output = Step;

    fn neg(self) -> Step {
        Step(-self.0)
    }
}
