use std::collections::HashMap;
use games::dice::{Ratio,BetId};
use super::{add, BetKind};

#[derive(Clone,Copy)]
struct Split(u16, u8, u8);

impl BetKind for Split {
    fn bingo(&self, num: u8) -> bool {
        self.1 == num || self.2 == num
    }
}

impl Ratio for Split {
    fn ratio(&self) -> f64 {
        17.0
    }
}

impl BetId for Split {
    fn id(&self) -> u16 {
        self.0
    }
}

pub fn all_bets(map: &mut HashMap<u16, Box<BetKind>>){
    add(Box::new(Split(200, 0, 1)), map);
    add(Box::new(Split(201, 0, 2)), map);
    add(Box::new(Split(202, 0, 3)), map);
    add(Box::new(Split(203, 1, 4)), map);
    add(Box::new(Split(204, 2, 5)), map);
    add(Box::new(Split(205, 3, 6)), map);
    add(Box::new(Split(206, 4, 7)), map);
    add(Box::new(Split(207, 5, 8)), map);
    add(Box::new(Split(208, 6, 9)), map);
    add(Box::new(Split(209, 7, 10)), map);
    add(Box::new(Split(210, 8, 11)), map);
    add(Box::new(Split(211, 9, 12)), map);
    add(Box::new(Split(212, 10, 13)), map);
    add(Box::new(Split(213, 11, 14)), map);
    add(Box::new(Split(214, 12, 15)), map);
    add(Box::new(Split(215, 13, 16)), map);
    add(Box::new(Split(216, 14, 17)), map);
    add(Box::new(Split(217, 15, 18)), map);
    add(Box::new(Split(218, 16, 19)), map);
    add(Box::new(Split(219, 17, 20)), map);
    add(Box::new(Split(220, 18, 21)), map);
    add(Box::new(Split(221, 19, 22)), map);
    add(Box::new(Split(222, 20, 23)), map);
    add(Box::new(Split(223, 21, 24)), map);
    add(Box::new(Split(224, 22, 25)), map);
    add(Box::new(Split(225, 23, 26)), map);
    add(Box::new(Split(226, 24, 27)), map);
    add(Box::new(Split(227, 25, 28)), map);
    add(Box::new(Split(228, 26, 29)), map);
    add(Box::new(Split(229, 27, 30)), map);
    add(Box::new(Split(230, 28, 31)), map);
    add(Box::new(Split(231, 29, 32)), map);
    add(Box::new(Split(232, 30, 33)), map);
    add(Box::new(Split(233, 31, 34)), map);
    add(Box::new(Split(234, 32, 35)), map);
    add(Box::new(Split(235, 33, 36)), map);
    add(Box::new(Split(236, 1, 2)), map);
    add(Box::new(Split(237, 2, 3)), map);
    add(Box::new(Split(238, 4, 5)), map);
    add(Box::new(Split(239, 5, 6)), map);
    add(Box::new(Split(240, 7, 8)), map);
    add(Box::new(Split(241, 8, 9)), map);
    add(Box::new(Split(242, 10, 11)), map);
    add(Box::new(Split(243, 11, 12)), map);
    add(Box::new(Split(244, 13, 14)), map);
    add(Box::new(Split(245, 14, 15)), map);
    add(Box::new(Split(246, 16, 17)), map);
    add(Box::new(Split(247, 17, 18)), map);
    add(Box::new(Split(248, 19, 20)), map);
    add(Box::new(Split(249, 20, 21)), map);
    add(Box::new(Split(250, 22, 23)), map);
    add(Box::new(Split(251, 23, 24)), map);
    add(Box::new(Split(252, 25, 26)), map);
    add(Box::new(Split(253, 26, 27)), map);
    add(Box::new(Split(254, 28, 29)), map);
    add(Box::new(Split(255, 29, 30)), map);
    add(Box::new(Split(256, 31, 32)), map);
    add(Box::new(Split(257, 32, 33)), map);
    add(Box::new(Split(258, 34, 35)), map);
    add(Box::new(Split(259, 35, 36)), map);
}
