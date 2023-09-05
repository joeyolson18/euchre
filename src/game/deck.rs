use rand::Rng;
use super::Card;
use super::N_CARDS;

pub struct Deck {
    pub cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        let suits = ['♣', '♡', '♠', '♢'];
        let values = ['9', 'T', 'J', 'Q', 'K', 'A'];
        let mut color: char;
        for suit in suits {
            if suit == '♣' || suit == '♠' { color = 'b'; }
            else { color = 'r'; }
            for value in values {
                cards.push(Card{ value, suit, color });
            }
        }
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        for card_index_1 in (1..N_CARDS).rev() {
            let card_index_2 = rng.gen_range(0..card_index_1);
            self.cards.swap(card_index_1, card_index_2);
        }
    }

    pub fn get_cards(&mut self, n: usize) -> Vec<Card> {
        let mut hand: Vec<Card> = Vec::new();

        for _i in 0..n {
            hand.push(self.cards.pop().unwrap());
        }
        return hand;
    }
}