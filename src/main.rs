#[macro_use] extern crate anyhow;
extern crate serde;
pub mod arc2020;

use anyhow::Result;
use std::iter::FromIterator;
use std::io::{stdout, BufWriter};
use std::path::Path;
use self::arc2020::io;
use self::arc2020::solver::{self, operations_solver::OpSolver, score::StubMetric, greedy::GreedySolver};
use self::arc2020::solver::operation::{*, types::*, geometry::*, noop::*};

fn main() -> Result<()> {
    let simple_ops: Vec<Box<dyn Operation>> = vec!(Box::new(Transpose::new()), Box::new(Noop::new()));
    let learnable_ops: Vec<Box<dyn LearnableOperationBox>> = vec!(Box::new(LearnableNoop::new()));
    let ops = Operations {
        simple: Vec::from_iter(simple_ops.iter().map(|x| x.as_ref())),
        learnable: Vec::from_iter(learnable_ops.iter().map(|x| x.as_ref())),
    };

    let data_path = Path::new("data/training");
    let training_tasks = io::read_all_tasks(data_path)?;
    let solver = OpSolver(ops, GreedySolver::new(2, StubMetric::new()));
    let solutions = solver::solve(&training_tasks, &solver)?;
    let mut stream = BufWriter::new(stdout());
    io::serialize_all_tasks(&mut stream, &solutions)?;
    Ok(())
}
