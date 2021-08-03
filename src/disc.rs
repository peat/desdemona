use std::fmt::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Disc {
    Light,
    Dark,
}

impl Disc {
    pub fn opposite(&self) -> Self {
        match self {
            Disc::Light => Disc::Dark,
            Disc::Dark => Disc::Light,
        }
    }
}

impl Display for Disc {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let representation = match self {
            Disc::Dark => "○",
            Disc::Light => "●",
        };

        write!(f, "{}", representation)
    }
}

impl Default for Disc {
    fn default() -> Self {
        Disc::Dark
    }
}
