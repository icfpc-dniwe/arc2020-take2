use crate::arc2020::types::*;

pub trait Metric {
    fn score(&self, a: ImageView, b: ImageView) -> u32;
}

pub struct StubMetric(());

impl StubMetric {
    pub fn new() -> StubMetric {
        StubMetric(())
    }
}

impl Metric for StubMetric {
    fn score(&self, a: ImageView, b: ImageView) -> u32 {
        if a == b { 0 } else { 1 }
    }
}
