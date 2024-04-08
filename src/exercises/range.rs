// use num::Integer;
// use std::ops::Add;
// use std::ops::Sub;

pub fn create_range(start: i32, end: i32) -> Vec<i32> {
    aux(start, end, Vec::new())
}

fn aux(curr: i32, lim: i32, mut acc: Vec<i32>) -> Vec<i32> {
    match curr {
        c if c == lim => acc,
        c => {
            acc.push(c);
            aux(c + 1, lim, acc)
        }
    }
}
