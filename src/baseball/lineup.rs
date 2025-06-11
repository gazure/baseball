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
