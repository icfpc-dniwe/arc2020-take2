use super::types::*;
use crate::arc2020::types::*;

#[derive(Debug)]
pub struct Transpose(());

impl Transpose {
    pub fn new() -> Transpose {
        Transpose(())
    }
}

impl Operation for Transpose {
    fn apply(&self, img: ImageView) -> Option<Image> {
        Some(img.to_owned().reversed_axes())
    }
}
