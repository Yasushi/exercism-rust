use std::collections::HashMap;

pub fn word_count(s: &str) -> HashMap<String, u32> {
    let mut r = HashMap::new();
    for w in s.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
    {
        *r.entry(w.to_string()).or_insert(0) += 1;
    }
    r
}
