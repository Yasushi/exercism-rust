
pub fn hamming_distance(a: &str, b: &str) -> Result<usize, ()> {
    if a.len() != b.len() {
        Err(())
    } else {
        Ok(a.chars().zip(b.chars()).filter(|&(x, y)| x != y).count())
    }
}
