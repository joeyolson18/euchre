// Game options
// 'n' - new round
// 'g<winning team>' - game over

use std::{ fmt, collections::VecDeque };
use super::{ Card, Player, Deck, Replace, Call };
use super::{ Score };
use super::{ HAND_SIZE, N_PLAYERS };

pub struct Game {
    deck: Deck,
    players: VecDeque<Player>,
    call_team: Option<usize>,
    prev_cards: Vec<Card>,
    discard: Vec<Card>,
    trump: Option<Card>,
    out_player: Option<Player>,
    n_players: usize,
}

impl Game {
    pub fn new() -> Self {
        let mut players: VecDeque<Player> = VecDeque::new();
        for i in 0..N_PLAYERS {
            players.push_back(Player::new(i));
        }

        Game {
            deck: Deck::new(),
            players: players,
            call_team: None,
            prev_cards: Vec::new(),
            trump: None,
            discard: Vec::new(),
            out_player: None,
            n_players: N_PLAYERS,
        }
    }

    pub fn play_game(mut self) -> usize {
        let mut score: Score = [0, 0];
        let mut dealer_position: usize = 0;
        loop {
            self.deck.shuffle();

            for i in 0..self.n_players {
                self.players[i].deal_hand(self.deck.get_cards(HAND_SIZE));
            }

            print!("n\n{} {}\n", score[0], score[1]);

            self.bid();

            //print!("{}", self);
            
            self.play_round(&mut score);

            if score[0] >= 11 { 
                print!("g0");
                return 0;
            }
            if score[1] >= 11 { 
                print!("g1");
                return 1;
            }

            if dealer_position == self.n_players - 1 {
                dealer_position = 0;
            }
            else {
                dealer_position += 1;
            }
        }
    }

    fn go_alone(&mut self, position: usize) {
        let partner_position = if position <= 2 { position + 2 } else { position - 2 };
        self.out_player = Some(self.players.remove(partner_position).unwrap());
        self.n_players -= 1;
    }

    fn bid(&mut self) {

        // Bid with order card
        let bid_card = self.deck.pop();
        for i in 1..self.n_players {
            let order_up = self.players[i].bid(&bid_card);
            match order_up {
                Call::Yes(bid_card_clone) => {
                    self.call_team = Some(self.players[i].team);
                    self.trump = Some(bid_card_clone);
                    let replaced_card = self.players[0].replace(bid_card);
                    self.deck.push(replaced_card);
                    return;
                },
                Call::Alone(bid_card_clone) => {
                    self.call_team = Some(self.players[i].team);
                    self.trump = Some(bid_card_clone);
                    let replaced_card = self.players[0].replace(bid_card);
                    self.deck.push(replaced_card);
                    self.go_alone(self.players[i].position);
                    return;
                },
                Call::No => (),
            }
        }
        let dealer_bid = self.players[0].bid_dealer(bid_card);
        match dealer_bid {
            Replace::Yes(replaced_card) => {
                self.call_team = Some(self.players[0].team);
                self.trump = Some(bid_card.clone());
                self.deck.push(replaced_card);
                return;
            },
            Replace::Alone(replaced_card) => {
                self.call_team = Some(self.players[0].team);
                self.trump = Some(bid_card.clone());
                self.deck.push(replaced_card);
                self.go_alone(self.players[0].position);
                return;
            }
            Replace::No(bid_card) => {
                self.deck.push(bid_card);
            },
        }

        // Bid with order card turned
        let nullified_suit = self.deck.last().suit;
        let mut suit_options: Vec<String> = self.deck.get_suits().into_iter()
            .filter(|&suit| 
                suit != nullified_suit
            ).map(|suit| 
                suit.to_string()
            ).collect();
        self.deck.get_suits().into_iter()
            .filter(|&suit|
                suit != nullified_suit
            ).for_each(|suit|
                suit_options.push(suit.to_string() + "a")
            );
        suit_options.push('P'.to_string());
        for i in 1..self.n_players {
            let called_suit = self.players[i].call_suit(suit_options.clone());
            match called_suit {
                Call::Yes(card) => {
                    self.call_team = Some(self.players[i].team);
                    self.trump = Some(card);
                    return;
                },
                Call::Alone(card) => {
                    self.call_team = Some(self.players[i].team);
                    self.trump = Some(card);
                    self.go_alone(self.players[i].position);
                    return;
                },
                Call::No => (),
            } 
        }
        suit_options.pop();
        let called_suit = self.players[0].call_suit(suit_options);
        match called_suit {
            Call::Yes(card) => {
                self.call_team = Some(self.players[0].team);
                self.trump = Some(card);
                return;
            },
            Call::Alone(card) => {
                self.call_team = Some(self.players[0].team);
                self.trump = Some(card);
                self.go_alone(self.players[0].position);
                return;
            },
            Call::No => { panic!("Invalid Input - dealer cannot pass"); },
        }
    }

