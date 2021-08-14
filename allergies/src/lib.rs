use crate::Allergen::{Cats, Chocolate, Eggs, Peanuts, Pollen, Shellfish, Strawberries, Tomatoes};

pub struct Allergies(u32);

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergen {
    pub fn all() -> Vec<Allergen> {
        vec![
            Eggs,
            Peanuts,
            Shellfish,
            Strawberries,
            Tomatoes,
            Chocolate,
            Pollen,
            Cats,
        ]
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0 & *allergen as u32 != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::all()
            .iter()
            .filter(|x| self.is_allergic_to(x))
            .map(|x| *x)
            .collect::<Vec<Allergen>>()
    }
}
