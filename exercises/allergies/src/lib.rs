#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergen {
    pub fn values() -> Vec<Allergen> {
        vec![
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ]
    }
}


pub struct Allergies {
    score: u32,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score: score }
    }

    pub fn is_allergic_to(&self, a: &Allergen) -> bool {
        (self.score & 1 << *a as u32) > 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::values()
            .into_iter()
            .filter(|a| self.is_allergic_to(a))
            .collect()
    }
}
