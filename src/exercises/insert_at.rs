pub fn insert_at<A>(it: A, at: usize, l: &[A]) -> Vec<A>
where
  A: Clone,
{
  insert_at_aux(l, it, at, Vec::new(), 0)
}

fn insert_at_aux<A>(
  l: &[A],
  it: A, // moving it consistently thru recursive layers
  at: usize,
  mut acc: Vec<A>,
  idx: usize,
) -> Vec<A>
where
  A: Clone,
{
  match l {
    [] => acc,
    [ref h, t @ ..] if idx == at => {
      acc.push(it.clone());
      acc.push(h.clone());
      insert_at_aux(t, it, at, acc, idx + 1)
    }
    [ref h, t @ ..] => {
      acc.push(h.clone());
      insert_at_aux(t, it, at, acc, idx + 1)
    }
  }
}
