//! Module M1: Demonstrates struct with both trait and inherent implementations

#[derive(Debug, Clone)]
pub struct M1 {
    pub value: i32,
}

pub trait M1Trait {
    fn method1(&self) -> i32;
}

impl M1Trait for M1 {
    fn method1(&self) -> i32 {
        self.value * 2
    }
}

impl M1 {
    pub fn method2(&self) -> i32 {
        self.value * 3
    }
}

