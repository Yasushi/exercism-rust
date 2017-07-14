extern crate rand;

use rand::Rng;

pub struct Robot {
    name: String,
}

fn generate() -> String {
    let mut rng = rand::thread_rng();
    let prefix = rng.gen_ascii_chars()
        .filter(|c| c.is_alphabetic())
        .take(2)
        .collect::<String>()
        .to_uppercase();
    let num = format!("{:03}", rng.gen_range(0, 1000));
    format!("{}{}", prefix, num)
}

impl Robot {
    pub fn new() -> Self {
        Self { name: generate() }
    }

    pub fn name<'a>(&'a self) -> &'a str {
        &self.name[..]
    }

    pub fn reset_name(&mut self) {
        self.name = generate();
    }
}
