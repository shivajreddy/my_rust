#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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
    pub fn value(&self) -> u32 {
        match self {
            Allergen::Eggs => 1,
            Allergen::Peanuts => 2,
            Allergen::Shellfish => 4,
            Allergen::Strawberries => 8,
            Allergen::Tomatoes => 16,
            Allergen::Chocolate => 32,
            Allergen::Pollen => 64,
            Allergen::Cats => 128,
        }
    }
}

const ALL_ALLERGENS: &[Allergen] = &[
    Allergen::Eggs,
    Allergen::Peanuts,
    Allergen::Shellfish,
    Allergen::Strawberries,
    Allergen::Tomatoes,
    Allergen::Chocolate,
    Allergen::Pollen,
    Allergen::Cats,
];

pub struct Allergies {
    score: u32,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        // make sure the score is in the range 1, 255 (inclusive)
        let mut given_score = score;
        while given_score >= 256 {
            given_score -= 256
        }
        Allergies { score: given_score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies().contains(&allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut result: Vec<Allergen> = vec![];
        let mut total_score = self.score;

        for allergen in ALL_ALLERGENS.iter().rev() {
            if total_score >= allergen.value() {
                total_score -= allergen.value();
                result.push(*allergen);
            }
        }

        result
    }
}
