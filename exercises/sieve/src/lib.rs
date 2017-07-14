pub fn primes_up_to(n: u32) -> Vec<u32> {
    let mut is: Vec<u32> = (2..n + 1).collect();
    let mut p = Some(2);

    while let Some(y) = p {
        is.retain(|&i| i == y || (i % y) != 0);
        p = is.clone().into_iter().find(|&i| i > y);
    }

    is
}
