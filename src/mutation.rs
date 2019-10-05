use super::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mutation {
    Red,
    Green,
    Blue,
}

impl Distribution<Mutation> for distributions::Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Mutation {
        match rng.gen_range(0, 3) {
            0 => Mutation::Red,
            1 => Mutation::Green,
            2 => Mutation::Blue,
            _ => unreachable!(),
        }
    }
}

impl Mutation {
    pub fn color(&self) -> Color<f32> {
        match self {
            Self::Red => Color::RED,
            Self::Green => Color::GREEN,
            Self::Blue => Color::BLUE,
        }
    }
}
