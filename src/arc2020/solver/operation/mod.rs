pub mod types;
pub mod noop;
pub mod geometry;

use crate::arc2020::types::*;
use types::*;

pub trait LearnableOperationBox {
    fn learn_box<'a>(&'a self, iter: &mut dyn Iterator<Item = (ImageView<'a>, ImageView<'a>)>) -> Option<Box<dyn Operation + 'static>>;
}

impl<L: LearnableOperation> LearnableOperationBox for L where L::Op: 'static {
    fn learn_box<'a>(&'a self, iter: &mut dyn Iterator<Item = (ImageView<'a>, ImageView<'a>)>) -> Option<Box<dyn Operation + 'static>> {
        Some(Box::new(self.learn(iter)?))
    }
}

pub struct Operations<'a> {
    pub simple: Vec<&'a dyn Operation>,
    pub learnable: Vec<&'a dyn LearnableOperationBox>,
}

pub enum OperationBox<'a> {
    Simple(&'a dyn Operation),
    Learnt(Box<dyn Operation>),
}

impl<'a> OperationBox<'a> {
    pub fn op(&'a self) -> &'a dyn Operation {
        match self {
            OperationBox::Simple(op) => *op,
            OperationBox::Learnt(op) => op.as_ref(),
        }
    }
}

pub fn apply_operations<'a, 'b, 'c>(task: &'c Task, images: &'c Vec<CowImage<'b>>, operations: &'c Operations<'a>) -> impl Iterator<Item = (OperationBox<'a>, Vec<CowImage<'b>>)> + 'c {
    let applied_learnt_iter = operations.learnable.iter().filter_map(move |lop| {
        let mut pairs_iter = images.iter().map(|i| i.view()).zip(task.train.iter().map(|example| example.output.view()));
        let op = lop.learn_box(&mut pairs_iter)?;
        let applied = images.iter().map(|img| Some(CowImage::from(op.apply(img.view())?)) ).collect::<Option<Vec<CowImage<'b>>>>()?;
        Some((OperationBox::Learnt(op), applied))
    });
    let applied_iter = operations.simple.iter().filter_map(move |&op| {
        let applied = images.iter().map(|img| Some(CowImage::from(op.apply(img.view())?)) ).collect::<Option<Vec<CowImage<'b>>>>()?;
        Some((OperationBox::Simple(op), applied))
    });
    applied_learnt_iter.chain(applied_iter)
}
