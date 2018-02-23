use super::{add, count, BetKind, Result};
use games::dice::{BetId, Ratio};
use std::collections::HashMap;

struct Triple(u16, u8);

impl BetId for Triple {
    fn id(&self) -> u16 {
        self.0
    }
}

impl Ratio for Triple {
    fn ratio(&self) -> f64 {
        150.0
    }
}

impl BetKind for Triple {
    fn bingo(&self, r: &Result) -> u8 {
        count(r.is_triple && r.d1 == self.1)
    }
}

struct AnyTriple;

impl BetId for AnyTriple {
    fn id(&self) -> u16 {
        37
    }
}

impl Ratio for AnyTriple {
    fn ratio(&self) -> f64 {
        24.0
    }
}

impl BetKind for AnyTriple {
    fn bingo(&self, r: &Result) -> u8 {
        count(r.is_triple)
    }
}

pub fn all_bets(map: &mut HashMap<u16, Box<BetKind>>) {
    add(Box::new(Triple(31, 1)), map);
    add(Box::new(Triple(32, 2)), map);
    add(Box::new(Triple(33, 3)), map);
    add(Box::new(Triple(34, 4)), map);
    add(Box::new(Triple(35, 5)), map);
    add(Box::new(Triple(36, 6)), map);
    add(Box::new(AnyTriple), map);
}
