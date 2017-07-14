
fn check(left: &Vec<char>, right: &Vec<char>) -> bool {
    let mut l = left.to_vec();
    let mut r = right.to_vec();
    l.sort();
    r.sort();
    left != right && l == r
}

pub fn anagrams_for<'a>(word: &str, candidates: &[&'a str]) -> Vec<&'a str> {
    let ls: Vec<char> = word.to_lowercase().chars().collect();

    println!("\nWORD {:?}", ls);
    for b in candidates.iter().map(|w| {
        w.to_lowercase().chars().collect::<Vec<char>>()
    })
    {
        println!("CAND {:?} {:?}", b, check(&ls, &b));
    }

    candidates
        .iter()
        .filter(|w| check(&ls, &w.to_lowercase().chars().collect()))
        .cloned()
        .collect()
}
