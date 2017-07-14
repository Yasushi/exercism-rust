pub fn encode(s: &str) -> String {
    let mut cs = s.chars();
    let mut ch: char = char::default();
    let mut run: usize = 0;
    let mut result = String::new();
    loop {
        match cs.next() {
            Some(c) if ch == c => run += 1,
            Some(c) => {
                if run == 1 {
                    result.push_str(ch.to_string().as_str());
                } else if run > 0 {
                    result.push_str(format!("{}{}", run, ch).as_str());
                }
                run = 1;
                ch = c;
            }
            None => {
                if run == 1 {
                    result.push(ch);
                } else if run > 1 {
                    result += format!("{}{}", run, ch).as_str();
                }
                break;
            }
        }
    }

    result
}

pub fn decode(s: &str) -> String {
    let mut run: usize = 0;
    let mut cs = s.chars();
    let mut result = String::new();
    loop {
        match cs.next() {
            Some(c) if c.is_digit(10) => run = run * 10 + c.to_string().parse::<usize>().unwrap(),
            Some(c) => {
                if run == 0 {
                    run = 1
                }
                result += c.to_string().repeat(run).as_str();
                // result += std::iter::repeat(c).take(run).collect::<String>().as_str();
                run = 0;
            }
            None => break,
        }
    }
    result
}
