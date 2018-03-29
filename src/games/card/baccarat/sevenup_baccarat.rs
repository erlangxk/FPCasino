use std::collections::{HashMap, HashSet};
use super::common::{Baccarat, Result};
use games::BetSerde;

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Bets {
    Banker,
    Player,
    Tie,
    Super7,
}

use self::Bets::*;

impl BetSerde for Bets {
    fn to_u16(&self) -> u16 {
        match *self {
            Banker => 1,
            Player => 2,
            Tie => 3,
            Super7 => 4,
        }
    }

    fn from_u16(id: u16) -> Option<Bets> {
        match id {
            1 => Some(Banker),
            2 => Some(Player),
            3 => Some(Tie),
            4 => Some(Super7),
            _ => None,
        }
    }
}

lazy_static! {
    static ref ALL_BETS:HashSet<Bets> = hashset!{ Banker,Player,Tie,Super7};
    static ref BETS_AFTER40:HashSet<Bets> = hashset!{ Banker,Player,Tie};
}

pub struct SevenupBaccaratGame;

pub fn valid_bets(hands: usize) -> &'static HashSet<Bets> {
    if hands <= 40 {
        &ALL_BETS
    } else {
        &BETS_AFTER40
    }
}

pub fn payout_map(b: &Baccarat) -> HashMap<Bets, f64> {
    let mut map = result_payout_map(b.result());
    if let Some(r) = ratio7(b.count_cards(7)) {
        map.insert(Super7, r);
    }
    map
}

fn result_payout_map(result: Result) -> HashMap<Bets, f64> {
    let mut map = HashMap::<Bets, f64>::new();
    match result {
        Result::Tie(7) => {
            map.insert(Tie, 10.0);
        }
        Result::Tie(_) => {
            map.insert(Tie, 8.0);
        }
        Result::Banker(7) => {
            map.insert(Banker, 2.5);
        }
        Result::Banker(_) => {
            map.insert(Banker, 2.0);
        }
        Result::Player(7) => {
            map.insert(Player, 1.5);
        }
        Result::Player(_) => {
            map.insert(Player, 2.0);
        }
    }
    if let Result::Tie(_) = result {
        map.insert(Banker, 1.0);
        map.insert(Player, 1.0);
    }
    map
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

#[cfg(test)]
mod tests {
    use super::*;
    use games::card::serde::str_to_card;
    use games::card::Card;

    fn card(s: &str) -> Card {
        str_to_card(s).unwrap()
    }

    #[test]
    fn test_payout_1() {
        let cards = vec![card("D7"), card("C2"), card("CJ"), card("CA"), card("HJ")];
        let b = Baccarat::from(&cards).unwrap();
        assert_eq!(3, b.banker_total_cards());
        assert_eq!(Result::Player(7), b.result());
        let m = payout_map(&b);
        assert_eq!(m, hashmap!{Player=>1.5});
    }

    #[test]
    fn test_payout_2() {
        let cards = vec![card("D7"), card("S4"), card("SA"), card("CJ")];
        let b = Baccarat::from(&cards).unwrap();
        assert_eq!(2, b.banker_total_cards());
        assert_eq!(Result::Player(8), b.result());
        let m = payout_map(&b);
        assert_eq!(m, hashmap!{Player=>2.0});
    }

    #[test]
    fn test_payout_3() {
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
        assert_eq!(Result::Banker(7), b.result());
        let m = payout_map(&b);
        assert_eq!(m, hashmap!{Banker=>2.5, Super7=>2.5});
    }

    #[test]
    fn test_payout_4() {
        let cards = vec![card("D7"), card("H8"), card("D5"), card("ST")];
        let b = Baccarat::from(&cards).unwrap();
        assert_eq!(2, b.banker_total_cards());
        assert_eq!(Result::Banker(8), b.result());
        let m = payout_map(&b);
        assert_eq!(m, hashmap!{Banker=>2.0});
    }

    #[test]
    fn test_payout_5() {
        let cards = vec![card("D7"), card("H9"), card("HQ"), card("H8")];
        let b = Baccarat::from(&cards).unwrap();
        assert_eq!(2, b.banker_total_cards());
        assert_eq!(Result::Tie(7), b.result());
        let m = payout_map(&b);
        assert_eq!(m, hashmap!{Banker=>1.0, Player=>1.0, Tie=>10.0});
    }

    #[test]
    fn test_payout_6() {
        let cards = vec![card("D7"), card("H8"), card("SA"), card("CK")];
        let b = Baccarat::from(&cards).unwrap();
        assert_eq!(2, b.banker_total_cards());
        assert_eq!(Result::Tie(8), b.result());
        let m = payout_map(&b);
        assert_eq!(m, hashmap!{Banker=>1.0, Player=>1.0, Tie=>8.0});
    }

    #[test]
    fn test_payout_7() {
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
        assert_eq!(Result::Banker(1), b.result());
        let m = payout_map(&b);
        assert_eq!(m, hashmap!{Banker=>2.0, Super7=>7.0});
    }

    #[test]
    fn test_ratio7() {
        assert_eq!(Some(778.0), ratio7(6));
        assert_eq!(Some(78.0), ratio7(5));
        assert_eq!(Some(16.0), ratio7(4));
        assert_eq!(Some(7.0), ratio7(3));
        assert_eq!(Some(2.5), ratio7(2));
        assert_eq!(None, ratio7(1));
    }

    #[test]
    fn test_valid_bets() {
        let s1 = valid_bets(40);
        assert_eq!(4, s1.len());
        assert_eq!(hashset!{Banker,Player,Tie,Super7}, *s1);
        let s1 = valid_bets(41);
        assert_eq!(3, s1.len());
        assert_eq!(hashset!{Banker,Player,Tie}, *s1);
    }
}
