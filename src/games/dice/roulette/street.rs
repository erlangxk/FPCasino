use std::collections::HashMap;
use games::dice::{BetId, Ratio};
use super::{add, BetKind};

#[derive(Clone, Copy)]
struct Street(u16, u8, u8, u8);

impl BetKind for Street {
    fn bingo(&self, num: u8) -> bool {
        self.1 == num || self.2 == num || self.3 == num
    }
}

impl Ratio for Street {
    fn ratio(&self) -> f64 {
        11.0
    }
}

impl BetId for Street {
    fn id(&self) -> u16 {
        self.0
    }
}

pub fn all_bets(map: &mut HashMap<u16, Box<BetKind>>) {
    add(Box::new(Street(300, 0, 1, 2)), map);
    add(Box::new(Street(301, 0, 2, 3)), map);
    add(Box::new(Street(302, 1, 2, 3)), map);
    add(Box::new(Street(303, 4, 5, 6)), map);
    add(Box::new(Street(304, 7, 8, 9)), map);
    add(Box::new(Street(305, 10, 11, 12)), map);
    add(Box::new(Street(306, 13, 14, 15)), map);
    add(Box::new(Street(307, 16, 17, 18)), map);
    add(Box::new(Street(308, 19, 20, 21)), map);
    add(Box::new(Street(309, 22, 23, 24)), map);
    add(Box::new(Street(310, 25, 26, 27)), map);
    add(Box::new(Street(311, 28, 29, 30)), map);
    add(Box::new(Street(312, 31, 32, 33)), map);
    add(Box::new(Street(313, 34, 35, 36)), map);
}
