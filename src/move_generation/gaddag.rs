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

    /// Check whether the exact word exists in the GADDAG.
    pub fn is_word(&self, word: &str) -> bool {
        let chars: Vec<char> = word.chars().collect();

        let mut node = self.get_root();

        for &c in chars.iter().rev() {
            if let Some(child) = node.get_child(c) {
                node = child;
            } else {
                return false;
            }
        }

        // Check pivot child (canonical representation uses pivot at end)
        if let Some(child) = node.get_child(PIVOT) {
            node = child;
        } else {
            return false;
        }

        node.is_word()
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
                let new_node = Box::new(GaddagNode::new());

                // Insert at the correct position to maintain mask order
                node.children_ptrs.insert(pos, new_node);
                node.children_mask |= bit;

                // Move to the new child
                node = &mut node.children_ptrs[pos];
            }

            // If this is the last character in the path, mark the node as a word.
            // Do this for both newly-created and existing nodes so insertion order
            // doesn't affect the 'is_word' flag.
            if i == path.len() - 1 {
                node.is_word = true;
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

    #[test]
    fn pivot_paths_from_insert_gaddag() {
        // Insert the word using insert_gaddag which creates paths containing the pivot
        let mut root = GaddagNode::new();
        root.insert_gaddag(&"CAT".to_string());

        // i = 0 path: [PIVOT, 'C', 'A', 'T']
        let p0 = traverse(&root, &[PIVOT, 'C', 'A', 'T']).expect("pivot-start path");
        assert!(p0.is_word(), "Path starting with pivot should be a word");

        // i = 1 path: ['C', PIVOT, 'A', 'T']
        let p1 = traverse(&root, &['C', PIVOT, 'A', 'T']).expect("pivot-middle path");
        assert!(p1.is_word(), "Path with pivot in middle should be a word");

        // i = 3 path: ['T', 'A', 'C', PIVOT]
        let p3 = traverse(&root, &['T', 'A', 'C', PIVOT]).expect("pivot-end path");
        assert!(p3.is_word(), "Path ending with pivot should be a word");
    }

    #[test]
    fn insert_ordering_marks_existing_node() {
        // Ensure inserting a longer path first then a shorter path still marks the
        // shorter path's endpoint as a word.
        let mut root = GaddagNode::new();

        // Insert C A T S first
        root.insert_path(&['C', 'A', 'T', 'S']);
        // Now insert CAT which ends on an existing node
        root.insert_path(&['C', 'A', 'T']);

        let cat = traverse(&root, &['C', 'A', 'T']).expect("CAT path after CATS");
        assert!(
            cat.is_word(),
            "CAT should be marked as a word even when inserted after CATS"
        );
    }

    #[test]
    fn gaddag_is_word_method() {
        // Build a gaddag from a small word list and test positives and negatives
        let words = vec!["CAT".to_string(), "CATS".to_string(), "DOG".to_string()];
        let g = Gaddag::from_wordlist(&words);

        // Positive cases
        assert!(g.is_word("CAT"), "CAT should be found by is_word");
        assert!(g.is_word("CATS"), "CATS should be found by is_word");
        assert!(g.is_word("DOG"), "DOG should be found by is_word");

        // Negative cases
        assert!(!g.is_word("DO"), "DO is a prefix but not a word");
        assert!(!g.is_word("ACT"), "ACT is not in the wordlist");
        assert!(!g.is_word(""), "Empty string should not be found");
    }
}
