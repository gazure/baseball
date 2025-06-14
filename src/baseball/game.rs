use std::{fmt::Display, ptr::write};

use crate::{
    Runs,
    baseball::{
        inning::{HalfInning, HalfInningResult, InningHalf},
        lineup::BattingPosition,
        pa::PitchOutcome,
    },
};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum InningNumber {
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GameScore {
    away: Runs,
    home: Runs,
}

impl Display for GameScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Away: {} - Home: {}", self.away, self.home)
    }
}

impl GameScore {
    pub fn new() -> Self {
        GameScore { away: 0, home: 0 }
    }

    pub fn away(&self) -> Runs {
        self.away
    }

    pub fn home(&self) -> Runs {
        self.home
    }

    pub fn add_away_runs(mut self, runs: Runs) -> Self {
        self.away += runs;
        self
    }

    pub fn add_home_runs(mut self, runs: Runs) -> Self {
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

impl Display for GameSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Final Score: Away: {} - Home: {}", self.final_score.away, self.final_score.home)
    }
}

impl GameSummary {
    pub fn new(final_score: GameScore, innings_played: InningNumber, winner: GameWinner) -> Self {
        GameSummary {
            final_score,
            innings_played,
            winner,
        }
    }

    pub fn final_score(&self) -> GameScore {
        self.final_score
    }

    pub fn innings_played(&self) -> InningNumber {
        self.innings_played
    }

    pub fn winner(&self) -> GameWinner {
        self.winner
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Inning(InningHalf),
    InningEnd(InningHalf),
    Complete,
}

impl GameState {
    pub fn is_bottom(&self) -> bool {
        matches!(self, GameState::Inning(InningHalf::Bottom))
    }

    pub fn is_top(&self) -> bool {
        matches!(self, GameState::Inning(InningHalf::Top))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    current_inning: InningNumber,
    state: GameState,
    score: GameScore,
    current_half_inning: HalfInning,
    away_batting_order: BattingPosition,
    home_batting_order: BattingPosition,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} Score: {}", self.inning_description(), self.score())
    }
}

impl Game {
    pub fn new() -> Self {
        Game {
            current_inning: InningNumber::First,
            state: GameState::Inning(InningHalf::Top),
            score: GameScore::new(),
            current_half_inning: HalfInning::new(InningHalf::Top, BattingPosition::First),
            away_batting_order: BattingPosition::First,
            home_batting_order: BattingPosition::First,
        }
    }

    pub fn with_batting_orders(away_order: BattingPosition, home_order: BattingPosition) -> Self {
        let first_half = HalfInning::new(InningHalf::Top, away_order);

        Game {
            current_inning: InningNumber::First,
            state: GameState::Inning(InningHalf::Top),
            score: GameScore::new(),
            current_half_inning: first_half,
            away_batting_order: away_order,
            home_batting_order: home_order,
        }
    }

    pub fn current_inning(&self) -> InningNumber {
        self.current_inning
    }

    pub fn state(&self) -> GameState {
        self.state
    }

    pub fn score(&self) -> GameScore {
        self.score
    }

    pub fn current_half_inning(&self) -> &HalfInning {
        &self.current_half_inning
    }

    pub fn advance(mut self, outcome: PitchOutcome) -> GameResult {
        match self.current_half_inning.advance(outcome) {
            HalfInningResult::InProgress(half_inning) => {
                self.current_half_inning = half_inning;
                let pending_runs = self.current_half_inning.runs_scored();
                if self.is_bottom_of_ninth() && self.should_end_game(pending_runs) {
                    self.complete_half_inning(pending_runs);
                    let winner = self.score.winner().expect("Game should have winner");
                    let game_summary = GameSummary::new(self.score, self.current_inning, winner);
                    return GameResult::Complete(game_summary);
                }

                GameResult::InProgress(self)
            }

            HalfInningResult::Complete(summary) => {
                // Half inning completed, update score and advance
                self.complete_half_inning(summary.runs_scored());

                // Check if game should end
                if self.should_end_game(0) {
                    let winner = self.score.winner().expect("Game should have winner");
                    let game_summary = GameSummary::new(self.score, self.current_inning, winner);
                    return GameResult::Complete(game_summary);
                }

                // Start next half inning
                self = self.start_next_half();
                GameResult::InProgress(self)
            }
        }
    }

