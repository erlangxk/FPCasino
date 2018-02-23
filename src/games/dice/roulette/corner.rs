use std::collections::HashMap;
use games::dice::{BetId, Ratio};
use super::{add, BetKind};

#[derive(Clone, Copy)]
struct Corner(u16, [u8; 4]);

impl BetKind for Corner {
    fn bingo(&self, num: u8) -> bool {
        self.1.iter().any(|&x| x == num)
    }
}

impl Ratio for Corner {
    fn ratio(&self) -> f64 {
        8.0
    }
}

impl BetId for Corner {
    fn id(&self) -> u16 {
        self.0
    }
}

pub fn all_bets(map: &mut HashMap<u16, Box<BetKind>>) {
    add(Box::new(Corner(400, [1, 2, 4, 5])), map);
    add(Box::new(Corner(401, [2, 3, 5, 6])), map);
    add(Box::new(Corner(402, [4, 5, 7, 8])), map);
    add(Box::new(Corner(403, [5, 6, 8, 9])), map);
    add(Box::new(Corner(404, [7, 8, 10, 11])), map);
    add(Box::new(Corner(405, [8, 9, 11, 12])), map);
    add(Box::new(Corner(406, [10, 11, 13, 14])), map);
    add(Box::new(Corner(407, [11, 12, 14, 15])), map);
    add(Box::new(Corner(408, [13, 14, 16, 17])), map);
    add(Box::new(Corner(409, [14, 15, 17, 18])), map);
    add(Box::new(Corner(410, [16, 17, 19, 20])), map);
    add(Box::new(Corner(411, [17, 18, 20, 21])), map);
    add(Box::new(Corner(412, [19, 20, 22, 23])), map);
    add(Box::new(Corner(413, [20, 21, 23, 24])), map);
    add(Box::new(Corner(414, [22, 23, 25, 26])), map);
    add(Box::new(Corner(415, [23, 24, 26, 27])), map);
    add(Box::new(Corner(416, [25, 26, 28, 29])), map);
    add(Box::new(Corner(417, [26, 27, 29, 30])), map);
    add(Box::new(Corner(418, [28, 29, 31, 32])), map);
    add(Box::new(Corner(419, [29, 30, 32, 33])), map);
    add(Box::new(Corner(420, [31, 32, 34, 35])), map);
    add(Box::new(Corner(421, [32, 33, 35, 36])), map);
    add(Box::new(Corner(422, [0, 1, 2, 3])), map);
}
