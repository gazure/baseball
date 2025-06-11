use tracing::info;

use crate::{
    baseball::{
        baserunners::BaserunnerState,
        lineup::BattingPosition,
        pa::{PitchOutcome, PlateAppearance, PlateAppearanceAdvance},
    }, Runs
};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum InningHalf {
    #[default]
    Top,
    Bottom,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Outs {
    #[default]
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

    pub fn as_number(self) -> Runs {
        match self {
            Outs::Zero => 0,
            Outs::One => 1,
            Outs::Two => 2,
            Outs::Three => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HalfInning {
    half: InningHalf,
    outs: Outs,
    current_batter: BattingPosition,
    current_pa: PlateAppearance,
    runs_scored: Runs,
    baserunners: BaserunnerState,
}

impl Default for HalfInning {
    fn default() -> Self {
        HalfInning {
            half: InningHalf::default(),
            outs: Outs::default(),
            current_batter: BattingPosition::default(),
            current_pa: PlateAppearance::new(),
            runs_scored: 0,
            baserunners: BaserunnerState::new(),
        }
    }
}

impl HalfInning {
    pub fn new(half: InningHalf, starting_batter: BattingPosition) -> Self {
        HalfInning {
            half,
            outs: Outs::Zero,
            current_batter: starting_batter,
            current_pa: PlateAppearance::new(),
            runs_scored: 0,
            baserunners: BaserunnerState::new(),
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

    pub fn runs_scored(&self) -> Runs {
        self.runs_scored
    }

    pub fn baserunners(&self) -> BaserunnerState {
        self.baserunners
    }

    fn increment_outs(self, n: u32) -> HalfInningAdvance {
        let mut outs = self.outs;
        for _ in 0..n {
            outs = outs.add_out();

            if matches!(outs, Outs::Three) {
                info!("Inning completed with {:?} outs", outs);
                info!("Halfinning: {:?}", self);
                return HalfInningAdvance::Complete(HalfInningSummary::new(self.runs_scored));
            }
        }
        info!("Halfinning: {:?}, new outs: {:?}", self, outs);
        self.set_outs(outs).advance_batter()
    }

    pub fn advance(mut self, outcome: PitchOutcome) -> HalfInningAdvance {
        let pa = self.current_pa.advance(outcome);

        match pa {
            PlateAppearanceAdvance::Strikeout => self.increment_outs(1),
            PlateAppearanceAdvance::InPlay(outcome) => {
                let outs = outcome.outs();
                let baserunners = outcome.baserunners();
                let runs_scored = outcome.runs_scored();
                self.add_runs(runs_scored).with_baserunners(baserunners).increment_outs(outs)
            }
            PlateAppearanceAdvance::Walk => {
                let (baserunners, runs) = self.baserunners.walk(self.current_batter);
                self.add_runs(runs).with_baserunners(baserunners).advance_batter()
            }
            PlateAppearanceAdvance::HitByPitch => {
                let (baserunners, runs) = self.baserunners.walk(self.current_batter);
                self.add_runs(runs).with_baserunners(baserunners).advance_batter()
            }
            PlateAppearanceAdvance::HomeRun => {
                let runs = self.baserunners.home_run();
                self.add_runs(runs).with_baserunners(BaserunnerState::empty()).advance_batter()
            }
            PlateAppearanceAdvance::InProgress(pa) => {
                self.current_pa = pa;
                HalfInningAdvance::in_progress(self)
            }
        }
    }

    fn set_outs(mut self, outs: Outs) -> Self {
        self.outs = outs;
        self
    }

    fn advance_batter(mut self) -> HalfInningAdvance {
        self.current_batter = self.current_batter.next();
        self.current_pa = PlateAppearance::new();
        HalfInningAdvance::in_progress(self)
    }

    fn add_runs(mut self, runs_scored: Runs) -> Self {
        self.runs_scored += runs_scored;
        self
    }

    fn with_baserunners(mut self, baserunners: BaserunnerState) -> Self {
        self.baserunners = baserunners;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HalfInningSummary {
    runs_scored: Runs,
}

impl HalfInningSummary {
    pub fn new(runs_scored: Runs) -> Self {
        HalfInningSummary { runs_scored }
    }

    pub fn runs_scored(&self) -> Runs {
        self.runs_scored
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum HalfInningAdvance {
    InProgress(HalfInning),
    Complete(HalfInningSummary),
}

impl HalfInningAdvance {
    pub fn advance(self, pitch: PitchOutcome) -> HalfInningAdvance {
        match self {
            HalfInningAdvance::InProgress(hi) => hi.advance(pitch),
            HalfInningAdvance::Complete(_) => self,
        }
    }

    pub fn is_complete(&self) -> bool {
        matches!(self, HalfInningAdvance::Complete(_))
    }

    pub fn half_inning(&self) -> Option<HalfInning> {
        match self {
            HalfInningAdvance::InProgress(hi) => Some(*hi),
            HalfInningAdvance::Complete(_) => None,
        }
    }

    pub fn half_inning_ref(&self) -> Option<&HalfInning> {
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
    use crate::baseball::{baserunners::PlayOutcome, pa::PitchOutcome};

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

        let result = half_inning.advance(PitchOutcome::HomeRun);
        let half_inning = result.half_inning().expect("unexpected inning end");

        assert_eq!(half_inning.runs_scored(), 1);
        assert_eq!(half_inning.current_batter().as_number(), 2); // Next batter
    }

    #[test]
    fn test_three_outs_ends_half_inning() {
        let batting_pos = BattingPosition::First;
        let half_inning = HalfInning::new(InningHalf::Top, batting_pos);

        let advance = half_inning
            .advance(PitchOutcome::InPlay(PlayOutcome::groundout()))
            .half_inning()
            .expect("unexpected inning end")
            .advance(PitchOutcome::InPlay(PlayOutcome::groundout()))
            .half_inning()
            .expect("unexpected inning end")
            .advance(PitchOutcome::InPlay(PlayOutcome::groundout()));

        assert!(advance.is_complete());
    }
}
