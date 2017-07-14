#[derive(PartialEq, Eq, Debug)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    if a == b {
        Comparison::Equal
    } else if contains(a, b) {
        Comparison::Sublist
    } else if contains(b, a) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

fn contains<T: PartialEq>(pat: &[T], target: &[T]) -> bool {
    if pat.is_empty() {
        return true;
    }
    if pat.len() > target.len() {
        return false;
    }
    for i in 0..(target.len() - pat.len() + 1) {
        if target[i..].starts_with(pat) {
            return true;
        }
    }
    false
}
