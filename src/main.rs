#[macro_use] extern crate failure;
extern crate serde;
pub mod arc2020;

use std::io::{stdout, BufWriter};
use std::path::Path;
use self::arc2020::io;
use self::arc2020::types::MyResult;
use self::arc2020::solver::{self, stub::StubSolver};

fn main() -> MyResult<()> {
    let data_path = Path::new("data/training");
    let training_tasks = io::read_all_tasks(data_path)?;
    let solver = StubSolver::new();
    let solutions = solver::solve(&training_tasks, &solver)?;
    let mut stream = BufWriter::new(stdout());
    io::serialize_all_tasks(&mut stream, &solutions)?;
    Ok(())
}
