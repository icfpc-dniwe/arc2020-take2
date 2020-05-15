use ndarray::CowArray;
use std::iter::FromIterator;
use crate::arc2020::types::*;
use super::operations_solver::*;
use super::operation::*;
use super::score::Metric;

pub struct GreedySolver<M: Metric> {
    max_depth: u32,
    metric: M,
}

impl<M: Metric> GreedySolver<M> {
    pub fn new(max_depth: u32, metric: M) -> GreedySolver<M> {
        GreedySolver {
            max_depth: max_depth,
            metric: metric,
        }
    }

    pub fn max_depth(&self) -> u32 {
        self.max_depth
    }

    pub fn metric(&self) -> &M {
        &self.metric
    }
}

fn solve_step<'a, 'b, M: Metric>(solver: &'b GreedySolver<M>, operations: &'b Operations<'a>, task: &'b Task, depth: u32, current: Vec<CowImage<'b>>) -> Option<Vec<OperationBox<'a>>> {
    if depth >= solver.max_depth() {
        return None;
    }

    let results = apply_operations(task, &current, operations);
    let (next_op, next_result, next_score) =
        results.fold(None, |best: Option<(OperationBox<'a>, Vec<CowImage<'b>>, u32)>, (op, result)| {
            let curr_score: u32 = result.iter().zip(task.train.iter()).map(|(x, train)| solver.metric().score(x.view(), train.output.view())).sum();
            match &best {
                Some((_old_op, _old_result, old_score)) if old_score < &curr_score => best,
                _ => Some((op, result, curr_score)),
            }
        })?;
    if next_score == 0 {
        Some(vec!(next_op))
    } else {
        let mut next = solve_step(solver, operations, task, depth + 1, next_result)?;
        next.insert(0, next_op);
        Some(next)
    }
}

impl<M: Metric> OperationsSolver for GreedySolver<M> {
    fn solve_operations<'a>(&self, operations: &Operations<'a>, task: &Task) -> Option<Vec<OperationBox<'a>>> {
        let input = Vec::from_iter(task.train.iter().map(|x| CowArray::from(x.input.view())));
        solve_step(self, operations, task, 0, input)
    }
}
