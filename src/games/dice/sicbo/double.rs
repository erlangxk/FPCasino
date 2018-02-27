use super::{add, count, BetKind, Result};
use games::dice::{BetId, Ratio};
use std::collections::HashMap;

struct Double(u16, u8);

impl BetId for Double {
    fn id(&self) -> u16 {
        self.0
    }
}

impl Ratio for Double {
    fn ratio(&self) -> f64 {
        8.0
    }
}

impl BetKind for Double {
    fn bingo(&self, r: &Result) -> u8 {
        let r1 = r.d1 == self.1;
        let r2 = r.d2 == self.1;
        let r3 = r.d3 == self.1;
        count((r1 && r2) || (r2 && r3) || (r1 && r3))
    }
}

pub fn all_bets(map: &mut HashMap<u16, Box<BetKind>>) {
    add(Box::new(Double(21, 1)), map);
    add(Box::new(Double(22, 2)), map);
    add(Box::new(Double(23, 3)), map);
    add(Box::new(Double(24, 4)), map);
    add(Box::new(Double(25, 5)), map);
    add(Box::new(Double(26, 6)), map);
}
