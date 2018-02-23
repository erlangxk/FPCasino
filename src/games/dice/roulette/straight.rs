use std::collections::HashMap;
use super::{add, BetKind};
use games::dice::{Ratio,BetId};

#[derive(Clone,Copy)]
struct Straight(u16, u8);

impl BetKind for Straight {
    fn bingo(&self, num: u8) -> bool {
        self.1 == num
    }
}

impl Ratio for Straight {
    fn ratio(&self) -> f64 {
        35.0
    }
}

impl BetId for Straight {
    fn id(&self) -> u16 {
        self.0
    }
}

pub fn all_bets(map: &mut HashMap<u16, Box<BetKind>>) {
    add(Box::new(Straight(100, 0)), map);
    add(Box::new(Straight(101, 1)), map);
    add(Box::new(Straight(102, 2)), map);
    add(Box::new(Straight(103, 3)), map);
    add(Box::new(Straight(104, 4)), map);
    add(Box::new(Straight(105, 5)), map);
    add(Box::new(Straight(106, 6)), map);
    add(Box::new(Straight(107, 7)), map);
    add(Box::new(Straight(108, 8)), map);
    add(Box::new(Straight(109, 9)), map);
    add(Box::new(Straight(110, 10)), map);
    add(Box::new(Straight(111, 11)), map);
    add(Box::new(Straight(112, 12)), map);
    add(Box::new(Straight(113, 13)), map);
    add(Box::new(Straight(114, 14)), map);
    add(Box::new(Straight(115, 15)), map);
    add(Box::new(Straight(116, 16)), map);
    add(Box::new(Straight(117, 17)), map);
    add(Box::new(Straight(118, 18)), map);
    add(Box::new(Straight(119, 19)), map);
    add(Box::new(Straight(120, 20)), map);
    add(Box::new(Straight(121, 21)), map);
    add(Box::new(Straight(122, 22)), map);
    add(Box::new(Straight(123, 23)), map);
    add(Box::new(Straight(124, 24)), map);
    add(Box::new(Straight(125, 25)), map);
    add(Box::new(Straight(126, 26)), map);
    add(Box::new(Straight(127, 27)), map);
    add(Box::new(Straight(128, 28)), map);
    add(Box::new(Straight(129, 29)), map);
    add(Box::new(Straight(130, 30)), map);
    add(Box::new(Straight(131, 31)), map);
    add(Box::new(Straight(132, 32)), map);
    add(Box::new(Straight(133, 33)), map);
    add(Box::new(Straight(134, 34)), map);
    add(Box::new(Straight(135, 35)), map);
    add(Box::new(Straight(136, 36)), map);
}
