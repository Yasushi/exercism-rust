use std::collections::HashMap;

pub struct Brackets(String);

impl Brackets {
    pub fn from(s: &str) -> Self {
        Brackets(s.to_string())
    }

    pub fn are_balanced(&self) -> bool {
        let mut parens: HashMap<char, char> = HashMap::new();
        parens.insert(')', '(');
        parens.insert('}', '{');
        parens.insert(']', '[');

        let mut stack: Vec<char> = Vec::new();

        for c in self.0.chars() {
            if let Some(&open) = parens.get(&c) {
                match stack.last() {
                    Some(&o) if o == open => {
                        stack.pop().unwrap();
                        ()
                    }
                    Some(_) => return false,
                    None => return false,
                }
            } else if parens.values().find(|&&p| p == c).is_some() {
                stack.push(c);
            }
        }

        stack.is_empty()
    }
}
