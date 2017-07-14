// use std::collections::HashSet;
pub fn is_valid(s: &str) -> bool {
    // let digits: HashSet<_> = "0123456789".chars().collect();
    let cs: String = s.chars().filter(|&c| c != ' ').collect();
    if cs.chars().any(|c| !c.is_digit(10)) || cs.len() <= 1 {
        return false;
    }
    // println!("\nCS {}", cs);
    // let xx = cs.chars()
    //     .rev()
    //     .map(|c| c.to_string().parse::<u32>().unwrap())
    //     .enumerate()
    //     .collect::<Vec<_>>();
    // println!("{:?}", xx);
    // let yy = xx.iter().map(|&(i, d)| if i % 2 == 1 {
    //     let dd = d * 2;
    //     if dd > 9 { (i, dd - 9) } else { (i, dd) }
    // } else {
    //     (i, d)
    // }).collect::<Vec<_>>();
    // println!("{:?}", yy);

    let sum = cs.chars()
        .filter_map(|c| c.to_digit(10))
        .rev()
        .enumerate()
        .map(|(i, d)| if i % 2 == 1 { d * 2 } else { d })
        .map(|d| if d > 9 { d - 9 } else { d })
        .sum::<u32>();
    // println!("sum {}", sum);
    sum % 10 == 0
}
