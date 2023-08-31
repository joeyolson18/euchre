use std::{fmt, collections::VecDeque};
use rand::Rng;

const N_CARDS: usize = 24;
const N_PLAYERS: usize = 4;
const HAND_SIZE: usize = 5;

type CardPos = usize;
type Points = usize;

fn main() {
    let mut game = Game::new();
    print!("{}", game);
    game.play_round(0);
}

struct Card {
    value: char,
    suit: char,
    color: char,
}

struct Deck {
    cards: Vec<Card>
}
impl Deck {
    fn new() -> Self {
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

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        for card_index_1 in (1..N_CARDS).rev() {
            let card_index_2 = rng.gen_range(0..card_index_1);
            self.cards.swap(card_index_1, card_index_2);
        }
    }

    fn get_cards(&mut self, n: usize) -> Vec<Card> {
        let mut hand: Vec<Card> = Vec::new();

        for _i in 0..n {
            hand.push(self.cards.pop().unwrap());
        }
        return hand;
    }
}
impl fmt::Display for Deck {
    
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", print_cards(&self.cards))
    }
}

struct Player {
    hand: Vec<Card>,
    position: usize,
    team: usize,
}
impl Player {
    fn new(position: usize) -> Self {
        let hand: Vec<Card> = Vec::new();
        let team = position % 2;
        Player {
            hand,
            position: 0,
            team,
        }
    }

    fn deal_hand(&mut self, hand: Vec<Card>) {
        self.hand = hand;
    }

    fn play_turn(&mut self, prev_cards: Option<&Vec<Card>>) -> Card {
        let mut playable_cards: Vec<usize> = Vec::new();
        match prev_cards {
            Some(prev_card) => { 
                let lead_suit = prev_card[0].suit;
                let lead_color = prev_card[0].color;
                let mut shortsuited = true;
                for (card_pos, card) in self.hand.iter().enumerate() {
                    if card.suit == lead_suit 
                    || (card.value == 'J' && card.color == lead_color) {
                        playable_cards.push(card_pos);
                        if !shortsuited { shortsuited = true; }
                    } 
                }
                if shortsuited { 
                    for card_pos in 0..self.hand.len() {
                        playable_cards.push(card_pos);
                    }
                }
            }
            None => {
                for card_pos in 0..self.hand.len() {
                    playable_cards.push(card_pos);
                }
            }
        }
        // TODO allow player input on played card
        let played_card_index = playable_cards[0];
        self.hand.remove(played_card_index)
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", print_cards(&self.hand))
    }
}

struct Game {
    deck: Deck,
    players: VecDeque<Player>,
    prev_cards: Vec<Card>,
    discard: Vec<Card>,
    trump: Card,
    score: (usize, usize),
}
impl Game {
    fn new() -> Self {
        let mut deck = Deck::new();
        deck.shuffle();

        let mut players: VecDeque<Player> = VecDeque::new();
        for i in 0..N_PLAYERS {
            players.push_back(Player::new(i));
            players[i].deal_hand(deck.get_cards(HAND_SIZE));
        }
        let prev_cards = Vec::new();
        let trump = Card { value: '9', suit: '♠', color: 'b' };

        let discard = Vec::new();

        Game {
            deck,
            players,
            prev_cards,
            trump,
            discard, 
            score: (0, 0),
        }
    }
    fn play_round(&mut self, dealer_position: usize) {//-> Points {
        let mut first_player_position = dealer_position;
        let mut hands: [usize; 2] = [0, 0];
        for _i in 0..HAND_SIZE {
            
            // rotates the vector in accordance with turn order
            let position_difference = (first_player_position - self.players[0].position) as isize;

            if position_difference > 0 {
                if position_difference > 2 {
                    self.players.rotate_right((position_difference - 2) as usize);
                }
                self.players.rotate_left((position_difference) as usize);
            }
            else if position_difference < 0 {
                if position_difference < -2 {
                    self.players.rotate_left((position_difference.abs() - 2) as usize);
                }
                else {
                    self.players.rotate_right((position_difference.abs()) as usize);
                }
            }
            let winner = self.play_hand();
            first_player_position = winner.position;
            hands[winner.team] += 1 
        }
        print!("{}, {}", hands[0], hands[1]);
    }

    fn play_hand(&mut self) -> &Player {
        for player in &mut self.players {
            if self.prev_cards.len() == 0 {
                self.prev_cards.push(player.play_turn(None));
            }
            else {
                self.prev_cards.push(player.play_turn(Some(&self.prev_cards)));
            }
        }

        let mut highest_card_pos: CardPos = 0;
        let mut highest_card_rank: usize = 0;
        let lead_suit = self.prev_cards[0].suit;
        for (card_pos, card) in self.prev_cards.iter().enumerate() {
            let card_rank = return_card_rank(card, &self.trump, lead_suit);
            if card_rank > highest_card_rank { 
                highest_card_rank = card_rank;
                highest_card_pos = card_pos;
            }
        }
        let winner = &self.players[highest_card_pos];
        print!("{}", print_cards(&self.prev_cards));
        for card in &self.prev_cards {
            print!("{} ", return_card_rank(card, &self.trump, lead_suit))
        }
        print!("\n{}\n", winner.team);
        
        for _i in 0..N_PLAYERS {
            let temp_card = self.prev_cards.pop().unwrap();
            self.discard.push(temp_card)
        }

        return winner;
    }

}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output += print_card(&self.deck.cards[0]).as_str();

        for player in &self.players {
            output += print_cards(&player.hand).as_str();
        }
        
        write!(f, "{}", output)
    }
}

fn return_card_rank(card: &Card, trump: &Card, lead_suit: char) -> usize {
    let mut rank: usize = 1;
    if card.suit == trump.suit {
        if card.value == 'J' { return 13 }
        else { rank += 6; }  
    }
    else if card.value == 'J' && card.color == trump.color {
        return 12;
    }
    if card.suit == lead_suit
    || card.suit == trump.suit {
        match card.value {
            '9' => return rank,
            'T' => return rank + 1,
            'J' => return rank + 2,
            'Q' => return rank + 3,
            'K' => return rank + 4,
            'A' => return rank + 5,
            _   => return rank + 1000,
        }
    }
    else {
        return 0;
    }
        
    
}

fn print_cards(cards: &Vec<Card>) -> String {
    let mut output = String::new();
    for _card in cards {
        output += "┌──┐";
    }
    output.push('\n');
    for card in cards {
        output.push('│');
        output.push(card.value);
        output.push(card.suit);
        output.push('│');
    }
    output.push('\n');
    for _card in cards {
        output += "└──┘";
    }
    output.push('\n');

    return output;
}

fn print_card(card: &Card) -> String {
    let mut output = String::from("┌──┐\n");
    output.push('│');
    output.push(card.value);
    output.push(card.suit);
    output += "│\n└──┘\n";
    return output; 
}
fn print_card_and_rank(card: &Card, trump: &Card, lead_suit: char) -> String {
    let mut output = String::new();
    let rank = return_card_rank(card, trump, lead_suit);
    output += rank.to_string().as_str() ;
    output.push('\n');
    output += "┌──┐\n";
    output.push('│');
    output.push(card.value);
    output.push(card.suit);
    output += "│\n└──┘\n";
    return output; 
}

#[derive(Debug, Clone)]
struct NoCardsError;

impl fmt::Display for NoCardsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No Cards Played!")
    }
}