pub fn binomial_coefficient(n: u64, k: u64) -> Option<u64> {
    if n <= k {
        None
    } else {
        Some(f(n) / (f(k) * f(n - k)))
    }
}

// factorial
fn f(n: u64) -> u64 {
    f_aux(n, 1)
}
fn f_aux(n: u64, res: u64) -> u64 {
    match n {
        1 => res,
        _ => f_aux(n - 1, res * n),
    }
}
