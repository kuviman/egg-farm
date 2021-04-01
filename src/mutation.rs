use super::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Mutation {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Purple,
    RGB,
}

impl Distribution<Mutation> for distributions::Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Mutation {
        match rng.gen_range(0..3) {
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
            Self::Cyan => Color::rgb(0.0, 1.0, 1.0),
            Self::Yellow => Color::rgb(1.0, 1.0, 0.0),
            Self::Purple => Color::rgb(1.0, 0.0, 1.0),
            Self::RGB => global_rng().gen::<Self>().color(),
        }
    }
    pub fn mix(self, other: Option<Self>) -> Option<Self> {
        if self == Self::RGB || other == Some(Self::RGB) {
            return Some(Self::RGB);
        }
        let mut color = self.color();
        if let Some(other) = other {
            let other = other.color();
            color.r = color.r.max(other.r);
            color.g = color.g.max(other.g);
            color.b = color.b.max(other.b);
        }
        match (color.r > 0.5, color.g > 0.5, color.b > 0.5) {
            (false, false, false) => None,
            (true, true, true) => Some(Self::RGB),
            (true, false, false) => Some(Self::Red),
            (false, true, false) => Some(Self::Green),
            (false, false, true) => Some(Self::Blue),
            (true, true, false) => Some(Self::Yellow),
            (true, false, true) => Some(Self::Purple),
            (false, true, true) => Some(Self::Cyan),
        }
    }
}
