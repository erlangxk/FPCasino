use std::collections::{HashMap, HashSet};
use super::common::Baccarat;

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Bets {
    Banker,
    Player,
    Tie,
    Super7,
}

pub fn all_bets() -> HashSet<Bets> {
    hashset!{ Bets::Banker,Bets::Player,Bets::Tie,Bets::Super7}
}

pub fn bets_after40() -> HashSet<Bets> {
    hashset!{ Bets::Banker,Bets::Player,Bets::Tie}
}

struct SevenupBaccarat {
    all_bets: HashSet<Bets>,
    bets_after40: HashSet<Bets>,
}

impl SevenupBaccarat {
    pub fn new() -> SevenupBaccarat {
        SevenupBaccarat {
            all_bets: all_bets(),
            bets_after40: bets_after40(),
        }
    }

    pub fn valid_bets(&self, hands: usize) -> &HashSet<Bets> {
        if hands <= 40 {
            &self.all_bets
        } else {
            &self.bets_after40
        }
    }
}

pub fn payout_map(b: &Baccarat) -> HashMap<Bets, f64> {
    let (tb, tp, is_banker, _, is_tie) = b.result();
    let mut result = HashMap::<Bets, f64>::new();
    if is_tie {
        result.insert(Bets::Tie, cmp(tb, 10.0, 8.0));
        result.insert(Bets::Banker, 1.0);
        result.insert(Bets::Player, 1.0);
    } else if is_banker {
        result.insert(Bets::Banker, cmp(tb, 2.5, 2.0));
    } else {
        result.insert(Bets::Player, cmp(tp, 1.5, 2.0));
    }
    if let Some(r) = ratio7(b.count_cards(7)) {
        result.insert(Bets::Super7, r);
    }
    result
}

fn ratio7(n7: usize) -> Option<f64> {
    match n7 {
        6 => Some(778.0),
        5 => Some(78.0),
        4 => Some(16.0),
        3 => Some(7.0),
        2 => Some(2.5),
        _ => None,
    }
}

fn cmp(total: u8, f1: f64, f2: f64) -> f64 {
    match total {
        7 => f1,
        _ => f2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use games::card::serde::str_to_card;
    use games::card::Card;
    fn card(s: &str) -> Card {
        str_to_card(s).unwrap()
    }

    #[test]
    fn test_payout_1(){
        let cards = vec![
            card("D7"),
            card("C2"),
            card("CJ"),
            card("CA"),
            card("HJ"),
        ];
        let b = Baccarat::from(&cards).unwrap();
        assert_eq!(3, b.banker_total_cards());
        assert_eq!((3,7,false, true, false), b.result());
        let m = payout_map(&b);
        assert_eq!(m, hashmap!{Bets::Player=>1.5});
    }

    #[test]
    fn test_payout_2(){
        let cards = vec![
            card("D7"),
            card("S4"),
            card("SA"),
            card("CJ"),
        ];
        let b = Baccarat::from(&cards).unwrap();
        assert_eq!(2, b.banker_total_cards());
        assert_eq!((4,8,false, true, false), b.result());
        let m = payout_map(&b);
        assert_eq!(m, hashmap!{Bets::Player=>2.0});
    }

    #[test]
    fn test_payout_3(){
        let cards = vec![
            card("D7"),
            card("D5"),
            card("D7"),
            card("H6"),
            card("CT"),
            card("S6"),
        ];
        let b = Baccarat::from(&cards).unwrap();
        assert_eq!(3, b.banker_total_cards());
        assert_eq!((7,4,true,false,false), b.result());
        let m = payout_map(&b);
        assert_eq!(m, hashmap!{Bets::Banker=>2.5, Bets::Super7=>2.5});
    }

    #[test]
    fn test_payout_4(){
        let cards = vec![
            card("D7"),
            card("H8"),
            card("D5"),
            card("ST"),
        ];
        let b = Baccarat::from(&cards).unwrap();
        assert_eq!(2, b.banker_total_cards());
        assert_eq!((8,2,true,false,false), b.result());
        let m = payout_map(&b);
        assert_eq!(m, hashmap!{Bets::Banker=>2.0});
    }

    #[test]
    fn test_payout_5(){
        let cards = vec![
            card("D7"),
            card("H9"),
            card("HQ"),
            card("H8"),
        ];
        let b = Baccarat::from(&cards).unwrap();
        assert_eq!(2, b.banker_total_cards());
        assert_eq!((7,7,false,false,true), b.result());
        let m = payout_map(&b);
        assert_eq!(m, hashmap!{Bets::Banker=>1.0, Bets::Player=>1.0, Bets::Tie=>10.0});
    }
    
    #[test]
    fn test_payout_6(){
        let cards = vec![
            card("D7"),
            card("H8"),
            card("SA"),
            card("CK"),
        ];
        let b = Baccarat::from(&cards).unwrap();
        assert_eq!(2, b.banker_total_cards());
        assert_eq!((8,8,false,false,true), b.result());
        let m = payout_map(&b);
        assert_eq!(m, hashmap!{Bets::Banker=>1.0, Bets::Player=>1.0, Bets::Tie=>8.0});
    }

    #[test]
    fn test_payout_7(){
        let cards = vec![
            card("D7"),
            card("H4"),
            card("H7"),
            card("D7"),
            card("S6"),
            card("HK"),
        ];
        let b = Baccarat::from(&cards).unwrap();
        assert_eq!(3, b.banker_total_cards());
        assert_eq!((1,0,true,false,false), b.result());
        let m = payout_map(&b);
        assert_eq!(m, hashmap!{Bets::Banker=>2.0, Bets::Super7=>7.0});
    }
    
    #[test]
    fn test_ratio7(){
        assert_eq!(Some(778.0), ratio7(6));
        assert_eq!(Some(78.0), ratio7(5));
        assert_eq!(Some(16.0), ratio7(4));
        assert_eq!(Some(7.0), ratio7(3));
        assert_eq!(Some(2.5), ratio7(2));
        assert_eq!(None, ratio7(1));
    }

    #[test]
    fn test_valid_bets(){
        let b = SevenupBaccarat::new();
        let s1 = b.valid_bets(40);
        assert_eq!(4, s1.len());
        assert_eq!(hashset!{Bets::Banker,Bets::Player,Bets::Tie,Bets::Super7}, *s1);
        let s1 = b.valid_bets(41);
        assert_eq!(3, s1.len());
        assert_eq!(hashset!{Bets::Banker,Bets::Player,Bets::Tie}, *s1);
    }
}