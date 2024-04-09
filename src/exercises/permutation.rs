use rand::Rng;

pub fn permutate<A: Clone>(l: &[A]) -> Vec<A> {
  match l {
    [] => [].to_vec(),
    l => {
      let len = l.len();
      let s = shuffle(0, len);

      s.into_iter()
        .fold(Vec::with_capacity(l.len()), |mut acc, idx| {
          acc.push(l[idx].clone());
          acc
        })
    }
  }
}

// Durstenfeld's version of the Fisher-Yates shuffle.
fn shuffle(low: usize, high: usize) -> Vec<usize> {
  let mut rng = rand::thread_rng();
  let mut out = (low..high).collect::<Vec<usize>>();

  for idx in low..high {
    // get random index
    let new_idx = rng.gen_range(idx..high);
    // swap the item under that index with the last element
    let last_untouched_idx = high - idx - 1;
    out.swap(new_idx, last_untouched_idx);
    // and then that index has a value already used and is out of bounds
  }

  out
}
