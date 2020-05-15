pub mod types;
pub mod score;
pub mod operation;
pub mod operations_solver;
pub mod stub;
pub mod greedy;

use std::collections::HashMap;
use types::Solver;
use crate::arc2020::types::*;

pub fn solve<S: Solver>(all_tasks: &HashMap<String, Task>, solver: &S) -> MyResult<HashMap<String, TaskSolution>> {
    let ret = all_tasks.iter().map(|(name, task)| {
        let solution = solver.solve(task);
        if solution.test.len() != task.test.len() {
            Err(format_err!("Solution array size is not equal to test array size"))
        } else {
            Ok((name.clone(), solution))
        }
    }).collect::<MyResult<HashMap<String, TaskSolution>>>()?;
    Ok(ret)
}
