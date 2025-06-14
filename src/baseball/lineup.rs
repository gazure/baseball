use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum BattingPosition {
    #[default]
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
    Ninth,
}

impl Display for BattingPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BattingPosition::First => write!(f, "First"),
            BattingPosition::Second => write!(f, "Second"),
            BattingPosition::Third => write!(f, "Third"),
            BattingPosition::Fourth => write!(f, "Fourth"),
            BattingPosition::Fifth => write!(f, "Fifth"),
            BattingPosition::Sixth => write!(f, "Sixth"),
            BattingPosition::Seventh => write!(f, "Seventh"),
            BattingPosition::Eighth => write!(f, "Eighth"),
            BattingPosition::Ninth => write!(f, "Ninth"),
        }
    }
}

impl BattingPosition {
    pub fn next(self) -> BattingPosition {
        match self {
            BattingPosition::First => BattingPosition::Second,
            BattingPosition::Second => BattingPosition::Third,
            BattingPosition::Third => BattingPosition::Fourth,
            BattingPosition::Fourth => BattingPosition::Fifth,
            BattingPosition::Fifth => BattingPosition::Sixth,
            BattingPosition::Sixth => BattingPosition::Seventh,
            BattingPosition::Seventh => BattingPosition::Eighth,
            BattingPosition::Eighth => BattingPosition::Ninth,
            BattingPosition::Ninth => BattingPosition::First,
        }
    }

    pub fn as_number(self) -> u8 {
        match self {
            BattingPosition::First => 1,
            BattingPosition::Second => 2,
            BattingPosition::Third => 3,
            BattingPosition::Fourth => 4,
            BattingPosition::Fifth => 5,
            BattingPosition::Sixth => 6,
            BattingPosition::Seventh => 7,
            BattingPosition::Eighth => 8,
            BattingPosition::Ninth => 9,
        }
    }
}
