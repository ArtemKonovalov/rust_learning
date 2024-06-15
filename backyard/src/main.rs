use std::{
    collections::HashMap,
    cmp::Ordering,
    io
};
// use std::collections::*;
use crate::garden::vegetables::Asparagus;
use rand::Rng;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}", plant);

    let mut map = HashMap::new();
    map.insert(1, 2);

    let secret_number = rand::thread_rng().gen_range(1..=100);
}
