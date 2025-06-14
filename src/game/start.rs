use bevy::prelude::*;

use crate::game::plugins::baseball::BaseballPlugin;

/// Runs the game.
pub fn run() {
    App::new().add_plugins(BaseballPlugin).run();
}
