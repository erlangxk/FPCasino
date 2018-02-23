use super::{add, count, BetKind, Result};
use games::dice::{BetId, Ratio};
use std::collections::HashMap;

enum Simple {
    Big,
    Small,
    Odd,
    Even,
}

impl BetId for Simple {
    fn id(&self) -> u16 {
        match *self {
            Simple::Big => 1,
            Simple::Small => 2,
            Simple::Odd => 3,
            Simple::Even => 4,
        }
    }
}

impl Ratio for Simple {
    fn ratio(&self) -> f64 {
        1.0
    }
}

impl BetKind for Simple {
    fn bingo(&self, r: &Result) -> u8 {
        if !r.is_triple {
            match *self {
                Simple::Big => count(r.sum >= 11 && r.sum <= 17),
                Simple::Small => count(r.sum >= 4 && r.sum <= 10),
                Simple::Odd => count(r.sum % 2 != 0),
                Simple::Even => count(r.sum % 2 == 0),
            }
        } else {
            0
        }
    }
}

pub fn all_bets(map: &mut HashMap<u16, Box<BetKind>>) {
    add(Box::new(Simple::Big), map);
    add(Box::new(Simple::Small), map);
    add(Box::new(Simple::Odd), map);
    add(Box::new(Simple::Even), map);
}
