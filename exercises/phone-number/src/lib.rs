pub fn number(s: &str) -> Option<String> {
    let ns: String = s.chars().filter(|c| c.is_digit(10)).collect();
    match ns.len() {
        11 => {
            if let ("1", r) = ns.split_at(1) {
                Some(r.to_string())
            } else {
                None
            }
        }
        10 => Some(ns),
        _ => None,
    }
}

pub fn area_code(s: &str) -> Option<String> {
    number(s).map(|ns| ns[..3].to_string())
}

pub fn pretty_print(s: &str) -> String {
    number(s)
        .map(|ns| format!("({}) {}-{}", &ns[..3], &ns[3..6], &ns[6..]))
        .unwrap_or("invalid".to_string())
}
