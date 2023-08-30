use crate::casino::cards::utils::*;
use crate::casino::poker::models::*;

pub struct GameOptions {
    pub buy_in: u32,
    pub big_blind: u32,
    pub small_blind: u32,
    pub max_players: u32
}

pub struct Game {
    players: Vec<Player>,
    dealer: u32,
    current_player: u32,
    
}