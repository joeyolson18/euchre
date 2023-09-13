#[derive(Clone, Copy)]
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

    pub fn return_option(&self) -> String {
        let mut output = String::from(self.value.clone());
        output.push(self.suit.clone());
        return output;
    }

    pub fn print_cards(cards: &Vec<Card>) -> String {
        let mut output = String::new();
        for card in cards {
            output += &card.return_option();
            output.push(' ');
        }
        return output;
    }

    pub fn print_cards_index(cards: &Vec<Card>, player_index: usize) -> String {
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
}


