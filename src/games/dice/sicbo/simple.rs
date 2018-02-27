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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bingo() {
        let r = Result::new(1, 1, 1);
        assert_eq!(Simple::Big.bingo(&r), 0);
        assert_eq!(Simple::Small.bingo(&r), 0);
        assert_eq!(Simple::Odd.bingo(&r), 0);
        assert_eq!(Simple::Even.bingo(&r), 0);

        let r = Result::new(1, 1, 2);
        assert_eq!(Simple::Big.bingo(&r), 0);
        assert_eq!(Simple::Small.bingo(&r), 1);
        assert_eq!(Simple::Odd.bingo(&r), 0);
        assert_eq!(Simple::Even.bingo(&r), 1);

        let r = Result::new(1, 1, 3);
        assert_eq!(Simple::Big.bingo(&r), 0);
        assert_eq!(Simple::Small.bingo(&r), 1);
        assert_eq!(Simple::Odd.bingo(&r), 1);
        assert_eq!(Simple::Even.bingo(&r), 0);

        let r = Result::new(4, 5, 6);
        assert_eq!(Simple::Big.bingo(&r), 1);
        assert_eq!(Simple::Small.bingo(&r), 0);
        assert_eq!(Simple::Odd.bingo(&r), 1);
        assert_eq!(Simple::Even.bingo(&r), 0);

        let r = Result::new(4, 4, 6);
        assert_eq!(Simple::Big.bingo(&r), 1);
        assert_eq!(Simple::Small.bingo(&r), 0);
        assert_eq!(Simple::Odd.bingo(&r), 0);
        assert_eq!(Simple::Even.bingo(&r), 1);
    }
}
