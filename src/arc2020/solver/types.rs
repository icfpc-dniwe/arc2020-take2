use crate::arc2020::types::*;

pub trait Solver {
    fn solve(&self, task: &Task) -> TaskSolution;
}
