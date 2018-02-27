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
        ratio_impl(self.1)
    }
}

#[inline]
fn ratio_impl(n: u8) -> f64 {
    match n {
        4 | 17 => 50.0,
        5 | 16 => 18.0,
        6 | 15 => 14.0,
        7 | 14 => 12.0,
        8 | 13 => 8.0,
        9 | 10 | 11 | 12 => 6.0,
        _ => unreachable!(),
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ratio() {
        assert_eq!(ratio_impl(4),50.0);
        assert_eq!(ratio_impl(5),18.0);
        assert_eq!(ratio_impl(6),14.0);
        assert_eq!(ratio_impl(7),12.0);
        assert_eq!(ratio_impl(8),8.0);
        assert_eq!(ratio_impl(9),6.0);
        assert_eq!(ratio_impl(10),6.0);
        assert_eq!(ratio_impl(11),6.0);
        assert_eq!(ratio_impl(12),6.0);
        assert_eq!(ratio_impl(13),8.0);
        assert_eq!(ratio_impl(14),12.0);
        assert_eq!(ratio_impl(15),14.0);
        assert_eq!(ratio_impl(16),18.0);
        assert_eq!(ratio_impl(17),50.0);
    }
}
