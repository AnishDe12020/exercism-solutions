pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
#[repr(u32)]
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

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn all(&self) -> Vec<Allergen> {
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

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score & (1 << *allergen as u32) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.all()
            .into_iter()
            .filter(|&allergen| self.is_allergic_to(&allergen))
            .collect()
    }
}