    fn complete_half_inning(&mut self, pending_runs: Runs) {
        match self.state {
            GameState::Inning(InningHalf::Top) => {
                self.score = self.score.add_away_runs(pending_runs);
                self.state = GameState::InningEnd(InningHalf::Top)
            }
            GameState::Inning(InningHalf::Bottom) => {
                self.score = self.score.add_home_runs(pending_runs);
                self.state = GameState::InningEnd(InningHalf::Bottom);
            }
            GameState::InningEnd(_) | GameState::Complete => {
                // Should not happen
            }
        }
    }

    fn should_end_game(&self, pending_runs: Runs) -> bool {
        // The state represents the NEXT half inning to be played after completing a half
        match self.current_inning {
            // Regular innings 1-8: never end
            InningNumber::First
            | InningNumber::Second
            | InningNumber::Third
            | InningNumber::Fourth
            | InningNumber::Fifth
            | InningNumber::Sixth
            | InningNumber::Seventh
            | InningNumber::Eighth => false,

            // 9th inning: special ending rules
            InningNumber::Ninth => {
                match self.state {
                    GameState::InningEnd(InningHalf::Top) => {
                        // Just finished top of 9th
                        // Game ends if home team is winning
                        self.score.home() > self.score.away()
                    }
                    GameState::InningEnd(InningHalf::Bottom) => {
                        // Just finished bottom of 9th
                        // Game ends if any team is winning
                        self.score.home() != self.score.away()
                    }
                    GameState::Inning(InningHalf::Top) => false,
                    GameState::Inning(InningHalf::Bottom) => {
                        self.score().home() + pending_runs > self.score().away()
                    }
                    GameState::Complete => true,
                }
            }

            // Extra innings (10th, 11th, etc.)
            InningNumber::Extra(_) => {
                match self.state {
                    GameState::InningEnd(InningHalf::Top) => false,
                    GameState::InningEnd(InningHalf::Bottom) => {
                        // Just finished bottom of extra inning
                        // Game ends if any team is winning
                        self.score.home() != self.score.away()
                    }
                    GameState::Inning(_) => false,
                    GameState::Complete => true,
                }
            }
        }
    }

    fn start_next_half(mut self) -> Self {
        let (half, batting_order) = match self.state {
            GameState::InningEnd(InningHalf::Top) => (InningHalf::Bottom, self.home_batting_order),
            GameState::InningEnd(InningHalf::Bottom) => (InningHalf::Top, self.away_batting_order),
            GameState::Inning(_) | GameState::Complete => {
                // Should not happen
                return self;
            }
        };

        if let InningHalf::Top = half {
            self.current_inning = self.current_inning.next();
        }

        self.current_half_inning = HalfInning::new(half, batting_order);
        self.state = GameState::Inning(half);
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
            GameState::Inning(InningHalf::Top) => "Top",
            GameState::Inning(InningHalf::Bottom) => "Bottom",
            GameState::InningEnd(InningHalf::Top) => "Mid",
            GameState::InningEnd(InningHalf::Bottom) => "End",
            GameState::Complete => return "Game Complete".to_string(),
        };

        format!("{} of the {}", half_text, inning_text)
    }

    fn is_bottom_of_ninth(&self) -> bool {
        self.current_inning == InningNumber::Ninth && self.state.is_bottom()
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameResult {
    InProgress(Game),
    Complete(GameSummary),
}

impl Display for GameResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameResult::InProgress(game) => write!(f, "{}", game),
            GameResult::Complete(summary) => write!(f, "{}", summary),
        }
    }
}

