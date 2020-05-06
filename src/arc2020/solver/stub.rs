use super::types::*;
use crate::arc2020::types::*;

pub struct StubSolver(());

impl StubSolver {
    pub fn new() -> StubSolver {
        StubSolver(())
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
