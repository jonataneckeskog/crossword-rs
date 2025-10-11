#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
pub struct LetterData {
    pub count: u32,
    pub score: u32,
}

// -------------------------
// Bag Constants
// -------------------------

// Immutable mapping from character to LetterData (static)
// Note: requires `phf` crate in Cargo.toml (e.g. phf = "0.10")
pub static TILE_DATA: phf::Map<char, LetterData> = phf::phf_map! {
    'A' => LetterData { count: 9, score: 1 },
    'B' => LetterData { count: 2, score: 3 },
    'C' => LetterData { count: 2, score: 3 },
    'D' => LetterData { count: 4, score: 2 },
    'E' => LetterData { count: 12, score: 1 },
    'F' => LetterData { count: 2, score: 4 },
    'G' => LetterData { count: 3, score: 2 },
    'H' => LetterData { count: 2, score: 4 },
    'I' => LetterData { count: 9, score: 1 },
    'J' => LetterData { count: 1, score: 8 },
    'K' => LetterData { count: 1, score: 5 },
    'L' => LetterData { count: 4, score: 1 },
    'M' => LetterData { count: 2, score: 3 },
    'N' => LetterData { count: 6, score: 1 },
    'O' => LetterData { count: 8, score: 1 },
    'P' => LetterData { count: 2, score: 3 },
    'Q' => LetterData { count: 1, score: 10 },
    'R' => LetterData { count: 6, score: 1 },
    'S' => LetterData { count: 4, score: 1 },
    'T' => LetterData { count: 6, score: 1 },
    'U' => LetterData { count: 4, score: 1 },
    'V' => LetterData { count: 2, score: 4 },
    'W' => LetterData { count: 2, score: 4 },
    'X' => LetterData { count: 1, score: 8 },
    'Y' => LetterData { count: 2, score: 4 },
    'Z' => LetterData { count: 1, score: 10 },
    '_' => LetterData { count: 2, score: 0 },
};

/// Index -> char mapping (const array). KEEP this as the authoritative order for indices.
pub const INDEX_TO_CHAR: [char; 27] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '_',
];

/// The number of unique tiles — derived from INDEX_TO_CHAR so it is always correct at compile time.
pub const UNIQUE_TILES: usize = 27;

/// A convenience const array with size `UNIQUE_TILES` so you can do:
/// `let arr: [char; bag_constants::UNIQUE_TILES] = bag_constants::INDEX_TO_CHAR_ARRAY;`
pub const INDEX_TO_CHAR_ARRAY: [char; UNIQUE_TILES] = INDEX_TO_CHAR;

pub fn get_index(letter: &char) -> usize {
    INDEX_TO_CHAR
        .iter()
        .position(|&c| c == letter.to_ascii_uppercase())
        .expect(&format!("Invalid tile: {}", letter))
}

pub fn is_valid_letter(letter: &char) -> bool {
    TILE_DATA.contains_key(&letter.to_ascii_uppercase())
}

// -------------------------
// Board Constants
// -------------------------

pub type BoardPosition = u8;

pub const NORMAL: u8 = 0;
pub const DOUBLE_LETTER: u8 = 1;
pub const TRIPLE_LETTER: u8 = 2;
pub const DOUBLE_WORD: u8 = 3;
pub const TRIPLE_WORD: u8 = 4;
pub const QUADRUPLE_LETTER: u8 = 5;
pub const QUADRUPLE_WORD: u8 = 6;

pub const BOARD_SIZE: usize = 15;
pub const TOTAL_SIZE: usize = BOARD_SIZE * BOARD_SIZE;

// Compile-time assertion
const _: () = assert!(
    TOTAL_SIZE <= BoardPosition::MAX as usize,
    "BoardPosition type is too small for TOTAL_SIZE"
);

pub const TILE_BONUSES: [u8; TOTAL_SIZE] = [
    4, 0, 0, 1, 0, 0, 0, 4, 0, 0, 0, 1, 0, 0, 4, 0, 3, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0,
    3, 0, 0, 0, 1, 0, 1, 0, 0, 0, 3, 0, 0, 1, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 3, 0, 0, 1, 0, 0, 0, 0,
    3, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0,
    1, 0, 1, 0, 0, 0, 1, 0, 0, 4, 0, 0, 1, 0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 4, 0, 0, 1, 0, 0, 0, 1, 0,
    1, 0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0,
    3, 0, 0, 0, 0, 1, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 3, 0, 0, 1, 0, 0, 3, 0, 0, 0, 1, 0, 1, 0, 0, 0,
    3, 0, 0, 0, 3, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 3, 0, 4, 0, 0, 1, 0, 0, 0, 4, 0, 0, 0, 1, 0, 0,
    4,
];

// -------------------------
// Game Constants
// -------------------------
pub const EMPTY_TILE: char = '.';
pub const BINGO_BONUS: u32 = 50;
pub const RACK_SIZE: usize = 7;
