#![allow(dead_code)]

use crate::constants::{PIVOT, PIVOT_BIT_IDX, TileBitboard, get_index};

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
            // PIVOT is not part of the normal tile index table. Treat it as a
            // reserved high bit so the gaddag can store a pivot child without
            // changing the global get_index behavior.
            let idx = if tile == PIVOT {
                PIVOT_BIT_IDX
            } else {
                get_index(tile) as u32
            };
            let bit: TileBitboard = (1 as TileBitboard) << idx;

            // Count the number of children before this index
            let lower_mask: TileBitboard = ((1 as TileBitboard) << idx) - 1;
            let pos = (node.children_mask & lower_mask).count_ones() as usize;

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
        let idx = if tile == PIVOT {
            PIVOT_BIT_IDX
        } else {
            get_index(tile) as u32
        };
        let bit: TileBitboard = (1 as TileBitboard) << idx;

        if self.children_mask & bit == 0 {
            return None;
        }

        // Count number of children before this tile to get vector index
        let lower_mask: TileBitboard = ((1 as TileBitboard) << idx) - 1;
        let pos = (self.children_mask & lower_mask).count_ones() as usize;
        Some(&self.children_ptrs[pos])
    }

    pub fn is_word(&self) -> bool {
        self.is_word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn traverse<'a>(node: &'a GaddagNode, path: &[char]) -> Option<&'a GaddagNode> {
        let mut cur: &'a GaddagNode = node;
        for &c in path {
            cur = cur.get_child(c)?;
        }
        Some(cur)
    }

    #[test]
    fn single_word_basic_paths() {
        // Insert a simple path C A T (no pivot) using insert_path directly to avoid
        // depending on PIVOT handling in insert_gaddag.
        let mut root = GaddagNode::new();
        root.insert_path(&['C', 'A', 'T']);

        // Check full path exists and is marked as a word
        let n = traverse(&root, &['C', 'A', 'T']).expect("path should exist");
        assert!(n.is_word(), "CAT should be a word");

        // Check prefix nodes exist but are not words (C and CA)
        let c = traverse(&root, &['C']).expect("C node should exist");
        assert!(!c.is_word(), "C should not be marked as a word");

        let ca = traverse(&root, &['C', 'A']).expect("CA node should exist");
        assert!(!ca.is_word(), "CA should not be marked as a word");

        // Non-existing child
        assert!(traverse(&root, &['C', 'X']).is_none());
    }

    #[test]
    fn multiple_words_shared_nodes() {
        // Use direct insert_path to create CAT and CATS without pivots
        let mut root = GaddagNode::new();
        root.insert_path(&['C', 'A', 'T']);
        root.insert_path(&['C', 'A', 'T', 'S']);

        // CAT present
        let cat = traverse(&root, &['C', 'A', 'T']).expect("CAT path");
        assert!(cat.is_word(), "CAT should be present as a word");

        // CATS present (longer)
        let cats = traverse(&root, &['C', 'A', 'T', 'S']).expect("CATS path");
        assert!(cats.is_word(), "CATS should be present as a word");

        // Ensure CAT node is still a word (prefix)
        assert!(
            cat.is_word(),
            "CAT should remain a word after inserting CATS"
        );
    }

    #[test]
    fn insert_path_marks_word() {
        // Directly use insert_path on a fresh node
        let mut node = GaddagNode::new();
        node.insert_path(&['M', 'A', 'N']);
        let n = traverse(&node, &['M', 'A', 'N']).expect("MAN path");
        assert!(n.is_word(), "Inserted path should be marked as a word");
    }
}
