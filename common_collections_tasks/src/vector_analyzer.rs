// Given a list of integers, use a vector and return the median
// (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;
use std::ops::Div;

pub fn median(vec: &Vec<i32>) -> i32 {
    let mut v = Vec::new();
    for n in vec {
        v.push(n);
    }
    v.sort();
    let index = v.len().div(2);

    **v.get(index).unwrap()
}

pub fn mode(vec: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for v in vec {
        let cnt = map.entry(*v).or_insert(0);
        *cnt += 1;
    }
    let mut max_val = 0;
    let mut max_key = 0;
    for x in map.iter() {
        if x.1 > &max_val {
            max_key = *x.0;
            max_val = *x.1;
        }
    }
    println!("{:?}", &map);
    max_key
}