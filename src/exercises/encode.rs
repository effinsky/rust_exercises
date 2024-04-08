pub fn encode<T: PartialEq + Clone>(lst: &[T]) -> Vec<(usize, T)> {
    encode_aux(0, &mut Vec::new(), lst)
}

fn encode_aux<T: PartialEq + Clone>(
    count: usize,
    acc: &mut Vec<(usize, T)>,
    lst: &[T],
) -> Vec<(usize, T)> {
    match lst {
        [] => acc.to_vec(),
        [a] => {
            acc.push((count + 1, a.clone()));
            acc.to_vec()
        }
        [a, b, t @ ..] if a == b => {
            t.to_vec().push(b.clone());
            encode_aux(count + 1, acc, t)
        }
        [a, _, t @ ..] => {
            acc.push((count + 1, a.clone()));
            encode_aux(0, acc, t)
        }
    }
}
