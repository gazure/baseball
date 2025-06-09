use crate::baseball::inning::{HalfInning, HalfInningAdvance, InningHalf, BattingPosition};
use crate::baseball::pa::PitchOutcome;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InningNumber {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
    Ninth,
    Extra(u8), // For extra innings (10th, 11th, etc.)
}

impl InningNumber {
    pub fn next(self) -> InningNumber {
        match self {
            InningNumber::First => InningNumber::Second,
            InningNumber::Second => InningNumber::Third,
            InningNumber::Third => InningNumber::Fourth,
            InningNumber::Fourth => InningNumber::Fifth,
            InningNumber::Fifth => InningNumber::Sixth,
            InningNumber::Sixth => InningNumber::Seventh,
            InningNumber::Seventh => InningNumber::Eighth,
            InningNumber::Eighth => InningNumber::Ninth,
            InningNumber::Ninth => InningNumber::Extra(10),
            InningNumber::Extra(n) => InningNumber::Extra(n + 1),
        }
    }

    pub fn as_number(self) -> u8 {
        match self {
            InningNumber::First => 1,
            InningNumber::Second => 2,
            InningNumber::Third => 3,
            InningNumber::Fourth => 4,
            InningNumber::Fifth => 5,
            InningNumber::Sixth => 6,
            InningNumber::Seventh => 7,
            InningNumber::Eighth => 8,
            InningNumber::Ninth => 9,
            InningNumber::Extra(n) => n,
        }
    }

    pub fn is_extra(&self) -> bool {
        matches!(self, InningNumber::Extra(_))
    }
}

