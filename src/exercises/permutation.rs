use rand::Rng;

pub fn permutate<A: Clone>(l: &[A]) -> Vec<A> {
    if l.is_empty() {
        return Vec::new();
    }

    let len = l.len();
    let s = random_vec(0, len);

    s.into_iter()
        .fold(Vec::with_capacity(l.len()), |mut acc, idx| {
            acc.push(l[idx].clone());
            acc
        })
}

// Durstenfeld's version of the Fisher-Yates shuffle.
fn random_vec(low: usize, high: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut values = (low..high).collect::<Vec<usize>>();

    for idx in low..high {
        // get random index
        let new_idx = rng.gen_range(idx..high);
        // swap the item under that index with the last element
        let last_untouched_idx = high - idx - 1;
        values.swap(new_idx, last_untouched_idx);
        // and then that index has a value already used and is out of bounds
    }

    values
}
