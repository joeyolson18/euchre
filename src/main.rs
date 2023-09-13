use std::{fmt};
mod game;
use game::Game;

fn main() {
    Game::new().play_game();
}

#[derive(Debug, Clone)]
struct NoCardsError;

impl fmt::Display for NoCardsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No Cards Played!")
    }
}