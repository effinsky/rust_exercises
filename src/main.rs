#![allow(unused)]
mod exercises;

use exercises::encode::encode;
use exercises::insert_at::insert_at;
use exercises::permutation::permutate;
use exercises::range::create_range;
use std::fmt::Debug;
use std::slice;

// DESIGN BY SUBTRACTION
fn main() {
    let orig = ["a", "b", "c", "d", "d", "d", "e", "e", "f", "f", "f", "f"];
    let run_len_encoded = encode(&orig);
    println!("{:?}", &run_len_encoded);

    match orig.get(10) {
        Some(v) => println!("we have a value: {}", v),
        None => println!("no value"),
    };

    let to_insert_into = [0, 1, 2, 3, 4, 5];
    let inserted_into = insert_at(666, 3, &to_insert_into);
    dbg!(&inserted_into);

    let rng = create_range(2, 10);
    dbg!(rng);

    let src_for_permutation = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("before permutation: {:?}", &src_for_permutation);
    let permutated = permutate(&src_for_permutation);
    println!("after permutation: {:?}", &permutated);
}

pub fn sum(l: &[i32]) -> i32 {
    // we could do this recursively and avoid mutation altogether but why would
    // we.
    fn sum_rec(mut l_iter: slice::Iter<i32>, res: i32) -> i32 {
        match l_iter.next() {
            None => res,
            Some(val) => sum_rec(l_iter, res + val),
        }
    }

    sum_rec(l.iter(), 0)
}

fn get_ordinal(n: i32) -> String {
    match n {
        11..=13 => "th",
        n if n % 10 == 1 => "st",
        n if n % 10 == 2 => "nd",
        n if n % 10 == 3 => "rd",
        _ => "th",
    }
    .to_string()
}

fn multiply_list<A>(base: &[A], times: u32) -> Vec<A>
where
    A: Clone + Debug,
{
    let mut out = vec![];
    for _ in 0..times {
        dbg!(&out);
        out.extend_from_slice(base);
    }
    out
}

#[derive(Debug)]
enum Node<'a, A>
where
    A: Debug,
{
    One(A),
    Many(&'a [A]),
}

fn flatten_uber<T>(inp: &[Node<T>]) -> Vec<T>
where
    T: Clone + Debug,
{
    fn flatten_inner<A: Clone + Debug>(
        inp: &[Node<A>],
        mut res: Vec<A>,
    ) -> Vec<A> {
        match inp[..] {
            [] => res,
            [ref h, ref t @ ..] => match h {
                Node::One(v) => {
                    res.push(v.clone());
                    res
                }
                Node::Many(_) => flatten_inner(t, res),
            },
        }
    }

    let result_holder: Vec<T> = Vec::new();
    flatten_inner(inp, result_holder)
}

fn compress<A: Debug + PartialEq + Clone>(l: &[A]) -> Vec<A> {
    match l {
        [a, b, t @ ..] => {
            if a == b {
                compress(t)
            } else {
                let mut new_list = vec![a.clone()];
                new_list.extend_from_slice(t);
                compress(&new_list)
            }
        }
        lst => lst.to_vec(),
    }
}
