pub fn square_of_sum(n: u32) -> u32 {
    let a: u32 = (1..n + 1).sum();
    a * a
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..n + 1).map(|i| i * i).sum()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
