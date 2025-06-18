use std::fmt::Display;

use crate::{Runs, baseball::lineup::BattingPosition};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Base {
    First,
    Second,
    Third,
    Home,
}

impl Base {
    pub fn next(self) -> Base {
        match self {
            Base::First => Base::Second,
            Base::Second => Base::Third,
            Base::Third => Base::Home,
            Base::Home => Base::Home, // Can't advance past home
        }
    }

    pub fn advance_by(self, bases: u8) -> Base {
        let mut current = self;
        for _ in 0..bases {
            if current == Base::Home {
                break;
            }
            current = current.next();
        }
        current
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BaseOutcome {
    ForceOut,
    TagOut,
    Runner(BattingPosition),
    None,
}

impl Display for BaseOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BaseOutcome::ForceOut => write!(f, "Force Out"),
            BaseOutcome::TagOut => write!(f, "Tag Out"),
            BaseOutcome::Runner(batting_position) => write!(f, "Runner: {}", batting_position),
            BaseOutcome::None => write!(f, "None"),
        }
    }
}

impl BaseOutcome {
    pub fn outs(&self) -> u32 {
        match self {
            BaseOutcome::ForceOut | BaseOutcome::TagOut => 1,
            _ => 0,
        }
    }

    pub fn is_out(&self) -> bool {
        self.outs() > 0
    }

    fn as_basrunner(self) -> Option<BattingPosition> {
        match self {
            BaseOutcome::Runner(batting_position) => Some(batting_position),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HomeOutcome {
    One,
    Two,
    Three,
    Four,
    None,
    Out,
}

impl Display for HomeOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HomeOutcome::One => write!(f, "1"),
            HomeOutcome::Two => write!(f, "2"),
            HomeOutcome::Three => write!(f, "3"),
            HomeOutcome::Four => write!(f, "4"),
            HomeOutcome::None => write!(f, "0"),
            HomeOutcome::Out => write!(f, "X"),
        }
    }
}

impl HomeOutcome {
    pub fn outs(self) -> u32 {
        if self == Self::Out { 1 } else { 0 }
    }

    pub fn is_out(self) -> bool {
        self.outs() > 0
    }

    fn runs_scored(self) -> Runs {
        match self {
            HomeOutcome::One => 1,
            HomeOutcome::Two => 2,
            HomeOutcome::Three => 3,
            HomeOutcome::Four => 4,
            HomeOutcome::None => 0,
            HomeOutcome::Out => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayOutcome {
    first: BaseOutcome,
    second: BaseOutcome,
    third: BaseOutcome,
    home: HomeOutcome,
}

impl Display for PlayOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}, {}", self.first, self.second, self.third, self.home)
    }
}

impl PlayOutcome {
    pub fn new(
        first: BaseOutcome,
        second: BaseOutcome,
        third: BaseOutcome,
        home: HomeOutcome,
    ) -> Self {
        PlayOutcome {
            first,
            second,
            third,
            home,
        }
    }

    pub fn groundout() -> Self {
        PlayOutcome {
            first: BaseOutcome::ForceOut,
            second: BaseOutcome::None,
            third: BaseOutcome::None,
            home: HomeOutcome::None,
        }
    }

    pub fn single(baserunners: BaserunnerState, batter: BattingPosition) -> PlayOutcome {
        PlayOutcome {
            first: BaseOutcome::Runner(batter),
            second: baserunners
                .first()
                .map(BaseOutcome::Runner)
                .unwrap_or(BaseOutcome::None),
            third: baserunners
                .second()
                .map(BaseOutcome::Runner)
                .unwrap_or(BaseOutcome::None),
            home: Self::scored(None, None, baserunners.third(), None),
        }
    }

    pub fn double(baserunners: BaserunnerState, batter: BattingPosition) -> PlayOutcome {
        PlayOutcome {
            first: BaseOutcome::None,
            second: BaseOutcome::Runner(batter),
            third: baserunners
                .first()
                .map(BaseOutcome::Runner)
                .unwrap_or(BaseOutcome::None),
            home: Self::scored(None, baserunners.second(), baserunners.third(), None),
        }
    }

    pub fn triple(baserunners: BaserunnerState, batter: BattingPosition) -> PlayOutcome {
        let home = Self::scored(baserunners.first(), baserunners.second(), baserunners.third(), None);
        PlayOutcome {
            first: BaseOutcome::None,
            second: BaseOutcome::None,
            third: BaseOutcome::Runner(batter),
            home,
        }
    }

    pub fn homerun(baserunners: BaserunnerState, batter: BattingPosition) -> PlayOutcome {
        PlayOutcome {
            first: BaseOutcome::None,
            second: BaseOutcome::None,
            third: BaseOutcome::None,
            home: Self::scored(
                baserunners.first(),
                baserunners.second(),
                baserunners.third(),
                Some(batter),
            ),
        }
    }

    pub fn outs(self) -> u32 {
        self.first().outs() + self.second().outs() + self.third().outs() + self.home.outs()
    }

    pub fn first(self) -> BaseOutcome {
        self.first
    }

    pub fn second(self) -> BaseOutcome {
        self.second
    }

    pub fn third(self) -> BaseOutcome {
        self.third
    }

    pub fn home(self) -> HomeOutcome {
        self.home
    }

    pub fn with_first(self, first: BaseOutcome) -> Self {
        Self {
            first,
            second: self.second,
            third: self.third,
            home: self.home,
        }
    }

    pub fn with_second(self, second: BaseOutcome) -> Self {
        Self {
            first: self.first,
            second,
            third: self.third,
            home: self.home,
        }
    }

