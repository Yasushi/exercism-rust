use std::collections::{HashMap, HashSet};

pub fn nucleotide_counts(s: &str) -> Result<HashMap<char, usize>, ()> {
    let mut r: HashMap<char, usize> = [('A', 0), ('T', 0), ('C', 0), ('G', 0)]
        .iter()
        .cloned()
        .collect();
    for c in s.chars() {
        if let Some(count) = r.get_mut(&c) {
            *count += 1;
        } else {
            return Err(());
        }
    }
    Ok(r)
}

pub fn count(c: char, s: &str) -> Result<usize, ()> {
    if s.is_empty() {
        return Ok(0);
    }
    let symbols: HashSet<char> = "ATCG".chars().collect();
    if !symbols.contains(&c) ||
        !s.chars()
            .collect::<HashSet<_>>()
            .difference(&symbols)
            .collect::<Vec<_>>()
            .is_empty()
    {
        return Err(());
    }
    Ok(s.chars().filter(|&cs| c == cs).count())
}
