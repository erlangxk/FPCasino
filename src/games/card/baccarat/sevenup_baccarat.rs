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
    let (tb, tp, is_tie, is_banker, _) = b.result();
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
