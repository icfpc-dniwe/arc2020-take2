use std::fmt::Debug;
use crate::arc2020::types::*;

pub trait NamedOperation {
    fn name(&self) -> String;
}

impl<Op: Debug> NamedOperation for Op {
    fn name(&self) -> String {
        format!("{:?}", self)
    }
}

pub trait Operation: NamedOperation {
    fn apply<'a>(&'a self, img: ImageView<'a>) -> Option<Image>;
}

pub trait LearnableOperation {
    type Op: Operation;

    fn learn<'a, I: Iterator<Item = (ImageView<'a>, ImageView<'a>)>>(&'a self, imgs: I) -> Option<Self::Op>;
}
