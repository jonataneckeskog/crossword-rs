#![allow(dead_code)]

use std::ops::Neg;

use crate::constants::BOARD_SIZE;

pub struct Step(pub isize);

impl Step {
    pub const RIGHT: Step = Step(1);
    pub const LEFT: Step = Step(-1);
    pub const DOWN: Step = Step(BOARD_SIZE as isize);
    pub const UP: Step = Step(-(BOARD_SIZE as isize));

    pub fn apply(&self, index: usize) -> Option<usize> {
        index.checked_add_signed(self.0)
    }
}

impl Neg for Step {
    type Output = Step;

    fn neg(self) -> Step {
        Step(-self.0)
    }
}
