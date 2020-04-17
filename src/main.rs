#[macro_use] extern crate failure;
extern crate serde;
pub mod arc2020;

use std::path::Path;
use self::arc2020::io;
use self::arc2020::types::MyResult;

fn main() -> MyResult<()> {
    let data_path = Path::new("data/training");
    let training_tasks = io::read_all_tasks(data_path)?;
    println!("{:?}", training_tasks);
    Ok(())
}
