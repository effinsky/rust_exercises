use std::cmp::Ordering;

pub fn search<T>(items: &[T], it: T) -> Option<usize>
where
  T: Ord,
{
  let mid_idx = items.len() / 2;
  let mid = &items[mid_idx];

  match it.cmp(mid) {
    Ordering::Equal => Some(mid_idx),
    Ordering::Less if mid_idx > 0 => search(&items[..mid_idx], it),
    Ordering::Greater if mid_idx < items.len() => {
      search(&items[mid_idx + 1..], it)
    }
    _ => None,
  }
}
