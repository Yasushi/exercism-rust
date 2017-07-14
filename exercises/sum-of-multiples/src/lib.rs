pub fn sum_of_multiples(n: u32, ns: &[u32]) -> u32 {
    (1..n).filter(|x| ns.iter().any(|y| x % y == 0)).sum()
}

