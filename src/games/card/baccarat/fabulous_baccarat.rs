use std::collections::{HashMap, HashSet};
use super::common::{Baccarat, Result};
use games::card::Card;
use games::BetSerde;

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

use self::Bets::*;

impl BetSerde for Bets {
    fn to_u16(&self) -> u16 {
        match *self {
            Banker => 1,
            Player => 2,
            Tie => 3,
            BankerFPair => 4,
            PlayerFPair => 5,
            BankerF4 => 6,
            PlayerF4 => 7,
        }
    }
    
    fn from_u16(id: u16) -> Option<Bets> {
        match id {
            1 => Some(Banker),
            2 => Some(Player),
            3 => Some(Tie),
            4 => Some(BankerFPair),
            5 => Some(PlayerFPair),
            6 => Some(BankerF4),
            7 => Some(PlayerF4),
            _ => None,
        }
    }
}

struct FabulousBaccaratGame {
    all_bets: HashSet<Bets>,
    bets_after70: HashSet<Bets>,
}

impl FabulousBaccaratGame {
    pub fn new() -> FabulousBaccaratGame {
        FabulousBaccaratGame {
            all_bets: hashset!{ Banker, Player, Tie, BankerF4, PlayerF4, BankerFPair, PlayerFPair},
            bets_after70: hashset!{ Banker, Player, Tie, BankerF4, PlayerF4},
        }
    }

    fn valid_bets(&self, hands: usize) -> &HashSet<Bets> {
        if hands <= 70 {
            &self.all_bets
        } else {
            &self.bets_after70
        }
    }
}

pub fn payout_map(b: &Baccarat) -> HashMap<Bets, f64> {
    let mut map = result_payout_map(b.result());
    if let Some(r) = fabulous_pair(b.banker_first2()) {
        map.insert(BankerFPair, r);
    }
    if let Some(r) = fabulous_pair(b.player_first2()) {
        map.insert(PlayerFPair, r);
    }
    map
}

fn result_payout_map(result: Result) -> HashMap<Bets, f64> {
    let mut map = HashMap::<Bets, f64>::new();
    match result {
        Result::Tie(_) => {
            map.insert(Tie, 9.0);
            map.insert(Banker, 1.0);
            map.insert(Player, 1.0);
        }
        Result::Banker(4) => {
            map.insert(BankerF4, 21.0);
            map.insert(Banker, 1.0);
        }
        Result::Banker(1) => {
            map.insert(Banker, 3.0);
        }
        Result::Banker(_) => {
            map.insert(Banker, 2.0);
        }
        Result::Player(4) => {
            map.insert(PlayerF4, 41.0);
            map.insert(Player, 1.5);
        }
        Result::Player(1) => {
            map.insert(Player, 3.0);
        }
        Result::Player(_) => {
            map.insert(Player, 2.0);
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
    use super::Bets::*;

    #[test]
    fn test_result_payout_map() {
        let r = Result::Tie(1);
        let m = result_payout_map(r);
        assert_eq!(
            m,
            hashmap!{Banker=> 1.0, Player=> 1.0, Tie=> 9.0}
        );

        let r = Result::Banker(4);
        let m = result_payout_map(r);
        assert_eq!(m,hashmap!{Banker=> 1.0, BankerF4=> 21.0});

        let r = Result::Banker(1);
        let m = result_payout_map(r);
        assert_eq!(m,hashmap!{Banker=> 3.0});

        let r = Result::Banker(9);
        let m = result_payout_map(r);
        assert_eq!(m,hashmap!{Banker=> 2.0});

        let r = Result::Player(4);
        let m = result_payout_map(r);
        assert_eq!(m,hashmap!{Player=> 1.5, PlayerF4=> 41.0});

        let r = Result::Player(1);
        let m = result_payout_map(r);
        assert_eq!(m,hashmap!{Player=> 3.0});

        let r = Result::Player(9);
        let m = result_payout_map(r);
        assert_eq!(m,hashmap!{Player=> 2.0});

        let r = Result::Player(8);
        let m = result_payout_map(r);
        assert_eq!(m,hashmap!{Player=> 2.0});
    }
}

