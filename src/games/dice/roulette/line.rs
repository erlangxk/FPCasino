use std::collections::HashMap;
use games::dice::{BetId, Ratio};
use super::{add, BetKind};

#[derive(Clone, Copy)]
struct Line(u16, [u8;6]);

impl BetKind for Line {
    fn bingo(&self, num: u8) -> bool {
        self.1.iter().any(|&x| x == num)
    }
}

impl Ratio for Line {
    fn ratio(&self) -> f64 {
        5.0
    }
}

impl BetId for Line {
    fn id(&self) -> u16 {
        self.0
    }
}

pub fn all_bets(map: &mut HashMap<u16, Box<BetKind>>) {
    add(Box::new(Line(600, [1, 2, 3, 4, 5, 6])), map);
    add(Box::new(Line(601, [4, 5, 6, 7, 8, 9])), map);
    add(Box::new(Line(602, [7, 8, 9, 10, 11, 12])), map);
    add(Box::new(Line(603, [10, 11, 12, 13, 14, 15])), map);
    add(Box::new(Line(604, [13, 14, 15, 16, 17, 18])), map);
    add(Box::new(Line(605, [16, 17, 18, 19, 20, 21])), map);
    add(Box::new(Line(606, [19, 20, 21, 22, 23, 24])), map);
    add(Box::new(Line(607, [22, 23, 24, 25, 26, 27])), map);
    add(Box::new(Line(608, [25, 26, 27, 28, 29, 30])), map);
    add(Box::new(Line(609, [28, 29, 30, 31, 32, 33])), map);
    add(Box::new(Line(610, [31, 32, 33, 34, 35, 36])), map);
}
