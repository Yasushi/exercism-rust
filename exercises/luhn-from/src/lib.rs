use std::convert::From;

pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let cs: String = self.0.chars().filter(|&c| c != ' ').collect();
        if cs.chars().any(|c| !c.is_digit(10)) || cs.len() <= 1 {
            return false;
        }
        let sum = cs.chars()
            .filter_map(|c| c.to_digit(10))
            .rev()
            .enumerate()
            .map(|(i, d)| if i % 2 == 1 { d * 2 } else { d })
            .map(|d| if d > 9 { d - 9 } else { d })
            .sum::<u32>();
        sum % 10 == 0
    }
}

impl From<String> for Luhn {
    fn from(s: String) -> Self {
        Luhn(s)
    }
}

impl<'a> From<&'a str> for Luhn {
    fn from(s: &'a str) -> Self {
        Self::from(s.to_string())
    }
}

impl From<u8> for Luhn {
    fn from(from: u8) -> Self {
        Self::from(from.to_string())
    }
}

impl From<u16> for Luhn {
    fn from(from: u16) -> Self {
        Self::from(from.to_string())
    }
}

impl From<u32> for Luhn {
    fn from(from: u32) -> Self {
        Self::from(from.to_string())
    }
}

impl From<u64> for Luhn {
    fn from(from: u64) -> Self {
        Self::from(from.to_string())
    }
}

impl From<usize> for Luhn {
    fn from(from: usize) -> Self {
        Self::from(from.to_string())
    }
}
