use super::Card;

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

    // TODO add actual jack functionality
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

// impl fmt::Display for Player {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", Card::print_cards(&self.hand))
//     }
// }