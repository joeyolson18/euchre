const N_CARDS: usize = 24;
const N_PLAYERS: usize = 4;
const HAND_SIZE: usize = 5;

type Score = [usize; 2];

pub use self::deck::Deck;
mod deck;

pub use self::game::Game;
mod game;

pub use self::player::Player;
mod player;

pub use self::card::Card;
mod card;