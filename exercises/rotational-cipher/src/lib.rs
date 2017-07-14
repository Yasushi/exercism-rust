use std::collections::HashMap;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn dic(shift: usize) -> HashMap<char, char> {
    let mut r = HashMap::new();
    let src = ALPHABET;
    let dst = ALPHABET
        .repeat(2)
        .chars()
        .skip(shift)
        .take(26)
        .collect::<String>();
    for (a, b) in src.chars().zip(dst.chars()) {
        r.insert(a, b);
    }
    let src_upper = src.to_uppercase();
    let dst_upper = dst.to_uppercase();
    for (a, b) in src_upper.chars().zip(dst_upper.chars()) {
        r.insert(a, b);
    }
    r
}

pub fn rotate(s: &str, shift: usize) -> String {
    let dic = dic(shift);
    s.chars()
        .map(|c| dic.get(&c).unwrap_or(&c).clone())
        .collect()
}
