
pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<'a> Luhn for &'a str {
    fn valid_luhn(&self) -> bool {
        String::from(*self).valid_luhn()
    }
}

impl Luhn for String {
    fn valid_luhn(&self) -> bool {
        let cs: String = self.chars().filter(|&c| c != ' ').collect();
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

impl Luhn for u8 {
    fn valid_luhn(&self) -> bool {
        self.to_string().valid_luhn()
    }
}

impl Luhn for u16 {
    fn valid_luhn(&self) -> bool {
        self.to_string().valid_luhn()
    }
}

impl Luhn for u32 {
    fn valid_luhn(&self) -> bool {
        self.to_string().valid_luhn()
    }
}

impl Luhn for u64 {
    fn valid_luhn(&self) -> bool {
        self.to_string().valid_luhn()
    }
}

impl Luhn for usize {
    fn valid_luhn(&self) -> bool {
        self.to_string().valid_luhn()
    }
}
