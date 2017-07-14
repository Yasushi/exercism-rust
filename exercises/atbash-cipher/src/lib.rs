use std::collections::HashMap;
use std::ascii::AsciiExt;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn dic() -> HashMap<char, char> {
    let mut r = HashMap::new();
    for (a, b) in ALPHABET.chars().zip(ALPHABET.chars().rev()) {
        r.insert(a, b);
    }
    r

}

pub fn encode(s: &str) -> String {
    let dic = dic();
    s.to_lowercase()
        .chars()
        .filter(|&c| c.is_alphanumeric() && c.is_ascii())
        .map(|c| dic.get(&c).unwrap_or(&c).clone())
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|c| c.iter().cloned().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn decode(s: &str) -> String {
    let dic = dic();
    s.chars()
        .filter(|&c| c != ' ')
        .map(|c| dic.get(&c).unwrap_or(&c).clone())
        .collect()
}
