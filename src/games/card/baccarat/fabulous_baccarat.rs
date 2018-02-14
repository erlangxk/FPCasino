use std::collections::{HashMap, HashSet};
use std::fmt;
use super::common::Baccarat;
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
    let tp = b.player_total_points();
    let tb = b.banker_total_points();
    let mut result = HashMap::<Bets, f64>::new();
    if tb == tp {
        result.insert(Bets::Tie, 9.0);
        result.insert(Bets::Banker, 1.0);
        result.insert(Bets::Player, 1.0);
    } else if tb > tp {
        if tb == 4 {
            result.insert(Bets::Banker, 1.0);
            result.insert(Bets::BankerF4, 21.0);
        } else if tb == 1 {
            result.insert(Bets::Banker, 3.0);
        } else {
            result.insert(Bets::Banker, 2.0);
        }
    } else {
        if tp == 4 {
            result.insert(Bets::Player, 1.5);
            result.insert(Bets::PlayerF4, 41.0);
        } else if tb == 1 {
            result.insert(Bets::Player, 3.0);
        } else {
            result.insert(Bets::Player, 2.0);
        }
    }
    fabulous_pair(&mut result, b.banker_first2(), Bets::BankerFPair);
    fabulous_pair(&mut result, b.player_first2(), Bets::PlayerFPair);
    result
}

fn fabulous_pair(result: &mut HashMap<Bets, f64>, first2: (Card, Card), bet: Bets) {
    let (c1, c2) = first2;
    let bsr = Card::is_same_rank(c1, c2);
    let bss = Card::is_same_suit(c1, c2);
    if bsr && bss {
        result.insert(bet, 8.0);
    } else if bsr {
        result.insert(bet, 5.0);
    } else if bss {
        result.insert(bet, 2.0);
    }
}