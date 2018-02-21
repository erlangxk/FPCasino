use std::hash::Hash;
use std::collections::HashMap;
use games::{BetSerde, Game};

struct Limit(f64, f64);

impl Limit {
    fn min(&self) -> f64 {
        self.0
    }
    fn max(&self) -> f64 {
        self.1
    }
}


#[derive(Debug)]
struct PlayerBet<T: Eq + Hash> {
    uuid: String,
    user_id: String,
    bets: HashMap<T, f64>,
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

struct Round {
    id: u64,
    hand: u8,
    start_time: u64,
    end_time: u64,
}


struct Table<G: Game> {
    id: u16,
    game: G,
    round: Round,
    current_bets: Vec<PlayerBet<G::B>>,
    previous_bets: Vec<PlayerBet<G::B>>,
}

impl<G: Game> Table<G> {
    pub fn bet(
        &mut self,
        user_id: &str,
        min_limit: f64,
        max_limit: f64,
        round_id: u64,
        bets: HashMap<u16, f64>,
    ) -> f64 {
        let checked_bets = self.game.from_raw_bets(&bets);
        32.0
    }
}
