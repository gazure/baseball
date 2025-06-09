use crate::baseball::pa::{PitchOutcome, PlateAppearance, PlateAppearanceAdvance};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InningHalf {
    Top,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Outs {
    Zero,
    One,
    Two,
    Three, // Side is retired
}

impl Outs {
    pub fn add_out(self) -> Outs {
        match self {
            Outs::Zero => Outs::One,
            Outs::One => Outs::Two,
            Outs::Two => Outs::Three,
            Outs::Three => Outs::Three, // Stay at three
        }
    }

    pub fn as_number(self) -> u8 {
        match self {
            Outs::Zero => 0,
            Outs::One => 1,
            Outs::Two => 2,
            Outs::Three => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(Default)]
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


#[derive(Debug, Clone, PartialEq)]
pub struct HalfInning {
    half: InningHalf,
    outs: Outs,
    current_batter: BattingPosition,
    current_pa: PlateAppearance,
    runs_scored: u32,
}

impl HalfInning {
    pub fn new(half: InningHalf, starting_batter: BattingPosition) -> Self {
        HalfInning {
            half,
            outs: Outs::Zero,
            current_batter: starting_batter,
            current_pa: PlateAppearance::new(),
            runs_scored: 0,
        }
    }

    pub fn half(&self) -> InningHalf {
        self.half
    }

    pub fn outs(&self) -> Outs {
        self.outs
    }

    pub fn current_batter(&self) -> BattingPosition {
        self.current_batter
    }

    pub fn current_plate_appearance(&self) -> &PlateAppearance {
        &self.current_pa
    }

    pub fn runs_scored(&self) -> u32 {
        self.runs_scored
    }

    pub fn advance(mut self, outcome: PitchOutcome) -> HalfInningAdvance {
        let pa = self.current_pa.advance(outcome);

        match pa {
            PlateAppearanceAdvance::Out | PlateAppearanceAdvance::Strikeout => {
                let outs = self.outs.add_out();
                match outs {
                    Outs::Zero | Outs::One | Outs::Two => {
                        self.set_outs(outs).to_advance()
                    }
                    Outs::Three => {
                        HalfInningAdvance::Complete(HalfInningSummary::new(self.runs_scored))
                    }
                }
            }
            PlateAppearanceAdvance::Walk
            | PlateAppearanceAdvance::HitByPitch
            | PlateAppearanceAdvance::Single
            | PlateAppearanceAdvance::Error => {
                self.to_advance()
            }
            PlateAppearanceAdvance::Double => {
                self.to_advance()
            }
            PlateAppearanceAdvance::Triple => {
                self.to_advance()
            }
            PlateAppearanceAdvance::HomeRun => {
                self.add_runs(1).to_advance()
            }
            PlateAppearanceAdvance::InProgress(pa) => {
                self.current_pa = pa;
                HalfInningAdvance::in_progress(self)
            }
        }
    }

    fn advance_batter(mut self) -> Self {
        self.current_batter = self.current_batter.next();
        self
    }

    fn set_outs(mut self, outs: Outs) -> Self {
        self.outs = outs;
        self
    }

    fn add_runs(mut self, runs: u32) -> Self {
        self.runs_scored += runs;
        self
    }

    fn to_advance(self) -> HalfInningAdvance {
        let s = self.advance_batter();
        HalfInningAdvance::in_progress(s)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HalfInningSummary {
    runs_scored: u32,
}

impl HalfInningSummary {
    pub fn new(runs_scored: u32) -> Self {
        HalfInningSummary { runs_scored }
    }

    pub fn runs_scored(&self) -> u32 {
        self.runs_scored
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum HalfInningAdvance {
    InProgress(HalfInning),
    Complete(HalfInningSummary),
}

impl HalfInningAdvance {
    pub fn is_complete(&self) -> bool {
        matches!(self, HalfInningAdvance::Complete(_))
    }

    pub fn half_inning(self) -> Option<HalfInning> {
        match self {
            HalfInningAdvance::InProgress(hi) => Some(hi),
            HalfInningAdvance::Complete(_) => None,
        }
    }

    fn in_progress(hi: HalfInning) -> HalfInningAdvance {
        HalfInningAdvance::InProgress(hi)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::baseball::pa::{BallInPlay, PitchOutcome};

    #[test]
    fn test_batting_position_as_number() {
        assert_eq!(BattingPosition::First.as_number(), 1);
        assert_eq!(BattingPosition::Ninth.as_number(), 9);
    }

    #[test]
    fn test_batting_position_next() {
        let pos1 = BattingPosition::First;
        let pos2 = pos1.next();
        assert_eq!(pos2.as_number(), 2);

        let pos9 = BattingPosition::Ninth;
        let pos1_again = pos9.next();
        assert_eq!(pos1_again.as_number(), 1);
    }

    #[test]
    fn test_outs_progression() {
        let outs = Outs::Zero;
        let outs = outs.add_out();
        assert_eq!(outs, Outs::One);

        let outs = outs.add_out();
        assert_eq!(outs, Outs::Two);

        let outs = outs.add_out();
        assert_eq!(outs, Outs::Three);
    }

    #[test]
    fn test_half_inning_creation() {
        let batting_pos = BattingPosition::Third;
        let half_inning = HalfInning::new(InningHalf::Top, batting_pos);

        assert_eq!(half_inning.half(), InningHalf::Top);
        assert_eq!(half_inning.outs(), Outs::Zero);
        assert_eq!(half_inning.current_batter().as_number(), 3);
        assert_eq!(half_inning.runs_scored(), 0);
    }

    #[test]
    fn test_half_inning_strikeout() {
        let batting_pos = BattingPosition::First;
        let half_inning = HalfInning::new(InningHalf::Top, batting_pos);

        // Simulate a strikeout (3 strikes)
        let half_inning = half_inning
            .advance(PitchOutcome::Strike)
            .half_inning()
            .expect("unexpected inning end")
            .advance(PitchOutcome::Strike)
            .half_inning()
            .expect("unexpected inning end")
            .advance(PitchOutcome::Strike)
            .half_inning()
            .expect("unexpected inning end");

        assert_eq!(half_inning.outs(), Outs::One);
        assert_eq!(half_inning.current_batter().as_number(), 2); // Next batter
    }

    #[test]
    fn test_half_inning_home_run() {
        let batting_pos = BattingPosition::First;
        let half_inning = HalfInning::new(InningHalf::Top, batting_pos);

        let result = half_inning.advance(PitchOutcome::InPlay(BallInPlay::HomeRun));
        let half_inning = result.half_inning().expect("unexpected inning end");

        assert_eq!(half_inning.runs_scored(), 1);
        assert_eq!(half_inning.current_batter().as_number(), 2); // Next batter
    }

    #[test]
    fn test_three_outs_ends_half_inning() {
        let batting_pos = BattingPosition::First;
        let half_inning = HalfInning::new(InningHalf::Top, batting_pos);

        let advance = half_inning.advance(PitchOutcome::InPlay(BallInPlay::Out))
            .half_inning()
            .expect("unexpected inning end")
            .advance(PitchOutcome::InPlay(BallInPlay::Out))
            .half_inning()
            .expect("unexpected inning end")
            .advance(PitchOutcome::InPlay(BallInPlay::Out));

        assert!(advance.is_complete());
    }
}
