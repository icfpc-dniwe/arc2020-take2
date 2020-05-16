use ndarray::{CowArray, Array2, Ix2, ArrayView2};

pub type Image = Array2<u8>;
pub type CowImage<'a> = CowArray<'a, u8, Ix2>;
pub type ImageView<'a> = ArrayView2<'a, u8>;

#[derive(Debug)]
pub struct TaskTrain {
    pub input: Image,
    pub output: Image,
}

#[derive(Debug)]
pub struct TaskTest {
    pub input: Image,
    pub output: Vec<Image>,
}

#[derive(Debug)]
pub struct Task {
    pub train: Vec<TaskTrain>,
    pub test: Vec<TaskTest>,
}

#[derive(Debug)]
pub struct TaskTestSolution {
    pub output: Vec<Image>,
}

#[derive(Debug)]
pub struct TaskSolution {
    pub test: Vec<TaskTestSolution>,
}
