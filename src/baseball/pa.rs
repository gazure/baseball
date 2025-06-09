#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum Balls {
    #[default]
    Zero,
    One,
    Two,
    Three,
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum Strikes {
    #[default]
    Zero,
    One,
    Two,
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Count {
    balls: Balls,
    strikes: Strikes,
}

impl std::fmt::Display for Count {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let balls = match self.balls() {
            Balls::Zero => 0,
            Balls::One => 1,
            Balls::Two => 2,
            Balls::Three => 3,
        };

        let strikes = match self.strikes() {
            Strikes::Zero => 0,
            Strikes::One => 1,
            Strikes::Two => 2,
        };

        write!(f, "{}-{}", balls, strikes)
    }
}

impl Count {
    pub fn new(balls: Balls, strikes: Strikes) -> Self {
        Count { balls, strikes }
    }

    pub fn balls(&self) -> Balls {
        self.balls
    }

    pub fn strikes(&self) -> Strikes {
        self.strikes
    }

    pub fn advance(self, outcome: PitchOutcome) -> CountAdvance {
        match outcome {
            PitchOutcome::Ball => self.advance_ball(),
            PitchOutcome::Strike => self.advance_strike(),
            PitchOutcome::Foul => self.advance_foul(),
            _ => CountAdvance::in_progress(self),
        }
    }

    fn advance_ball(self) -> CountAdvance {
        match self.balls {
            Balls::Zero => CountAdvance::in_progress(Count::new(Balls::One, self.strikes)),
            Balls::One => CountAdvance::in_progress(Count::new(Balls::Two, self.strikes)),
            Balls::Two => CountAdvance::in_progress(Count::new(Balls::Three, self.strikes)),
            Balls::Three => CountAdvance::Walk,
        }
    }

    fn advance_strike(self) -> CountAdvance {
        match self.strikes {
            Strikes::Zero => CountAdvance::in_progress(Count::new(Balls::Zero, Strikes::One)),
            Strikes::One => CountAdvance::in_progress(Count::new(Balls::Zero, Strikes::Two)),
            Strikes::Two => CountAdvance::Strikeout,
        }
    }

