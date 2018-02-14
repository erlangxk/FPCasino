use std::collections::{HashMap, HashSet};
use super::common::Baccarat;
use games::card::Card;

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
    let tp = b.player_total_points();
    let tb = b.banker_total_points();
    let mut result = HashMap::<Bets, f64>::new();
    if tb == tp {
        if tb == 7 {
            result.insert(Bets::Tie, 10.0);
        } else {
            result.insert(Bets::Tie, 8.0);
        }
        result.insert(Bets::Banker, 1.0);
        result.insert(Bets::Player, 1.0);
    } else if tb > tp {
        if tb == 7 {
            result.insert(Bets::Banker, 2.5);
        } else {
            result.insert(Bets::Banker, 2.0);
        }
    } else {
        if tp == 7 {
            result.insert(Bets::Player, 1.5);
        } else {
            result.insert(Bets::Player, 2.0);
        }
    }
    let n7 = b.count_cards(7);
    if n7 == 6 {
        result.insert(Bets::Super7, 778.0);
    } else if n7 == 5 {
        result.insert(Bets::Super7, 78.0);
    } else if n7 == 4 {
        result.insert(Bets::Super7, 16.0);
    } else if n7 == 3 {
        result.insert(Bets::Super7, 7.0);
    } else if n7 == 2 {
        result.insert(Bets::Super7, 2.5);
    }
    result
}
