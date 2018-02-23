use super::{add, count, BetKind, Result};
use games::dice::{BetId, Ratio};
use std::collections::HashMap;

struct Pair(u16, u8, u8);

impl BetId for Pair {
    fn id(&self) -> u16 {
        self.0
    }
}

impl Ratio for Pair {
    fn ratio(&self) -> f64 {
        5.0
    }
}

impl BetKind for Pair {
    fn bingo(&self, r: &Result) -> u8 {
        let r1 = self.1 == r.d1 || self.1 == r.d2 || self.1 == r.d3;
        let r2 = self.2 == r.d1 || self.2 == r.d2 || self.2 == r.d3;
        count(r1 && r2)
    }
}

pub fn all_bets(map: &mut HashMap<u16, Box<BetKind>>) {
    add(Box::new(Pair(212, 1, 2)), map);
    add(Box::new(Pair(213, 1, 3)), map);
    add(Box::new(Pair(214, 1, 4)), map);
    add(Box::new(Pair(215, 1, 5)), map);
    add(Box::new(Pair(216, 1, 6)), map);
    add(Box::new(Pair(223, 2, 3)), map);
    add(Box::new(Pair(224, 2, 4)), map);
    add(Box::new(Pair(225, 2, 5)), map);
    add(Box::new(Pair(226, 2, 6)), map);
    add(Box::new(Pair(234, 3, 4)), map);
    add(Box::new(Pair(235, 3, 5)), map);
    add(Box::new(Pair(236, 3, 6)), map);
    add(Box::new(Pair(245, 4, 5)), map);
    add(Box::new(Pair(246, 4, 6)), map);
    add(Box::new(Pair(256, 5, 6)), map);
}
