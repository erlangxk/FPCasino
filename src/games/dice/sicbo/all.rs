use std::collections::HashMap;
use super::{double, pair, simple, single, total, triple, BetKind, Result};

struct Sicbo {
    all_bets: HashMap<u16, Box<BetKind>>,
}

impl Sicbo {
    fn new() -> Sicbo {
        let mut map = HashMap::<u16, Box<BetKind>>::new();
        simple::all_bets(&mut map);
        single::all_bets(&mut map);
        double::all_bets(&mut map);
        triple::all_bets(&mut map);
        pair::all_bets(&mut map);
        total::all_bets(&mut map);
        Sicbo { all_bets: map }
    }

    fn valid_bets(&self, bet_id: u16) -> bool {
        self.all_bets.contains_key(&bet_id)
    }

    fn payout_map(&self, d1: u8, d2: u8, d3: u8) -> HashMap<u16, f64> {
        let r = Result::new(d1, d2, d3);
        let mut map = HashMap::<u16, f64>::new();
        for (&id, b) in &self.all_bets {
            let c = b.bingo(&r);
            if c > 0 {
                map.insert(id, b.ratio() * (c as f64) + 1.0);
            }
        }
        map
    }
}
