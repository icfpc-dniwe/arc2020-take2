use super::types::*;
use crate::arc2020::types::*;

pub struct GreedySolver {
    max_depth: u32,
}

impl GreedySolver {
    pub fn new(max_depth: u32) -> GreedySolver {
        GreedySolver {
            max_depth: max_depth,
        }
    }
}

impl Solver for StubSolver {
    fn solve(&self, task: &Task) -> TaskSolution {
        let solutions = task.test.iter().map(|test| {
            if test.output.len() > 0 {
                TaskTestSolution {
                    output: test.output.clone(),
                }
            } else {
                TaskTestSolution {
                    output: vec!(test.input.clone()),
                }
            }
        }).collect::<Vec<TaskTestSolution>>();
        TaskSolution {
            test: solutions,
        }
    }
}