    fn advance_foul(self) -> CountAdvance {
        match self.strikes {
            Strikes::Zero => CountAdvance::in_progress(Count::new(self.balls, Strikes::One)),
            Strikes::One => CountAdvance::in_progress(Count::new(self.balls, Strikes::Two)),
            Strikes::Two => CountAdvance::in_progress(self), // Foul with 2 strikes doesn't advance
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CountAdvance {
    InProgress(Count),
    Strikeout,
    Walk,
}

impl CountAdvance {
    pub fn in_progress(count: Count) -> Self {
        CountAdvance::InProgress(count)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PitchOutcome {
    Ball,
    Strike,
    Foul,
    InPlay(BallInPlay),
    HitByPitch,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BallInPlay {
    Out,
    Single,
    Double,
    Triple,
    HomeRun,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlateAppearance {
    count: Count,
}

impl Default for PlateAppearance {
    fn default() -> Self {
        Self {
            count: Count::default(),
        }
    }
}

impl PlateAppearance {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_count(count: Count) -> Self {
        Self { count }
    }

    pub fn advance(self, outcome: PitchOutcome) -> PlateAppearanceAdvance {
        match outcome {
            PitchOutcome::Ball | PitchOutcome::Strike | PitchOutcome::Foul => {
                let count_advance = self.count.advance(outcome);

                match count_advance {
                    CountAdvance::InProgress(count) => {
                        PlateAppearanceAdvance::InProgress(PlateAppearance::with_count(count))
                    }
                    CountAdvance::Strikeout => PlateAppearanceAdvance::Strikeout,
                    CountAdvance::Walk => PlateAppearanceAdvance::Walk,
                }
            }
            PitchOutcome::InPlay(ball_in_play) => match ball_in_play {
                BallInPlay::Single => PlateAppearanceAdvance::Single,
                BallInPlay::Double => PlateAppearanceAdvance::Double,
                BallInPlay::Triple => PlateAppearanceAdvance::Triple,
                BallInPlay::HomeRun => PlateAppearanceAdvance::HomeRun,
                BallInPlay::Out => PlateAppearanceAdvance::Out,
                BallInPlay::Error => PlateAppearanceAdvance::Error,
            },
            PitchOutcome::HitByPitch => PlateAppearanceAdvance::HitByPitch,
        }
    }

    pub fn count(&self) -> Count {
        self.count
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlateAppearanceAdvance {
    InProgress(PlateAppearance),
    Out,
    Walk,
    HitByPitch,
    Single,
    Double,
    Triple,
    HomeRun,
    Error,
    Strikeout,
}

impl PlateAppearanceAdvance {
    pub fn is_complete(&self) -> bool {
        match self {
            PlateAppearanceAdvance::InProgress(_) => false,
            PlateAppearanceAdvance::Out => true,
            PlateAppearanceAdvance::Walk => true,
            PlateAppearanceAdvance::HitByPitch => true,
            PlateAppearanceAdvance::Single => true,
            PlateAppearanceAdvance::Double => true,
            PlateAppearanceAdvance::Triple => true,
            PlateAppearanceAdvance::HomeRun => true,
            PlateAppearanceAdvance::Error => true,
            PlateAppearanceAdvance::Strikeout => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_new() {
        let count = Count::new(Balls::One, Strikes::Two);
        assert_eq!(count.balls(), Balls::One);
        assert_eq!(count.strikes(), Strikes::Two);
    }

    #[test]
    fn test_count_default() {
        let count = Count::default();
        assert_eq!(count.balls(), Balls::Zero);
        assert_eq!(count.strikes(), Strikes::Zero);
    }

    #[test]
    fn test_plate_appearance_new() {
        let pa = PlateAppearance::new();
        assert_eq!(pa.count(), Count::default());
    }

    #[test]
    fn test_advance_ball() {
        let pa = PlateAppearance::new();
        let pa = pa.advance(PitchOutcome::Ball);

        if let PlateAppearanceAdvance::InProgress(pa) = pa {
            assert_eq!(pa.count().balls(), Balls::One);
            assert_eq!(pa.count().strikes(), Strikes::Zero);
        } else {
            panic!("Expected in-progress plate appearance");
        }
    }

    #[test]
    fn test_advance_strike() {
        let pa = PlateAppearance::new();
        let pa = pa.advance(PitchOutcome::Strike);

        if let PlateAppearanceAdvance::InProgress(pa) = pa {
            assert_eq!(pa.count().balls(), Balls::Zero);
            assert_eq!(pa.count().strikes(), Strikes::One);
        } else {
            panic!("Expected in-progress plate appearance");
        }
    }

    #[test]
    fn test_advance_foul() {
        let pa = PlateAppearance::new();
        let pa = pa.advance(PitchOutcome::Foul);

        if let PlateAppearanceAdvance::InProgress(pa) = pa {
            assert_eq!(pa.count().balls(), Balls::Zero);
            assert_eq!(pa.count().strikes(), Strikes::One);
        } else {
            panic!("Expected in-progress plate appearance");
        }
    }

    #[test]
    fn test_advance_foul_with_two_strikes() {
        let count = Count::new(Balls::Two, Strikes::Two);
        let pa = PlateAppearance::with_count(count);
        let pa = pa.advance(PitchOutcome::Foul);

        if let PlateAppearanceAdvance::InProgress(pa) = pa {
            assert_eq!(pa.count().balls(), Balls::Two);
            assert_eq!(pa.count().strikes(), Strikes::Two);
        } else {
            panic!("Expected in-progress plate appearance");
        }
    }

    #[test]
    fn test_walk() {
        let count = Count::new(Balls::Three, Strikes::One);
        let pa = PlateAppearance::with_count(count);
        let pa = pa.advance(PitchOutcome::Ball);

        assert!(pa.is_complete());
        assert!(matches!(pa, PlateAppearanceAdvance::Walk));
    }

    #[test]
    fn test_strikeout() {
        let count = Count::new(Balls::One, Strikes::Two);
        let pa = PlateAppearance::with_count(count);
        let pa = pa.advance(PitchOutcome::Strike);

        assert!(pa.is_complete());
        assert!(matches!(pa, PlateAppearanceAdvance::Strikeout));
    }

    #[test]
    fn test_hit_by_pitch() {
        let pa = PlateAppearance::new();
        let pa = pa.advance(PitchOutcome::HitByPitch);

        assert!(pa.is_complete());
        assert!(matches!(pa, PlateAppearanceAdvance::HitByPitch));
    }

    #[test]
    fn test_single() {
        let pa = PlateAppearance::new();
        let pa = pa.advance(PitchOutcome::InPlay(BallInPlay::Single));

        assert!(pa.is_complete());
        assert!(matches!(pa, PlateAppearanceAdvance::Single));
    }

    #[test]
    fn test_home_run() {
        let count = Count::new(Balls::Two, Strikes::Two);
        let pa = PlateAppearance::with_count(count);
        let pa = pa.advance(PitchOutcome::InPlay(BallInPlay::HomeRun));

        assert!(pa.is_complete());
        assert!(matches!(pa, PlateAppearanceAdvance::HomeRun));
    }

    #[test]
    fn test_full_count_scenarios() {
        // Test a full count walk
        let pa = PlateAppearance::new();
        let pa = pa.advance(PitchOutcome::Ball); // 1-0

        let pa = if let PlateAppearanceAdvance::InProgress(pa) = pa {
            pa.advance(PitchOutcome::Strike) // 1-1
        } else {
            panic!("Expected in-progress");
        };

        let pa = if let PlateAppearanceAdvance::InProgress(pa) = pa {
            pa.advance(PitchOutcome::Ball) // 2-1
        } else {
            panic!("Expected in-progress");
        };

        let pa = if let PlateAppearanceAdvance::InProgress(pa) = pa {
            pa.advance(PitchOutcome::Strike) // 2-2
        } else {
            panic!("Expected in-progress");
        };

        let pa = if let PlateAppearanceAdvance::InProgress(pa) = pa {
            pa.advance(PitchOutcome::Ball) // 3-2
        } else {
            panic!("Expected in-progress");
        };

        let pa = if let PlateAppearanceAdvance::InProgress(pa) = pa {
            pa.advance(PitchOutcome::Ball) // Walk
        } else {
            panic!("Expected in-progress");
        };

        assert!(pa.is_complete());
        assert!(matches!(pa, PlateAppearanceAdvance::Walk));
    }
}
