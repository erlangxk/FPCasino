use super::{add, count, BetKind, Result};
use games::dice::{BetId, Ratio};
use std::collections::HashMap;

struct Total(u16, u8);

impl BetId for Total {
    fn id(&self) -> u16 {
        self.0
    }
}

impl Ratio for Total {
    fn ratio(&self) -> f64 {
        match self.1 {
            4 | 17 => 50.0,
            5 | 16 => 18.0,
            6 | 15 => 14.0,
            7 | 14 => 12.0,
            8 | 13 => 8.0,
            9 | 10 | 11 | 12 => 6.0,
            _ => unreachable!(),
        }
    }
}

impl BetKind for Total {
    fn bingo(&self, r: &Result) -> u8 {
        count(self.1 == r.sum)
    }
}

pub fn all_bets(map: &mut HashMap<u16, Box<BetKind>>) {
    add(Box::new(Total(104, 4)), map);
    add(Box::new(Total(105, 5)), map);
    add(Box::new(Total(106, 6)), map);
    add(Box::new(Total(107, 7)), map);
    add(Box::new(Total(108, 8)), map);
    add(Box::new(Total(109, 9)), map);
    add(Box::new(Total(110, 10)), map);
    add(Box::new(Total(111, 11)), map);
    add(Box::new(Total(112, 12)), map);
    add(Box::new(Total(113, 13)), map);
    add(Box::new(Total(114, 14)), map);
    add(Box::new(Total(115, 15)), map);
    add(Box::new(Total(116, 16)), map);
    add(Box::new(Total(117, 17)), map);
}
