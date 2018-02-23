use std::collections::HashMap;
use games::dice::{BetId, Ratio};
use super::{add, BetKind};

struct Simple18(u16, [u8; 18]);

impl Ratio for Simple18 {
    fn ratio(&self) -> f64 {
        1.0
    }
}

impl BetKind for Simple18 {
    fn bingo(&self, num: u8)->bool {
        self.1.iter().any(|&x| x == num)
    }
}

impl BetId for Simple18 {
    fn id(&self) -> u16 {
        self.0
    }
}

struct Simple12(u16, [u8; 12]);

impl Ratio for Simple12 {
    fn ratio(&self) -> f64 {
        2.0
    }
}

impl BetKind for Simple12 {
    fn bingo(&self, num: u8)->bool {
        self.1.iter().any(|&x| x == num)
    }
}

impl BetId for Simple12 {
    fn id(&self) -> u16 {
        self.0
    }
}

pub fn all_bets(map: &mut HashMap<u16, Box<BetKind>>) {
    //red
    add(Box::new(Simple18(1, [1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36])),map);
    //black
    add(Box::new(Simple18(2, [2, 4, 6, 8, 10, 11, 13, 15, 17, 20, 22, 24, 26, 28, 29, 31, 33, 35])),map);
    //odd
    add(Box::new(Simple18(3, [1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35])),map);
    //even
    add(Box::new(Simple18(4, [2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36])),map);
    //high
    add(Box::new(Simple18(5, [19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36])),map);
    //low
    add(Box::new(Simple18(6, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18])),map);
    
    //1st dozen
    add(Box::new(Simple12(7, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12])),map);
    //2nd dozen
    add(Box::new(Simple12(8, [13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24])),map);
    //3rd dozen
    add(Box::new(Simple12(9, [25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36])),map);
    //1st column
    add(Box::new(Simple12(10, [1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 31, 34])),map);
    //2nd column
    add(Box::new(Simple12(11, [2, 5, 8, 11, 14, 17, 20, 23, 26, 29, 32, 35])),map);
    //3rd column
    add(Box::new(Simple12(12, [3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36])),map);
}


