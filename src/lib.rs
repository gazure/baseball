#![deny(clippy::pedantic)]

mod baseball;
mod game;

pub use baseball::*;
pub use game::start::run;
