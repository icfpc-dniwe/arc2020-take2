use crate::arc2020::types::*;
use super::types::*;
use super::operation::*;

pub trait OperationsSolver {
    fn solve_operations<'a>(&self, operations: &Operations<'a>, task: &Task) -> Option<Vec<OperationBox<'a>>>;
}

pub struct OpSolver<'a, S: OperationsSolver>(pub Operations<'a>, pub S);

impl<S: OperationsSolver> Solver for OpSolver<'_, S> {
    fn solve(&self, task: &Task) -> TaskSolution {
        let test =
            match self.1.solve_operations(&self.0, task) {
                None => task.test.iter().map(|_test| TaskTestSolution { output: vec!() }).collect(),
                Some(ops) => task.test.iter().map(|test| {
                    let solution = ops.iter().try_fold(test.input.clone(), |img, op| op.op().apply(img.view())).unwrap();
                    TaskTestSolution { output: vec!(solution) }
                }).collect(),
            };
        TaskSolution {
            test: test,
        }
    }
}