    pub fn with_third(self, third: BaseOutcome) -> Self {
        Self {
            first: self.first,
            second: self.second,
            third,
            home: self.home,
        }
    }

    pub fn with_home(self, home: HomeOutcome) -> Self {
        Self {
            first: self.first,
            second: self.second,
            third: self.third,
            home,
        }
    }

    fn scored(
        first: Option<BattingPosition>,
        second: Option<BattingPosition>,
        third: Option<BattingPosition>,
        batter: Option<BattingPosition>,
    ) -> HomeOutcome {
        match (first, second, third, batter) {
            (None, None, None, None) => HomeOutcome::None,
            (None, None, None, Some(_)) => HomeOutcome::One,
            (None, None, Some(_), None) => HomeOutcome::Two,
            (None, None, Some(_), Some(_)) => HomeOutcome::Three,
            (None, Some(_), None, None) => HomeOutcome::One,
            (None, Some(_), None, Some(_)) => HomeOutcome::Two,
            (None, Some(_), Some(_), None) => HomeOutcome::Two,
            (None, Some(_), Some(_), Some(_)) => HomeOutcome::Three,
            (Some(_), None, None, None) => HomeOutcome::One,
            (Some(_), None, None, Some(_)) => HomeOutcome::Two,
            (Some(_), None, Some(_), None) => HomeOutcome::Two,
            (Some(_), None, Some(_), Some(_)) => HomeOutcome::Three,
            (Some(_), Some(_), None, None) => HomeOutcome::Two,
            (Some(_), Some(_), None, Some(_)) => HomeOutcome::Three,
            (Some(_), Some(_), Some(_), None) => HomeOutcome::Three,
            (Some(_), Some(_), Some(_), Some(_)) => HomeOutcome::Four,
        }
    }

    pub fn baserunners(self) -> BaserunnerState {
        BaserunnerState {
            first: self.first.as_basrunner(),
            second: self.second.as_basrunner(),
            third: self.third.as_basrunner(),
        }
    }

    pub fn runs_scored(self) -> Runs {
        self.home.runs_scored()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BaserunnerState {
    first: Option<BattingPosition>,
    second: Option<BattingPosition>,
    third: Option<BattingPosition>,
}

impl BaserunnerState {
    pub fn new() -> Self {
        BaserunnerState {
            first: None,
            second: None,
            third: None,
        }
    }

    pub fn empty() -> Self {
        Self::new()
    }

    pub fn is_empty(&self) -> bool {
        self.first.is_none() && self.second.is_none() && self.third.is_none()
    }

    pub fn first(&self) -> Option<BattingPosition> {
        self.first
    }

    pub fn second(&self) -> Option<BattingPosition> {
        self.second
    }

    pub fn third(&self) -> Option<BattingPosition> {
        self.third
    }

    pub fn set_first(mut self, runner: Option<BattingPosition>) -> Self {
        self.first = runner;
        self
    }

    pub fn set_second(mut self, runner: Option<BattingPosition>) -> Self {
        self.second = runner;
        self
    }

    pub fn set_third(mut self, runner: Option<BattingPosition>) -> Self {
        self.third = runner;
        self
    }

    pub fn runner_count(&self) -> u8 {
        let mut count = 0;
        if self.first.is_some() {
            count += 1;
        }
        if self.second.is_some() {
            count += 1;
        }
        if self.third.is_some() {
            count += 1;
        }
        count
    }

    pub fn has_runner_on(&self, base: Base) -> bool {
        match base {
            Base::First => self.first.is_some(),
            Base::Second => self.second.is_some(),
            Base::Third => self.third.is_some(),
            Base::Home => false, // No one stays on home
        }
    }

    pub fn walk(&self, batter: BattingPosition) -> (BaserunnerState, Runs) {
        let mut new_state = BaserunnerState::new().set_first(Some(batter));
        let mut runs_scored = Runs::default();

        if let Some(runner) = self.first {
            new_state = new_state.set_second(Some(runner));
        }
        if let Some(runner) = self.second {
            new_state = new_state.set_third(Some(runner));
        }
        if self.third.is_some() {
            runs_scored += 1;
        }

        (new_state, runs_scored)
    }

    pub fn home_run(&self) -> Runs {
        let mut runs = 1;
        if self.first.is_some() {
            runs += 1;
        }
        if self.second.is_some() {
            runs += 1;
        }
        if self.third.is_some() {
            runs += 1;
        }
        runs
    }
}

impl Default for BaserunnerState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_base_advancement() {
        assert_eq!(Base::First.next(), Base::Second);
        assert_eq!(Base::Second.next(), Base::Third);
        assert_eq!(Base::Third.next(), Base::Home);
        assert_eq!(Base::Home.next(), Base::Home);
    }

    #[test]
    fn test_base_advance_by() {
        assert_eq!(Base::First.advance_by(2), Base::Third);
        assert_eq!(Base::Second.advance_by(2), Base::Home);
        assert_eq!(Base::Third.advance_by(5), Base::Home); // Can't go past home
    }

    #[test]
    fn test_baserunner_state_creation() {
        let state = BaserunnerState::new();
        assert!(state.is_empty());
        assert_eq!(state.runner_count(), 0);
        assert!(!state.has_runner_on(Base::First));
    }

    #[test]
    fn test_baserunner_state_with_runners() {
        let state = BaserunnerState::new()
            .set_first(Some(BattingPosition::First))
            .set_third(Some(BattingPosition::Third));

        assert!(!state.is_empty());
        assert_eq!(state.runner_count(), 2);
        assert!(state.has_runner_on(Base::First));
        assert!(!state.has_runner_on(Base::Second));
        assert!(state.has_runner_on(Base::Third));
    }

}
