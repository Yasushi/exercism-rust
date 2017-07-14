pub fn reply(s: &str) -> &'static str {
    if s.ends_with("?") {
        "Sure."
    } else if s.is_empty() {
        "Fine. Be that way!"
    } else if s == s.to_uppercase() {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
