#![allow(dead_code)]

use crate::constants::{UNIQUE_TILES, get_index};

/// A GADDAG trie structure for efficient word lookup and Scrabble-like move generation.
pub struct Gaddag {
    root: GaddagNode,
}

/// # Fields
/// - 'is_word': Indicates if the node, and thereby the path to the node, is a word.
/// - 'children': An array of optional child nodes, each corresponding to a valid
///   tile character. The tail node is the pivot (represented by '>') from which
///   suffixes are stored backwards.
pub struct GaddagNode {
    is_word: bool,
    children: [Option<Box<GaddagNode>>; UNIQUE_TILES + 1],
}

impl Gaddag {
    pub fn from_wordlist(words: &Vec<String>) -> Self {
        let mut gaddag = Self {
            root: GaddagNode::new(),
        };
        for word in words {
            gaddag.root.insert_gaddag(word);
        }
        gaddag
    }
}

impl GaddagNode {
    fn new() -> Self {
        Self {
            is_word: false,
            children: Default::default(),
        }
    }

    // Creates paths that are then inserted
    fn insert_gaddag(&mut self, word: &String) {
        let chars: Vec<char> = word.chars().collect();

        for i in 0..=chars.len() {
            let mut path = Vec::new();

            // reversed prefix
            for &c in chars[..i].iter().rev() {
                path.push(c);
            }

            // pivot
            path.push('>');

            // suffix
            for &c in &chars[i..] {
                path.push(c);
            }

            self.insert_path(&path);
        }
    }

    fn insert_path(&mut self, path: &[char]) {
        let mut current_node = self;

        for &tile in path {
            let idx = get_index(tile);
            current_node = current_node.children[idx]
                .get_or_insert_with(|| Box::new(GaddagNode::new()))
                .as_mut();
        }

        current_node.is_word = true;
    }

    pub fn has_child(&self, tile: char) -> bool {
        let idx = get_index(tile);
        self.children[idx].is_some()
    }

    pub fn get_child(&self, tile: char) -> &Option<Box<GaddagNode>> {
        let idx = get_index(tile);
        &self.children[idx]
    }

    pub fn is_word(&self) -> bool {
        self.is_word
    }
}
