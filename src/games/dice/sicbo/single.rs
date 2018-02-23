use super::{add, count, BetKind, Result};
use games::dice::{BetId, Ratio};
use std::collections::HashMap;

struct Single(u16, u8);

impl BetId for Single {
    fn id(&self) -> u16 {
        self.0
    }
}

impl Ratio for Single {
    fn ratio(&self) -> f64 {
        1.0
    }
}

impl BetKind for Single {
    fn bingo(&self, r: &Result) -> u8 {
        count(r.d1 == self.1) + count(r.d2 == self.1) + count(r.d3 == self.1)
    }
}

pub fn all_bets(map: &mut HashMap<u16, Box<BetKind>>) {
    add(Box::new(Single(11, 1)), map);
    add(Box::new(Single(12, 2)), map);
    add(Box::new(Single(13, 3)), map);
    add(Box::new(Single(14, 4)), map);
    add(Box::new(Single(15, 5)), map);
    add(Box::new(Single(16, 6)), map);
}
