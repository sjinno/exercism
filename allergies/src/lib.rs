use Allergen::*;

const ALLEGEN: [Allergen; 8] = [
    Eggs,         //   1
    Peanuts,      //   2
    Shellfish,    //   4
    Strawberries, //   8
    Tomatoes,     //  16
    Chocolate,    //  32
    Pollen,       //  64
    Cats,         // 128
];
pub struct Allergies {
    pub score: u32,
}

#[derive(Debug, PartialEq, Clone)]
#[rustfmt::skip]
pub enum Allergen {
    Eggs         =   1,
    Peanuts      =   2,
    Shellfish    =   4,
    Strawberries =   8,
    Tomatoes     =  16,
    Chocolate    =  32,
    Pollen       =  64,
    Cats         = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score }
    }

    #[rustfmt::skip]
    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        match allergen {
            Allergen::Eggs         => self.score == 1 || self.score % 2 == 1,
            Allergen::Peanuts      => self.score /   2 == 1,
            Allergen::Shellfish    => self.score /   4 == 1,
            Allergen::Strawberries => self.score /   8 == 1,
            Allergen::Tomatoes     => self.score /  16 == 1,
            Allergen::Chocolate    => self.score /  32 == 1,
            Allergen::Pollen       => self.score /  64 == 1,
            Allergen::Cats         => self.score / 128 == 1,
        }
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut score = self.score;

        // If score is over 255, divide the number by 128,
        // and subtract (the_quotient - 1) * 128 from the score.
        if score > 255 {
            let quo = score / 128;
            score -= (quo - 1) * 128;
        }

        if score == 0 {
            return Vec::new();
        } else if score == 255 {
            return ALLEGEN.into();
        }

        let mut allegen = Vec::new();
        for a in ALLEGEN.iter().rev() {
            let num = a.clone() as u32;
            let quo = score / num;
            if quo == 1 {
                allegen.push(a.clone());
                score -= quo * num;
            }
        }

        allegen
    }
}
