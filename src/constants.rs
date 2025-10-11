pub type BoardPosition = u8;

pub const BOARD_SIZE: usize = 15;
pub const TOTAL_SIZE: usize = BOARD_SIZE * BOARD_SIZE;
pub const RACK_SIZE: usize = 7;
pub const EMPTY_TILE: char = '.';

// Compile-time assertions (making sure the program won't work because of overflow)
const _: () = assert!(
    TOTAL_SIZE <= BoardPosition::MAX as usize,
    "BoardPosition type is too small for TOTAL_SIZE"
);
