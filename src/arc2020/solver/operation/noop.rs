use super::types::*;
use crate::arc2020::types::*;

#[derive(Debug)]
pub struct Noop(());

impl Noop {
    pub fn new() -> Noop {
        Noop(())
    }
}

impl Operation for Noop {
    fn apply(&self, img: ImageView) -> Option<Image> {
        Some(img.to_owned())
    }
}

pub struct LearnableNoop(());

impl LearnableNoop {
    pub fn new() -> LearnableNoop {
        LearnableNoop(())
    }
}

impl LearnableOperation for LearnableNoop {
    type Op = Noop;
   
    fn learn<'a, 'b, I: Iterator<Item = (ImageView<'a>, ImageView<'a>)>>(&'b self, _imgs: I) -> Option<Self::Op> {
        Some(Noop::new())
    }
}
