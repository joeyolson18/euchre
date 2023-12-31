// Player Options
// '<player>c' - play a card
// '<player>o' - order up card displayed to dealer
// '<player>d' - order up card to self (dealer)
// '<player>t' - call up trump suit

use super::{ Card, Replace, Call };
use super::{ read_input };

#[derive(Clone)]
pub struct Player {
    pub hand: Vec<Card>,
    pub position: usize,
    pub team: usize,
}

impl Player {
    pub fn new(position: usize) -> Self {
        let hand: Vec<Card> = Vec::new();
        let team = position % 2;
        Player {
            hand,
            position,
            team,
        }
    }

    pub fn deal_hand(&mut self, hand: Vec<Card>) {
        self.hand = hand;
    }

    pub fn bid(&self, bid_card: &Card) -> Call<Card> {
        let card_option = bid_card.return_option();
        let card_option_alone = card_option.clone() + "a";
        let bid_card_clone = bid_card.clone();
        
        let bid_card_options = vec![card_option, card_option_alone, 'P'.to_string()];
        self.print_options('o', &bid_card_options);
        let input = read_input();

        match input {
            Ok(i) => {
                match i {
                    0 => { return Call::Yes(bid_card_clone); },
                    1 => { return Call::Alone(bid_card_clone); },
                    2 => { return Call::No; },
                    _ => { panic!("Invalid Input for fn bid"); },
                }
            },
            Err(e) => { panic!("{}", e); },
        }
    }

    pub fn bid_dealer(&mut self, bid_card: Card) -> Replace<Card> {
        let bid_card_options = bid_card.return_option();
        let mut replace_options: Vec<String> = self.hand.iter().map(|card| 
            card.return_option() + &bid_card_options
        ).collect();
        self.hand.iter().for_each(|card|
            replace_options.push(card.return_option() + &bid_card_options + "a")
        );
        replace_options.push('P'.to_string());
        self.print_options('d', &replace_options);
        let input = read_input();

        match input {
            Ok(i) => {
                match i {
                    0..=4 => {
                        let replaced_card = self.hand[i].clone(); 
                        self.hand[i] = bid_card;
                        Replace::Yes(replaced_card)
                    },
                    5..=9 => {
                        let replaced_card = self.hand[i - 5].clone(); 
                        self.hand[i - 5] = bid_card;
                        Replace::Alone(replaced_card)  
                    }
                    10 => { return Replace::No(bid_card); }
                    _ => { panic!("Invalid Input for fn replace"); },
                }

            },
            Err(e) => { panic!("{}", e); },
        }
    }

    pub fn replace(&mut self, ordered_card: Card) -> Card {
        let ordered_card_option = ordered_card.return_option();
        let replace_options = self.hand.iter().map(|card| 
                card.return_option() + " " + &ordered_card_option
            ).collect();
        self.print_options('r', &replace_options);
        let input = read_input();

        match input {
            Ok(i) => {
                match i {
                    0..=4 => {
                        let replaced_card = self.hand[i].clone(); 
                        self.hand[i] = ordered_card;
                        return replaced_card;
                    },
                    _ => { panic!("Invalid Input for fn replace"); },
                }

            },
            Err(e) => { panic!("{}", e); },
        }
    }

    pub fn call_suit(&mut self, suit_options: Vec<String>) -> Call<Card> {

        self.print_options('t', &suit_options);
        let input = read_input();

        match input {
            Ok(i) => {
                match i {
                    0..=2 => {
                        let suit_chars: Vec<char> = suit_options[i].chars().collect();
                        let suit = suit_chars[0];
                        return Call::Yes(Card{
                            value: ' ',
                            suit,
                            color: if suit == '♣' || suit == '♠' { 'b' }
                            else { 'r' }
                        })
                    },
                    3..=5 => {
                        let suit_chars: Vec<char> = suit_options[i].chars().collect();
                        let suit = suit_chars[0];
                        return Call::Alone(Card{
                            value: ' ',
                            suit,
                            color: if suit == '♣' || suit == '♠' { 'b' }
                            else { 'r' }
                        })
                    }
                    6 => { return Call::No; },
                    _ => { panic!("Invalid Input for fn call_suit"); },
                }
            },
            Err(e) => { panic!("{}", e); },
        }
    }

    pub fn play_turn(&mut self, prev_cards: Option<&Vec<Card>>, trump: &Card) -> Card {
        let mut playable_cards: Vec<usize> = Vec::new();
        match prev_cards {
            Some(prev_cards_unwrapped) => { 
                let prev_card = &prev_cards_unwrapped[0];
                let lead_suit = 
                    if prev_card.value == 'J' 
                    && prev_card.color == trump.color {
                        trump.suit
                    }
                    else {
                        prev_card.suit
                    };
                let lead_color = prev_card.color;

                let mut shortsuited = true;
                for (card_pos, card) in self.hand.iter().enumerate() {
                    if card.suit == lead_suit 
                    || (card.value == 'J' && card.color == lead_color) {
                        playable_cards.push(card_pos);
                        if shortsuited { shortsuited = false; }
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
        let card_options = playable_cards.iter().map(|&card_pos|
            self.hand[card_pos].return_option()
        ).collect();
        self.print_options('c', &card_options);
        let input = read_input();
        let played_card_index = match input {
            Ok(i) => {
                if i < self.hand.len() { playable_cards[i] }
                else { panic!("Invalid Input for fn play_turn"); }
            }
            Err(e) => { panic!("{}", e); },
        };
        return self.hand.remove(played_card_index);
    }   

    pub fn print_options(&self, option_type: char, options: &Vec<String>) {
        let mut output = String::new();
        output += &self.position.to_string();
        output.push(option_type);
        output.push('\n');
        for card in &self.hand {
            output += &card.return_option();
            output.push(' ');
        }
        output.push('\n');
        for (i, option) in options.iter().enumerate() { 
            output += &i.to_string();
            output.push('|');
            output += option;
            output.push('\n');
        }
        print!("{}", output)
    }
}

// impl fmt::Display for Player {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", Card::print_cards(&self.hand))
//     }
// }