pub struct Roman;

fn convert1(
    i: usize,
    cur_ch: &str,
    cur_base: usize,
    x: Option<(&str, usize)>,
) -> Option<(String, usize)> {
    if i >= cur_base {
        let d = i / cur_base;
        let remain = i - d * cur_base;
        if let Some((ch, n)) = x {
            if d == n {
                return Some((ch.to_string(), remain));
            }
        }
        Some((cur_ch.repeat(d), remain))
    } else {
        None
    }
}


impl Roman {
    pub fn from(n: u32) -> String {
        let mut r: String = "".to_string();
        let i = n as usize;

        let (i, d) = (i / 10, i % 10);
        match d {
            1 | 2 | 3 => r += "I".repeat(d).as_ref(),
            4 => r += "VI",
            5 | 6 | 7 | 8 => {
                r += "I".repeat(d - 5).as_ref();
                r += "V";
            }
            9 => r += "XI",
            0 => (),
            _ => panic!(),
        }
        if i == 0 {
            return r.chars().rev().collect();
        }

        let (i, d) = (i / 10, i % 10);
        match d {
            1 | 2 | 3 => r += "X".repeat(d).as_ref(),
            4 => r += "LX",
            5 | 6 | 7 | 8 => {
                r += "X".repeat(d - 5).as_ref();
                r += "L";
            }
            9 => r += "CX",
            0 => (),
            _ => panic!("i={}, d={}", i, d),
        }
        if i == 0 {
            return r.chars().rev().collect();
        }

        let (i, d) = (i / 10, i % 10);
        match d {
            1 | 2 | 3 => r += "C".repeat(d).as_ref(),
            4 => r += "DC",
            5 | 6 | 7 | 8 => {
                r += "C".repeat(d - 5).as_ref();
                r += "D";
            }
            9 => r += "MC",
            0 => (),
            _ => panic!("i={}, d={}", i, d),
        }

        if i > 0 {
            r += "M".repeat(i).as_ref();
        }
        r.chars().rev().collect()
    }

    pub fn from_(n: u32) -> String {
        let mut r: String = "".to_string();
        let mut i = n as usize;
        while i > 0 {
            if i >= 1000 {
                r += "M".repeat(i / 1000).as_ref();
                i -= (i / 1000) * 1000;
            } else if i >= 500 {
                r += "D".repeat(i / 500).as_ref();
                i -= (i / 500) * 500;
            } else if i >= 100 {
                if let Some((rs, n)) = convert1(i, "C", 100, Some(("CD", 4))) {
                    r += rs.as_ref();
                    i = n;
                } else {
                    panic!();
                }
            } else if i >= 50 {
                if let Some((rs, n)) = convert1(i, "L", 50, Some(("LC", 9))) {
                    r += rs.as_ref();
                    i = n;
                } else {
                    panic!();
                }
            } else if i >= 10 {
                if let Some((rs, n)) = convert1(i, "X", 10, Some(("XL", 4))) {
                    r += rs.as_ref();
                    i = n;
                } else {
                    panic!();
                }
            } else if i >= 5 {
                if let Some((rs, n)) = convert1(i, "V", 5, Some(("IX", 9))) {
                    println!("{} {} {} ", i, rs, n);
                    r += rs.as_ref();
                    i = n;
                } else {
                    panic!();
                }
            } else {
                if let Some((rs, n)) = convert1(i, "I", 1, Some(("IV", 4))) {
                    r += rs.as_ref();
                    i = n;
                } else {
                    panic!();
                }
            }
        }

        r
    }
}
