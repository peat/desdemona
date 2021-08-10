use std::fmt::*;

/// A light or dark game disc
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Disc {
    Light,
    Dark,
}

impl Disc {
    /// Returns the opposite color disc
    pub fn opposite(&self) -> Self {
        match self {
            Disc::Light => Disc::Dark,
            Disc::Dark => Disc::Light,
        }
    }
}

impl Display for Disc {
    // TODO: This only works well for people using dark terminal themes
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
