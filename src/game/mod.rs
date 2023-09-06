use std::fmt::Display;
use std::io::{self, BufRead};

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

pub enum Replace<T> {
    Yes(T),
    No(T),
}

fn read_input() -> io::Result<usize> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_line(&mut buffer)?;
    buffer.pop();
    buffer.pop();
    let input = buffer.parse::<usize>().unwrap();
    Ok(input)
}
