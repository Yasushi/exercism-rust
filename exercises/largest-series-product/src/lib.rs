pub fn lsp(s: &str, n: usize) -> Result<u32, ()> {
    if n == 0 {
        return Ok(1);
    }
    if s.chars().any(|c| !c.is_digit(10)) {
        return Err(());
    }
    let ws = s.chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>()
        .windows(n)
        .map(|ds| ds.to_vec().iter().product())
        .collect::<Vec<_>>();
    // println!("{:?}", ws);
    // println!("{:?}", ws.iter().max());

    ws.into_iter().max().ok_or(())
}
