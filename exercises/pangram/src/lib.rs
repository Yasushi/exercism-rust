use std::collections::HashSet;

pub fn is_pangram(s: &str) -> bool {
    let all: HashSet<_> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let cs: HashSet<_> = s.to_uppercase().chars().collect();
    all.difference(&cs).collect::<Vec<_>>().is_empty()
}
