use std::collections::{HashMap, HashSet};
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
    let (tb, tp) = b.totals();
    let mut result = HashMap::<Bets, f64>::new();
    let is_tie = tb == tp;
    let is_banker = tb > tp;
    let is_player = tb < tp;
    
    if is_tie {
        result.insert(Bets::Tie, 9.0);
        result.insert(Bets::Banker, 1.0);
        result.insert(Bets::Player, 1.0);
    } else if is_banker {
        result.insert(Bets::Banker, ratio(tb, 1.0, 3.0, 2.0));
    } else {
        result.insert(Bets::Player, ratio(tp, 1.5, 3.0, 2.0));
    }

    if (is_banker && tb == 4) {
        result.insert(Bets::BankerF4, 21.0);
    }
    if (is_player && tp == 4) {
        result.insert(Bets::PlayerF4, 41.0);
    }

    if let Some(r) = fabulous_pair(b.banker_first2()) {
        result.insert(Bets::BankerFPair, r);
    }
    if let Some(r) = fabulous_pair(b.player_first2()) {
        result.insert(Bets::PlayerFPair, r);
    }
    result
}

fn ratio(total: u8, f1: f64, f2: f64, f3: f64) -> f64 {
    match total {
        4 => f1,
        1 => f2,
        _ => f3,
    }
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
