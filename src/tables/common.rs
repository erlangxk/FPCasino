use std::hash::Hash;
use std::collections::HashMap;

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
