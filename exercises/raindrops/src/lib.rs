fn pling(i: u32) -> Option<String> {
    if i % 3 == 0 {
        Some("Pling".to_string())
    } else {
        None
    }
}

fn plang(i: u32) -> Option<String> {
    if i % 5 == 0 {
        Some("Plang".to_string())
    } else {
        None
    }
}

fn plong(i: u32) -> Option<String> {
    if i % 7 == 0 {
        Some("Plong".to_string())
    } else {
        None
    }
}

pub fn raindrops(i: u32) -> String {
    let ppp =  [pling(i), plang(i), plong(i)].iter()
        .flat_map(|x| x)
        .cloned()
        .collect::<Vec<_>>()
        .join("");

    if ppp.len() > 0 {
        ppp
    } else {
        i.to_string()
    }
}