    fn play_round(&mut self, score: &mut Score) {
        let mut first_player_position = self.players[0].position;
        let mut hands: Score = [0, 0];
        for _i in 0..HAND_SIZE {
            
            // rotates the vector in accordance with turn order
            let position_difference = first_player_position as isize - self.players[0].position as isize;

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
            let winner = self.play_hand(&mut hands);
            first_player_position = winner.position;
        }
 
        let mut went_alone: bool = false;

        // if a player went alone
        if let Some(player) = &mut self.out_player {
            for _i in 0..player.hand.len() {
                self.deck.cards.push(player.hand.pop().unwrap());
            }
            self.n_players += 1;
            self.players.insert(player.position, player.clone());
            self.out_player = None;
            went_alone = true;
        }    

        for _i in 0..self.discard.len() {
            self.deck.cards.push(self.discard.pop().unwrap());
        }
        
        if hands[0] == 5 {
            score[0] += if went_alone { 4 } else { 2 }; 
        }
        else if hands[1] == 5 {
            score[1] += if went_alone { 4 } else { 2 };
        }
        else if hands[0] > hands[1] {
            if self.call_team == Some(0) { score[0] += 1; }
            else { score[0] += 2; }
        }
        else {
            if self.call_team == Some(1) { score[1] += 1; }
            else { score[1] += 2; }
            
        }
    }

    fn play_hand(&mut self, hands: &mut Score) -> &Player {
        for i in 1..self.n_players {
            if self.prev_cards.len() == 0 {
                print!("\n");
                self.prev_cards.push(self.players[i].play_turn(None, self.trump.as_ref().unwrap()));
            }
            else {
                print!("{}\n", Card::print_cards(&self.prev_cards));
                self.prev_cards.push(self.players[i].play_turn(Some(&self.prev_cards), self.trump.as_ref().unwrap()));
            }
        }
        if self.prev_cards.len() == 0 {
            print!("\n");
            self.prev_cards.push(self.players[0].play_turn(None, self.trump.as_ref().unwrap()));
        }
        else {
            print!("{}\n", Card::print_cards(&self.prev_cards));
            self.prev_cards.push(self.players[0].play_turn(Some(&self.prev_cards), self.trump.as_ref().unwrap()));
        }

        let mut highest_card_pos: usize = 0;
        let mut highest_card_rank: usize = 0;
        let lead_suit = self.prev_cards[0].suit;
        for (card_pos, card) in self.prev_cards.iter().enumerate() {
            let card_rank = card.return_rank(&self.trump.as_ref().unwrap(), lead_suit);
            if card_rank > highest_card_rank { 
                highest_card_rank = card_rank;
                highest_card_pos = card_pos;
            }
        }

        let winner = &self.players[highest_card_pos];
        hands[winner.team] += 1;

        //print!("{}", Card::print_hand(&self.prev_cards, self.players[0].position, winner.position, hands));

        for _i in 0..self.n_players {
            let temp_card = self.prev_cards.pop().unwrap();
            self.discard.push(temp_card)
        }
        
        return winner;
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::from("┌──┐");
        output += "Calling Team: ";
        output += &self.call_team.unwrap().to_string();
        output.push('\n');
        output.push('│');
        output.push(self.trump.as_ref().unwrap().value);
        output.push(self.trump.as_ref().unwrap().suit);
        output += "│\n└──┘\n";

        for player in &self.players {
            output += Card::print_cards_index(&player.hand, player.position).as_str();
        }
        
        write!(f, "{}", output)
    }
}