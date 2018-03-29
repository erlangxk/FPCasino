use super::{add, count, BetKind, Result};
use games::dice::{BetId, Ratio};
use std::collections::HashMap;

struct Single(u16, u8);

impl BetId for Single {
    fn id(&self) -> u16 {
        self.0
    }
}

impl Ratio for Single {
    fn ratio(&self) -> f64 {
        1.0
    }
}

impl BetKind for Single {
    fn bingo(&self, r: &Result) -> u8 {
        count(r.d1 == self.1) + count(r.d2 == self.1) + count(r.d3 == self.1)
    }
}

pub fn all_bets(map: &mut HashMap<u16, Box<BetKind>>) {
    add(Box::new(Single(11, 1)), map);
    add(Box::new(Single(12, 2)), map);
    add(Box::new(Single(13, 3)), map);
    add(Box::new(Single(14, 4)), map);
    add(Box::new(Single(15, 5)), map);
    add(Box::new(Single(16, 6)), map);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bingo() {
        let r = Result::new(1, 1, 1);
        assert_eq!(Single(11,1).bingo(&r), 3);
        
        let r = Result::new(1, 1, 2);
        assert_eq!(Single(11,1).bingo(&r), 2);
        assert_eq!(Single(12,2).bingo(&r), 1);
        
        let r = Result::new(1, 2, 3);
        assert_eq!(Single(11,1).bingo(&r), 1);
        assert_eq!(Single(12,2).bingo(&r), 1);
        assert_eq!(Single(13,3).bingo(&r), 1);
        
        let r = Result::new(4, 5, 6);
        assert_eq!(Single(14,4).bingo(&r), 1);
        assert_eq!(Single(15,5).bingo(&r), 1);
        assert_eq!(Single(16,6).bingo(&r), 1);
    }

    #[test]
    fn test_all_bets() {
        
        let mut m= HashMap::<u16, Box<BetKind>>::new();
        all_bets(&mut m);

        let r = Result::new(1, 1, 1);
        let r1:Vec<_> =m.iter().filter(|&(_,bet)|bet.bingo(&r)>0).collect();
        assert_eq!(r1.len(), 1);

        let r = Result::new(1, 1, 2);
        let r1:Vec<_> =m.iter().filter(|&(_,bet)|bet.bingo(&r)>0).collect();
        assert_eq!(r1.len(), 2);

        let r = Result::new(1, 2, 3);
        let r1:Vec<_> =m.iter().filter(|&(_,bet)|bet.bingo(&r)>0).collect();
        assert_eq!(r1.len(), 3);
    }
}