impl Default for InningNumber {
    fn default() -> Self {
        InningNumber::First
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GameScore {
    away: u32,
    home: u32,
}

impl GameScore {
    pub fn new() -> Self {
        GameScore { away: 0, home: 0 }
    }

    pub fn away(&self) -> u32 {
        self.away
    }

    pub fn home(&self) -> u32 {
        self.home
    }

    pub fn add_away_runs(mut self, runs: u32) -> Self {
        self.away += runs;
        self
    }

    pub fn add_home_runs(mut self, runs: u32) -> Self {
        self.home += runs;
        self
    }

    pub fn winner(&self) -> Option<GameWinner> {
        if self.away > self.home {
            Some(GameWinner::Away)
        } else if self.home > self.away {
            Some(GameWinner::Home)
        } else {
            None // Tie
        }
    }
}

impl Default for GameScore {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameWinner {
    Away,
    Home,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GameSummary {
    final_score: GameScore,
    innings_played: InningNumber,
    winner: GameWinner,
}

impl GameSummary {
    pub fn new(final_score: GameScore, innings_played: InningNumber, winner: GameWinner) -> Self {
        GameSummary {
            final_score,
            innings_played,
            winner,
        }
    }

    pub fn final_score(&self) -> &GameScore {
        &self.final_score
    }

    pub fn innings_played(&self) -> InningNumber {
        self.innings_played
    }

    pub fn winner(&self) -> GameWinner {
        self.winner
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    TopHalf,
    BottomHalf,
    Complete,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    current_inning: InningNumber,
    state: GameState,
    score: GameScore,
    current_half_inning: Option<HalfInning>,
    away_batting_order: BattingPosition,
    home_batting_order: BattingPosition,
}

impl Game {
    pub fn new() -> Self {
        let first_half = HalfInning::new(InningHalf::Top, BattingPosition::First);
        
        Game {
            current_inning: InningNumber::First,
            state: GameState::TopHalf,
            score: GameScore::new(),
            current_half_inning: Some(first_half),
            away_batting_order: BattingPosition::First,
            home_batting_order: BattingPosition::First,
        }
    }

    pub fn with_batting_orders(away_order: BattingPosition, home_order: BattingPosition) -> Self {
        let first_half = HalfInning::new(InningHalf::Top, away_order);
        
        Game {
            current_inning: InningNumber::First,
            state: GameState::TopHalf,
            score: GameScore::new(),
            current_half_inning: Some(first_half),
            away_batting_order: away_order,
            home_batting_order: home_order,
        }
    }

    pub fn current_inning(&self) -> InningNumber {
        self.current_inning
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    pub fn score(&self) -> &GameScore {
        &self.score
    }

    pub fn current_half_inning(&self) -> Option<&HalfInning> {
        self.current_half_inning.as_ref()
    }

    pub fn advance(mut self, outcome: PitchOutcome) -> GameAdvance {
        if let Some(current_half) = self.current_half_inning.take() {
            match current_half.advance(outcome) {
                HalfInningAdvance::InProgress(half_inning) => {
                    self.current_half_inning = Some(half_inning);
                    GameAdvance::InProgress(self)
                }
                HalfInningAdvance::Complete(summary) => {
                    // Half inning completed, update score and advance
                    self = self.complete_half_inning(summary.runs_scored());
                    
                    // Check if game should end
                    if self.should_end_game() {
                        let winner = self.score.winner().expect("Game should have winner");
                        let game_summary = GameSummary::new(
                            self.score.clone(),
                            self.current_inning,
                            winner
                        );
                        return GameAdvance::Complete(game_summary);
                    }

                    // Start next half inning
                    self = self.start_next_half();
                    GameAdvance::InProgress(self)
                }
            }
        } else {
            // Game is already complete
            GameAdvance::Complete(GameSummary::new(
                self.score.clone(),
                self.current_inning,
                self.score.winner().expect("Complete game should have winner")
            ))
        }
    }

    fn complete_half_inning(mut self, runs_scored: u32) -> Self {
        match self.state {
            GameState::TopHalf => {
                self.score = self.score.add_away_runs(runs_scored);
                self.state = GameState::BottomHalf;
            }
            GameState::BottomHalf => {
                self.score = self.score.add_home_runs(runs_scored);
                self.state = GameState::TopHalf;
                self.current_inning = self.current_inning.next();
            }
            GameState::Complete => {
                // Should not happen
            }
        }
        self
    }

    fn should_end_game(&self) -> bool {
        // The state represents the NEXT half inning to be played after completing a half
        match self.current_inning {
            // Regular innings 1-8: never end
            InningNumber::First | InningNumber::Second | InningNumber::Third |
            InningNumber::Fourth | InningNumber::Fifth | InningNumber::Sixth |
            InningNumber::Seventh | InningNumber::Eighth => false,
            
            // 9th inning: special ending rules
            InningNumber::Ninth => {
                match self.state {
                    GameState::TopHalf => {
                        // We're about to start top 10th (just completed bottom 9th)
                        // Game ends unless tied
                        self.score.home() != self.score.away()
                    }
                    GameState::BottomHalf => {
                        // We're about to start bottom 9th (just completed top 9th)
                        // Game ends if home team is already ahead (walk-off situation)
                        self.score.home() > self.score.away()
                    }
                    GameState::Complete => true,
                }
            }
            
            // Extra innings (10th, 11th, etc.)
            InningNumber::Extra(_) => {
                match self.state {
                    GameState::TopHalf => {
                        // We're about to start top of extra inning (just completed bottom of previous)
                        // Game ends unless tied
                        self.score.home() != self.score.away()
                    }
                    GameState::BottomHalf => {
                        // We're about to start bottom of extra inning - always play it
                        false
                    }
                    GameState::Complete => true,
                }
            }
        }
    }

    fn start_next_half(mut self) -> Self {
        let (half, batting_order) = match self.state {
            GameState::TopHalf => {
                (InningHalf::Top, self.away_batting_order)
            }
            GameState::BottomHalf => {
                (InningHalf::Bottom, self.home_batting_order)
            }
            GameState::Complete => {
                // Should not happen
                return self;
            }
        };

        self.current_half_inning = Some(HalfInning::new(half, batting_order));
        self
    }

    pub fn is_complete(&self) -> bool {
        matches!(self.state, GameState::Complete)
    }

    pub fn inning_description(&self) -> String {
        let inning_text = if self.current_inning.is_extra() {
            format!("{}th", self.current_inning.as_number())
        } else {
            match self.current_inning {
                InningNumber::First => "1st".to_string(),
                InningNumber::Second => "2nd".to_string(),
                InningNumber::Third => "3rd".to_string(),
                _ => format!("{}th", self.current_inning.as_number()),
            }
        };

        let half_text = match self.state {
            GameState::TopHalf => "Top",
            GameState::BottomHalf => "Bottom",
            GameState::Complete => return "Game Complete".to_string(),
        };

        format!("{} of the {}", half_text, inning_text)
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameAdvance {
    InProgress(Game),
    Complete(GameSummary),
}

impl GameAdvance {
    pub fn is_complete(&self) -> bool {
        matches!(self, GameAdvance::Complete(_))
    }

    pub fn game(self) -> Option<Game> {
        match self {
            GameAdvance::InProgress(game) => Some(game),
            GameAdvance::Complete(_) => None,
        }
    }

    pub fn summary(self) -> Option<GameSummary> {
        match self {
            GameAdvance::InProgress(_) => None,
            GameAdvance::Complete(summary) => Some(summary),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::baseball::pa::{BallInPlay, PitchOutcome};

    #[test]
    fn test_inning_number_progression() {
        let first = InningNumber::First;
        assert_eq!(first.as_number(), 1);
        
        let ninth = InningNumber::Ninth;
        let tenth = ninth.next();
        assert_eq!(tenth.as_number(), 10);
        assert!(tenth.is_extra());
    }

    #[test]
    fn test_game_creation() {
        let game = Game::new();
        assert_eq!(game.current_inning(), InningNumber::First);
        assert_eq!(game.state(), &GameState::TopHalf);
        assert_eq!(game.score().away(), 0);
        assert_eq!(game.score().home(), 0);
        assert!(game.current_half_inning().is_some());
    }

    #[test]
    fn test_game_score_tracking() {
        let mut score = GameScore::new();
        score = score.add_away_runs(3);
        score = score.add_home_runs(2);
        
        assert_eq!(score.away(), 3);
        assert_eq!(score.home(), 2);
        assert_eq!(score.winner(), Some(GameWinner::Away));
    }

    #[test]
    fn test_simple_half_inning_completion() {
        let game = Game::new();
        
        // Three quick outs to complete top 1st
        let game = game.advance(PitchOutcome::InPlay(BallInPlay::Out))
            .game().expect("Game should continue")
            .advance(PitchOutcome::InPlay(BallInPlay::Out))
            .game().expect("Game should continue")
            .advance(PitchOutcome::InPlay(BallInPlay::Out))
            .game().expect("Game should continue");
        
        // Should now be bottom of 1st
        assert_eq!(game.state(), &GameState::BottomHalf);
        assert_eq!(game.current_inning(), InningNumber::First);
    }

    #[test]
    fn test_home_run_scoring() {
        let game = Game::new();
        
        // Home run in top 1st
        let game = game.advance(PitchOutcome::InPlay(BallInPlay::HomeRun))
            .game().expect("Game should continue");
        
        // Complete top 1st with two outs
        let game = game.advance(PitchOutcome::InPlay(BallInPlay::Out))
            .game().expect("Game should continue")
            .advance(PitchOutcome::InPlay(BallInPlay::Out))
            .game().expect("Game should continue")
            .advance(PitchOutcome::InPlay(BallInPlay::Out))
            .game().expect("Game should continue");
        
        // Should be bottom 1st with 1-0 score
        assert_eq!(game.state(), &GameState::BottomHalf);
        assert_eq!(game.score().away(), 1);
        assert_eq!(game.score().home(), 0);
    }

    #[test]
    fn test_inning_description() {
        let game = Game::new();
        assert_eq!(game.inning_description(), "Top of the 1st");
        
        let game = Game::with_batting_orders(BattingPosition::First, BattingPosition::First);
        // Simulate completing top half
        let mut game_state = game;
        game_state.state = GameState::BottomHalf;
        assert_eq!(game_state.inning_description(), "Bottom of the 1st");
        
        // Test extra innings
        game_state.current_inning = InningNumber::Extra(12);
        game_state.state = GameState::TopHalf;
        assert_eq!(game_state.inning_description(), "Top of the 12th");
    }

    #[test]
    fn test_game_ending_conditions() {
        let mut game = Game::new();
        
        // Simulate game state at end of 9th inning
        game.current_inning = InningNumber::Ninth;
        game.state = GameState::TopHalf; // Bottom 9th just completed
        game.score = game.score.add_home_runs(5).add_away_runs(3);
        
        // Home team ahead, game should end
        assert!(game.should_end_game());
        
        // Test tie game - should not end
        game.score = GameScore::new().add_home_runs(3).add_away_runs(3);
        assert!(!game.should_end_game());
    }
}