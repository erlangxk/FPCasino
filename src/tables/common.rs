use std::hash::Hash;
use std::collections::HashMap;
use games::Game;

pub struct Limit(f64, f64);

impl Limit {
    pub fn min(&self) -> f64 {
        self.0
    }
    pub fn max(&self) -> f64 {
        self.1
    }
}

#[derive(Debug)]
pub struct PlayerBet<T: Eq + Hash> {
    pub uuid: String,
    pub user_id: String,
    pub bets: HashMap<T, f64>,
}

impl<T: Eq + Hash> PlayerBet<T> {
    pub fn new(uuid: String, user_id: String, bets: HashMap<T, f64>) -> PlayerBet<T> {
        PlayerBet {
            uuid,
            user_id,
            bets,
        }
    }
}

pub struct Round {
    pub id: u64,
    pub hand: u8,
    pub start_time: u64,
    pub end_time: u64,
}

pub struct Table<G: Game> {
    pub id: u16,
    pub game: G,
    pub round: Round,
    pub current_bets: Vec<PlayerBet<G::B>>,
    pub previous_bets: Vec<PlayerBet<G::B>>,
}

impl<G: Game> Table<G> {
    pub fn bet(
        &mut self,
        _user_id: &str,
        _min_limit: f64,
        _max_limit: f64,
        _round_id: u64,
        bets: HashMap<u16, f64>,
    ) -> f64 {
        let _checked_bets = self.game.from_raw_bets(&bets);
        32.0
    }
}
