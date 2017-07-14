use std::ascii::AsciiExt;
use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    input
        .iter()
        .flat_map(|(&k, vs)| {
            vs.iter().map(move |v| (v.to_ascii_lowercase(), k))
        })
        .collect()
}
