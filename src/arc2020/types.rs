use ndarray::Array2;
use failure;

pub type MyResult<A> = Result<A, failure::Error>;

pub type Image = Array2<u8>;

#[derive(Debug)]
pub struct TaskTrain {
    pub input: Image,
    pub output: Image,
}

#[derive(Debug)]
pub struct TaskTest {
    pub input: Image,
    pub output: Option<Image>,
}

#[derive(Debug)]
pub struct Task {
    pub train: Vec<TaskTrain>,
    pub test: Vec<TaskTest>,
}
