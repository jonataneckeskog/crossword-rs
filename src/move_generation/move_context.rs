#![allow(dead_code)]

use std::collections::HashSet;

use crate::constants::{BOARD_SIZE, BoardPosition, EMPTY_TILE, PIVOT, RACK_SIZE, TOTAL_SIZE};
use crate::core::{Board, CrosswordMove, Rack};
use crate::move_generation::gaddag::GaddagNode;

pub struct GeneratorContext {
    // Store values
    pub moves: HashSet<CrosswordMove>,
    pub explored_anchors: [bool; TOTAL_SIZE],

    // Precomputer buffers
    pub hori_buffers: [[char; BOARD_SIZE]; BOARD_SIZE],
    pub vert_buffers: [[char; BOARD_SIZE]; BOARD_SIZE],
}

pub struct RecursionContext<'a> {
    // Anchor we are generating for
    pub anchor: usize,

    // Keep track of move
    pub current_tiles: [char; RACK_SIZE],
    pub current_positions: [BoardPosition; RACK_SIZE],
    pub current_move_len: u8,

    // Recursion logic
    pub node: &'a GaddagNode,
    pub rack: &'a mut Rack,
    pub buffer: [char; BOARD_SIZE],
    pub depth: i32,
    pub is_horizontal: bool,
    pub is_forwards: bool,
}

#[derive(Debug)]
pub enum ExtendAction {
    PlaceFromRack(usize, char),
    TraverseExisting(),
    TraversePivot(),
}

impl GeneratorContext {
    pub fn new(board: &Board) -> Self {
        let moves = HashSet::new();
        let explored_anchors = [false; TOTAL_SIZE];

        let mut hori_buffers = [[EMPTY_TILE; BOARD_SIZE]; BOARD_SIZE];
        let mut vert_buffers = [[EMPTY_TILE; BOARD_SIZE]; BOARD_SIZE];

        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                let tile = board.get(y * BOARD_SIZE + x);
                hori_buffers[y][x] = tile;
                vert_buffers[x][y] = tile;
            }
        }

        Self {
            moves,
            explored_anchors,
            hori_buffers,
            vert_buffers,
        }
    }
}

impl<'a> RecursionContext<'a> {
    pub fn new(
        anchor: usize,
        node: &'a GaddagNode,
        rack: &'a mut Rack,
        buffer: [char; BOARD_SIZE],
        depth: i32,
        is_horizontal: bool,
        is_forwards: bool,
    ) -> Self {
        Self {
            anchor,
            current_tiles: [EMPTY_TILE; RACK_SIZE],
            current_positions: [0; RACK_SIZE],
            current_move_len: 0,
            node,
            rack,
            buffer,
            depth,
            is_horizontal,
            is_forwards,
        }
    }

    #[inline]
    // Returns depth as a usize (for convenience)
    pub fn depth(&self) -> usize {
        self.depth as usize
    }

    #[inline]
    /// True when the depth is negative (used for backwards bounds checks)
    pub fn out_of_bounds_backwards(&self) -> bool {
        self.depth < 0
    }

    #[inline]
    /// True when the depth index has reached or passed the board size (for forwards checks)
    pub fn out_of_bounds_forwards(&self) -> bool {
        self.depth() >= BOARD_SIZE
    }

    #[inline]
    /// Convenience: is the current buffer position empty
    pub fn is_current_empty(&self) -> bool {
        self.current_tile() == EMPTY_TILE
    }

    #[inline]
    /// Is there an existing tile immediately before the current depth?
    pub fn prev_tile_exists(&self) -> bool {
        self.depth > 0 && self.current_tile_with_mod(-1) != EMPTY_TILE
    }

    #[inline]
    /// Is there an existing tile immediately after the current depth?
    pub fn next_tile_exists(&self) -> bool {
        (self.depth() + 1) < BOARD_SIZE && self.current_tile_with_mod(1) != EMPTY_TILE
    }

