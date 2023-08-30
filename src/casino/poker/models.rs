use crate::casino::cards::utils::*;
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum PlayerState {
    Out,
    In,
    ToCall,
    AllIn
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Player {
    pub hand: [Card; 2],
    pub state: PlayerState,
    pub chips: u32
}

pub struct Pot {
    pub amount: u32,
    pub players: HashMap<Player, u32>,
    pub raise : u32
}

impl Pot {
    pub fn new() -> Self {
        Pot {
            amount: 0,
            players: HashMap::new(),
            raise: 0
        }
    }

    pub fn get_chips_in_pot(&self, player: &Player) -> u32 {
        *self.players.get(player).unwrap_or(&0)
    }

    pub fn chips_to_call(&self, player: &Player) -> u32 {
        self.raise - *self.players.get(player).unwrap_or(&0)
    }

    pub fn player_bet(&mut self, player: Player, chips: u32) {
        let total_amount = self.get_chips_in_pot(&player) + chips;

        self.players.insert(player, total_amount);

        if total_amount > self.raise {
            self.raise = total_amount;
        }
    }

    pub fn collect_bets(&mut self) {
        let mut new_players: HashMap<Player, u32> = HashMap::new();

        for (player, chips) in self.players.iter_mut() {
            self.amount += *chips;
            new_players.insert(player.clone(), 0);
        }

        self.players = new_players;
    }

    pub fn remove_player(&mut self, player: Player) {
        if self.players.contains_key(&player) {
            self.amount = self.players.remove(&player).unwrap();
        }
    }
}