impl GameResult {
    pub fn advance(self, outcome: PitchOutcome) -> GameResult {
        match self {
            GameResult::InProgress(game) => game.advance(outcome),
            GameResult::Complete(_) => self, // Already complete, ignore the pitch
        }
    }

    pub fn is_complete(&self) -> bool {
        matches!(self, GameResult::Complete(_))
    }

    pub fn game(self) -> Option<Game> {
        match self {
            GameResult::InProgress(game) => Some(game),
            GameResult::Complete(_) => None,
        }
    }

    pub fn game_ref(&self) -> Option<&Game> {
        match self {
            GameResult::InProgress(game) => Some(game),
            GameResult::Complete(_) => None,
        }
    }

    pub fn summary(self) -> Option<GameSummary> {
        match self {
            GameResult::InProgress(_) => None,
            GameResult::Complete(summary) => Some(summary),
        }
    }

    pub fn summary_ref(&self) -> Option<&GameSummary> {
        match self {
            GameResult::InProgress(_) => None,
            GameResult::Complete(summary) => Some(summary),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::baseball::{baserunners::PlayOutcome, pa::PitchOutcome};

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
        assert_eq!(game.state(), GameState::Inning(InningHalf::Top));
        assert_eq!(game.score().away(), 0);
        assert_eq!(game.score().home(), 0);
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
        let game = game
            .advance(PitchOutcome::InPlay(PlayOutcome::groundout()))
            .game()
            .expect("Game should continue")
            .advance(PitchOutcome::InPlay(PlayOutcome::groundout()))
            .game()
            .expect("Game should continue")
            .advance(PitchOutcome::InPlay(PlayOutcome::groundout()))
            .game()
            .expect("Game should continue");

        // Should now be bottom of 1st
        assert_eq!(game.state(), GameState::Inning(InningHalf::Bottom));
        assert_eq!(game.current_inning(), InningNumber::First);
    }

    #[test]
    fn test_home_run_scoring() {
        let game = Game::new();

        // Home run in top 1st
        let game = game
            .advance(PitchOutcome::HomeRun)
            .game()
            .expect("Game should continue");

        // Complete top 1st with two outs
        let game = game
            .advance(PitchOutcome::InPlay(PlayOutcome::groundout()))
            .advance(PitchOutcome::InPlay(PlayOutcome::groundout()))
            .advance(PitchOutcome::InPlay(PlayOutcome::groundout()))
            .game()
            .expect("Game should continue");

        // Should be bottom 1st with 1-0 score
        assert_eq!(game.state(), GameState::Inning(InningHalf::Bottom));
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
        game_state.state = GameState::Inning(InningHalf::Bottom);
        assert_eq!(game_state.inning_description(), "Bottom of the 1st");

        // Test extra innings
        game_state.current_inning = InningNumber::Extra(12);
        game_state.state = GameState::Inning(InningHalf::Top);
        assert_eq!(game_state.inning_description(), "Top of the 12th");
    }

    #[test]
    fn test_game_ending_conditions() {
        let mut game = Game::new();

        // Simulate game state at end of 9th inning
        game.current_inning = InningNumber::Ninth;
        game.state = GameState::Inning(InningHalf::Top); // Bottom 9th just completed
        game.score = GameScore::new().add_home_runs(5).add_away_runs(3);

        // Home team ahead, game should end
        game.complete_half_inning(0);
        assert!(game.should_end_game(0));

        // Test tie game - should not end
        game.score = GameScore::new().add_home_runs(3).add_away_runs(3);
        assert!(!game.should_end_game(0));

        // Test home team walk-off run
        game.state = GameState::Inning(InningHalf::Bottom);
        game.score = GameScore::new().add_home_runs(3).add_away_runs(3);
        assert!(game.should_end_game(1));

        game.current_inning = InningNumber::Eighth;
        assert!(!game.should_end_game(0))
    }
}
