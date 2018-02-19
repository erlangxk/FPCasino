use std::collections::{HashMap, HashSet};
use super::common::{Baccarat, Result};
use games::card::Card;

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Bets {
    Banker,
    Player,
    Tie,
    BankerFPair,
    PlayerFPair,
    BankerF4,
    PlayerF4,
}

pub fn all_bets() -> HashSet<Bets> {
    hashset!{ Bets::Banker,Bets::Player,Bets::Tie,Bets::BankerF4, Bets::PlayerF4,
    Bets::BankerFPair,Bets::PlayerFPair}
}

pub fn bets_after70() -> HashSet<Bets> {
    hashset!{ Bets::Banker,Bets::Player,Bets::Tie,Bets::BankerF4, Bets::PlayerF4}
}

struct FabulousBaccarat {
    all_bets: HashSet<Bets>,
    bets_after70: HashSet<Bets>,
}

impl FabulousBaccarat {
    pub fn new() -> FabulousBaccarat {
        FabulousBaccarat {
            all_bets: all_bets(),
            bets_after70: bets_after70(),
        }
    }

    pub fn valid_bets(&self, hands: usize) -> &HashSet<Bets> {
        if hands <= 70 {
            &self.all_bets
        } else {
            &self.bets_after70
        }
    }
}

pub fn payout_map(b: &Baccarat) -> HashMap<Bets, f64> {
    let mut map = result_payout_map(b.result2());
    if let Some(r) = fabulous_pair(b.banker_first2()) {
        map.insert(Bets::BankerFPair, r);
    }
    if let Some(r) = fabulous_pair(b.player_first2()) {
        map.insert(Bets::PlayerFPair, r);
    }
    map
}

fn result_payout_map(result: Result) -> HashMap<Bets, f64> {
    let mut map = HashMap::<Bets, f64>::new();
    match result {
        Result::Tie(_) => {
            map.insert(Bets::Tie, 9.0);
            map.insert(Bets::Banker, 1.0);
            map.insert(Bets::Player, 1.0);
        }
        Result::Banker(4) => {
            map.insert(Bets::BankerF4, 21.0);
            map.insert(Bets::Banker, 1.0);
        }
        Result::Banker(1) => {
            map.insert(Bets::Banker, 3.0);
        }
        Result::Banker(_) => {
            map.insert(Bets::Banker, 2.0);
        }
        Result::Player(4) => {
            map.insert(Bets::PlayerF4, 41.0);
            map.insert(Bets::Player, 1.5);
        }
        Result::Player(1) => {
            map.insert(Bets::Player, 3.0);
        }
        Result::Player(_) => {
            map.insert(Bets::Player, 2.0);
        }
    }
    map
}

fn fabulous_pair(first2: (Card, Card)) -> Option<f64> {
    let (c1, c2) = first2;
    let bsr = c1.is_same_rank(&c2);
    let bss = c1.is_same_suit(&c2);
    if bsr && bss {
        Some(8.0)
    } else if bsr {
        Some(5.0)
    } else if bss {
        Some(2.0)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_payout_map() {
        let r = Result::Tie(1);
        let m = result_payout_map(r);
        assert_eq!(
            m,
            hashmap!{Bets::Banker=> 1.0, Bets::Player=> 1.0, Bets::Tie=> 9.0}
        );

        let r = Result::Banker(4);
        let m = result_payout_map(r);
        assert_eq!(m,hashmap!{Bets::Banker=> 1.0, Bets::BankerF4=> 21.0});

        let r = Result::Banker(1);
        let m = result_payout_map(r);
        assert_eq!(m,hashmap!{Bets::Banker=> 3.0});

        let r = Result::Banker(9);
        let m = result_payout_map(r);
        assert_eq!(m,hashmap!{Bets::Banker=> 2.0});

        let r = Result::Player(4);
        let m = result_payout_map(r);
        assert_eq!(m,hashmap!{Bets::Player=> 1.5, Bets::PlayerF4=> 41.0});

        let r = Result::Player(1);
        let m = result_payout_map(r);
        assert_eq!(m,hashmap!{Bets::Player=> 3.0});

        let r = Result::Player(9);
        let m = result_payout_map(r);
        assert_eq!(m,hashmap!{Bets::Player=> 2.0});

        let r = Result::Player(8);
        let m = result_payout_map(r);
        assert_eq!(m,hashmap!{Bets::Player=> 2.0});
    }
}

