use std::str::FromStr;

#[derive(Debug)]
enum Op {
    Plus,
    Minus,
    Multiply,
    Divide,
}

pub struct WordProblem {
    problem: String,
}

impl WordProblem {
    pub fn new(p: &str) -> Self {
        Self { problem: p.to_string() }
    }

    pub fn answer(&self) -> Result<i32, ()> {
        let mut acc = 0;
        let mut op: Option<Op> = None;
        for w in self.problem[..self.problem.len()-1].split_whitespace() {
            match w {
                "plus" => op = Some(Op::Plus),
                "minus" => op = Some(Op::Minus),
                "multiplied" => op = Some(Op::Multiply),
                "divided" => op = Some(Op::Divide),
                "What" | "is" | "by" => (),
                n => {
                    if let Ok(num) = i32::from_str(n) {
                        match op {
                            Some(o) => {
                                match o {
                                    Op::Plus => acc += num,
                                    Op::Minus => acc -= num,
                                    Op::Multiply => acc *= num,
                                    Op::Divide => acc /= num,
                                }
                                op = None
                            }
                            None => acc = num,
                        }
                    } else {
                        return Err(());
                    }
                }
            }
        }
        Ok(acc)
    }
}