    #[inline]
    /// Return the pivot child node if present (small helper to clarify intent)
    pub fn pivot_child(&self) -> Option<&'a GaddagNode> {
        self.node.get_child(PIVOT)
    }

    #[inline]
    pub fn position_at_depth(&self) -> usize {
        if self.is_horizontal {
            self.anchor + self.depth()
        } else {
            self.anchor + BOARD_SIZE * self.depth()
        }
    }

    #[inline]
    /// Return the starting square index along the row/column for this anchor.
    /// If horizontal, this is the column (anchor % BOARD_SIZE). If vertical,
    /// this is the row (anchor / BOARD_SIZE).
    pub fn starting_square(&self) -> usize {
        if self.is_horizontal {
            self.anchor % BOARD_SIZE
        } else {
            self.anchor / BOARD_SIZE
        }
    }

    #[inline]
    pub fn current_tile(&self) -> char {
        self.buffer[self.depth()]
    }

    #[inline]
    pub fn current_tile_with_mod(&self, modifyer: i32) -> char {
        self.buffer[(self.depth + modifyer) as usize]
    }

    pub fn extend(&mut self, action: &ExtendAction, new_node: &'a GaddagNode) {
        // Always update node
        self.update_node(new_node);

        match action {
            ExtendAction::PlaceFromRack(idx, tile) => {
                self.update_move(*idx, *tile);
            }
            ExtendAction::TraverseExisting() => {
                self.update_depth_forward();
            }
            ExtendAction::TraversePivot() => {
                self.update_direction();
                return;
            }
        }
    }

    pub fn undo(&mut self, action: &ExtendAction, previous_node: &'a GaddagNode) {
        // Always update node
        self.update_node(previous_node);

        match action {
            ExtendAction::TraversePivot() => {
                self.update_direction();
                return;
            }
            ExtendAction::PlaceFromRack(idx, _) => {
                self.revert_move(*idx);
            }
            ExtendAction::TraverseExisting() => {
                self.update_depth_backward();
            }
        }
    }

    fn update_node(&mut self, node: &'a GaddagNode) {
        self.node = node;
    }

    fn update_move(&mut self, rack_idx: usize, tile: char) {
        self.rack.mark_used(rack_idx);
        self.buffer[self.depth()] = tile;
        let move_len = self.current_move_len as usize;
        self.current_tiles[move_len] = tile;
        self.current_positions[move_len] = self.position_at_depth() as BoardPosition;
        self.current_move_len += 1;
    }

    fn revert_move(&mut self, rack_idx: usize) {
        self.rack.unmark_used(rack_idx);
        self.current_move_len -= 1;
        let move_len = self.current_move_len as usize;
        self.buffer[self.depth()] = EMPTY_TILE;
        self.current_positions[move_len] = 0;
        self.current_tiles[move_len] = EMPTY_TILE;
    }

    fn update_depth_forward(&mut self) {
        if self.is_forwards {
            self.depth += 1;
        } else {
            self.depth = self.depth.saturating_sub(1);
        }
    }

    fn update_depth_backward(&mut self) {
        if self.is_forwards {
            self.depth = self.depth.saturating_sub(1);
        } else {
            self.depth += 1;
        }
    }

    fn update_direction(&mut self) {
        self.is_forwards = !self.is_forwards;
    }

    fn debug_log(ctx: &RecursionContext, action: &ExtendAction) {
        println!(
            "[DEBUG] action={:?}, depth={}, anchor={}, is_horizontal={}, is_forwards={}",
            action, ctx.depth, ctx.anchor, ctx.is_horizontal, ctx.is_forwards
        );

        // Print current buffer row/column
        println!("Buffer: {:?}", ctx.buffer);

        // Print current move so far
        if ctx.current_move_len > 0 {
            let tiles: Vec<_> = ctx.current_tiles[..ctx.current_move_len as usize].to_vec();
            let positions: Vec<_> = ctx.current_positions[..ctx.current_move_len as usize].to_vec();
            println!("Move so far: {:?} at positions {:?}", tiles, positions);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{BOARD_SIZE, EMPTY_TILE};
    use crate::move_generation::gaddag::Gaddag;

    #[test]
    fn out_of_bounds_helpers_work() {
        // backward out of bounds when depth < 0
        let mut rack = crate::core::Rack::from_arrays([EMPTY_TILE; 7], 7);
        let gaddag = Gaddag::from_wordlist(&vec![]);
        let root = gaddag.get_root();

        let ctx = RecursionContext::new(
            0,
            root,
            &mut rack,
            [EMPTY_TILE; BOARD_SIZE],
            -1,
            true,
            false,
        );
        assert!(ctx.out_of_bounds_backwards());

        let ctx2 = RecursionContext::new(
            0,
            root,
            &mut rack,
            [EMPTY_TILE; BOARD_SIZE],
            BOARD_SIZE as i32,
            true,
            true,
        );
        assert!(ctx2.out_of_bounds_forwards());
    }

    #[test]
    fn tile_existence_helpers_and_position() {
        let mut rack = crate::core::Rack::from_arrays([EMPTY_TILE; 7], 7);
        let gaddag = Gaddag::from_wordlist(&vec![]);
        let root = gaddag.get_root();

        let mut buffer = [EMPTY_TILE; BOARD_SIZE];
        buffer[0] = 'X';
        buffer[1] = EMPTY_TILE;
        buffer[2] = 'Y';

        // depth = 1 -> prev exists (index 0), next exists (index 2)
        let ctx = RecursionContext::new(5, root, &mut rack, buffer, 1, true, false);
        assert!(ctx.prev_tile_exists());
        assert!(ctx.next_tile_exists());

        // current at depth 1 is EMPTY_TILE
        assert!(ctx.is_current_empty());

        // position_at_depth respects horizontal/vertical
        let pos_h =
            RecursionContext::new(5, root, &mut rack, buffer, 2, true, false).position_at_depth();
        assert_eq!(pos_h, 5 + 2);

        let pos_v =
            RecursionContext::new(5, root, &mut rack, buffer, 2, false, false).position_at_depth();
        assert_eq!(pos_v, 5 + BOARD_SIZE * 2);
    }

    #[test]
    fn pivot_child_is_found_when_present() {
        let mut rack = crate::core::Rack::from_arrays([EMPTY_TILE; 7], 7);
        let gaddag = Gaddag::from_wordlist(&vec![]);
        let root = gaddag.get_root();

        let ctx =
            RecursionContext::new(0, root, &mut rack, [EMPTY_TILE; BOARD_SIZE], 0, true, false);
        // Empty gaddag means no pivot child
        assert!(ctx.pivot_child().is_none());
    }

    #[test]
    fn extend_and_undo_place_from_rack_forward() {
        use crate::constants::{BOARD_SIZE, RACK_SIZE};

        let mut tiles = [EMPTY_TILE; RACK_SIZE];
        tiles[2] = 'B';
        let mut rack = crate::core::Rack::from_arrays(tiles, 1);

        let gaddag = Gaddag::from_wordlist(&vec![]);
        let root = gaddag.get_root();

        let buffer = [EMPTY_TILE; BOARD_SIZE];
        let anchor = 10usize;
        let mut ctx = RecursionContext::new(anchor, root, &mut rack, buffer, 0, true, true);

        let previous_node = ctx.node;

        // Place tile
        ctx.extend(&ExtendAction::PlaceFromRack(2, 'B'), root);

        // After placing: move data updated, but depth unchanged
        assert_eq!(ctx.current_move_len, 1);
        assert_eq!(ctx.current_tiles[0], 'B');
        assert_eq!(ctx.current_positions[0], anchor as BoardPosition);
        assert_eq!(ctx.buffer[0], 'B');

        // Depth should remain unchanged (0)
        assert_eq!(ctx.depth(), 0);

        // Undo the placement
        ctx.undo(&ExtendAction::PlaceFromRack(2, 'B'), previous_node);

        // Move cleared, buffer cleared, rack restored
        assert_eq!(ctx.depth(), 0);
        assert_eq!(ctx.current_move_len, 0);
        assert_eq!(ctx.buffer[0], EMPTY_TILE);
        assert_eq!(ctx.current_tiles[0], EMPTY_TILE);
        assert_eq!(ctx.current_positions[0], 0);
    }

    #[test]
    fn traverse_pivot_toggles_and_undo_restores_direction() {
        let mut rack = crate::core::Rack::from_arrays([EMPTY_TILE; 7], 7);
        let gaddag = Gaddag::from_wordlist(&vec![]);
        let root = gaddag.get_root();

        let mut ctx =
            RecursionContext::new(0, root, &mut rack, [EMPTY_TILE; BOARD_SIZE], 3, true, true);

        let prev_depth = ctx.depth();
        let prev_dir = ctx.is_forwards;

        // Extend with pivot should flip direction and not change depth
        ctx.extend(&ExtendAction::TraversePivot(), root);
        assert_eq!(ctx.depth(), prev_depth);
        assert_eq!(ctx.is_forwards, !prev_dir);

        // Undo pivot should flip direction back and leave depth unchanged
        ctx.undo(&ExtendAction::TraversePivot(), root);
        assert_eq!(ctx.is_forwards, prev_dir);
        assert_eq!(ctx.depth(), prev_depth);
    }

    #[test]
    fn generator_context_buffers_reflect_board() {
        use crate::core::Board;

        let mut board = Board::new();
        // Place two tiles at different positions
        board.place('X', 0 as BoardPosition);
        board.place('Y', 17 as BoardPosition); // row 1, col 2

        let ctx = GeneratorContext::new(&board);

        // Check horizontal buffer (rows)
        let x0 = ctx.hori_buffers[0][0];
        assert_eq!(x0, 'X');

        // position 17 -> row = 17 / 15 = 1, col = 2
        assert_eq!(ctx.hori_buffers[1][2], 'Y');

        // Vertical buffers are transposed
        assert_eq!(ctx.vert_buffers[0][0], 'X');
        assert_eq!(ctx.vert_buffers[2][1], 'Y');
    }
}
