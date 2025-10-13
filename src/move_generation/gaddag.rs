#![allow(dead_code)]

use crate::constants::{PIVOT, TileBitboard, get_index};

/// A GADDAG trie structure for efficient word lookup and Scrabble-like move generation.
pub struct Gaddag {
    root: GaddagNode,
}

/// # Fields
/// - 'is_word': Indicates if the node, and thereby the path to the node, is a word.
/// - 'children': An array of optional child nodes, each corresponding to a valid
///   tile character. The tail node is the pivot (represented by '>') from which
///   suffixes are stored backwards.
///
/// # Notes
/// - Letters at the start of the gaddag (from root) are all in reverse order. There is
///   no pivot from root.
pub struct GaddagNode {
    is_word: bool,
    children_mask: TileBitboard,
    children_ptrs: Vec<Box<GaddagNode>>,
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

    pub fn get_root(&self) -> &GaddagNode {
        &self.root
    }
}

impl GaddagNode {
    fn new() -> Self {
        Self {
            is_word: false,
            children_mask: 0,
            children_ptrs: Vec::new(),
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
            path.push(PIVOT);

            // suffix
            for &c in &chars[i..] {
                path.push(c);
            }

            self.insert_path(&path);
        }
    }

    pub fn insert_path(&mut self, path: &[char]) {
        let mut node = self;

        for (i, &tile) in path.iter().enumerate() {
            let idx = get_index(tile) as u32;
            let bit: TileBitboard = 1 << idx;

            // Count the number of children before this index
            let pos = (node.children_mask & ((1 << idx) - 1)).count_ones() as usize;

            // Check if the child exists
            if node.children_mask & bit != 0 {
                // Move to existing child
                node = &mut node.children_ptrs[pos];
            } else {
                // Create new child node
                let mut new_node = Box::new(GaddagNode::new());

                // Mark as a word if this is the last character in the path
                if i == path.len() - 1 {
                    new_node.is_word = true;
                }

                // Insert at the correct position to maintain mask order
                node.children_ptrs.insert(pos, new_node);
                node.children_mask |= bit;

                // Move to the new child
                node = &mut node.children_ptrs[pos];
            }
        }
    }

    pub fn get_child(&self, tile: char) -> Option<&GaddagNode> {
        let idx = get_index(tile) as u32;
        let bit: TileBitboard = 1 << idx;

        if self.children_mask & bit == 0 {
            return None;
        }

        // Count number of children before this tile to get vector index
        let pos = (self.children_mask & ((1 << idx) - 1)).count_ones() as usize;
        Some(&self.children_ptrs[pos])
    }

    pub fn is_word(&self) -> bool {
        self.is_word
    }
}
