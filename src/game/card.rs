use super::N_PLAYERS;

#[derive(Clone)]
pub struct Card {
    pub value: char,
    pub suit: char,
    pub color: char,
}

impl Card {
    pub fn return_rank(&self, trump: &Card, lead_suit: char) -> usize {
        let mut rank: usize = 1;
        if self.suit == trump.suit {
            if self.value == 'J' { return 13 }
            else { rank += 6; }  
        }
        else if self.value == 'J' && self.color == trump.color {
            return 12;
        }
        if self.suit == lead_suit
        || self.suit == trump.suit {
            match self.value {
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

    pub fn print_cards(cards: &Vec<Card>, player_index: usize) -> String {
        let mut output = String::new();
        output += "Player ";
        output += &player_index.to_string();
        output += " Hand ";
        for _card in cards {
            output += "┌──┐";
        }
        output += "\n              ";
        for card in cards {
            output.push('│');
            output.push(card.value);
            output.push(card.suit);
            output.push('│');
        }
        output += "\n              ";
        for _card in cards {
            output += "└──┘";
        }
        output.push('\n');
    
        return output;
    }

    pub fn print_card(card: &Card) -> String {
        let mut output = String::from("┌──┐\n");
        output.push('│');
        output.push(card.value);
        output.push(card.suit);
        output += "│\n└──┘\n";
        return output; 
    }

    pub fn print_hand(cards: &Vec<Card>, lead_player_index: usize, winner_index: usize, hands: &[usize; 2]) -> String {
        let lead_player = lead_player_index;
        let mut output = String::new();

        for i in lead_player..N_PLAYERS {
            output += &i.to_string();
            output += "┌──┐ ";
        }
        for i in 0..lead_player {
            output += &i.to_string();
            output += "┌──┐ ";
        }
        output += "Winner: Player ";
        output += &winner_index.to_string();
        output.push('\n');
        for card in cards {
            output += " │";
            output.push(card.value);
            output.push(card.suit);
            output += "│ ";
        }
        output += "Hands:  ";
        output += &hands[0].to_string();
        output += " | ";
        output += &hands[1].to_string();
        output.push('\n');
        for _card in cards {
            output += " └──┘ ";
        }
        output.push('\n');

        return output;
    }

    pub fn print_card_and_rank(card: &Card, trump: &Card, lead_suit: char) -> String {
        let mut output = String::new();
        let rank = card.return_rank(trump, lead_suit);
        output += rank.to_string().as_str() ;
        output.push('\n');
        output += "┌──┐\n";
        output.push('│');
        output.push(card.value);
        output.push(card.suit);
        output += "│\n└──┘\n";
        return output; 
    }
}


