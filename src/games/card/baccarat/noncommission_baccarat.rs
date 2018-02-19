use std::collections::{HashMap, HashSet};
use super::common::{Baccarat, Result};

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Bets {
    Banker,
    Player,
    Tie,
    BankerPair,
    PlayerPair,
    BankerN8,
    BankerN9,
    PlayerN8,
    PlayerN9,
    Super6,
    Big,
    Small,
}

pub struct NonCommissionBaccarat {
    all_bets: HashSet<Bets>,
    bets_after40: HashSet<Bets>,
    bets_after70: HashSet<Bets>,
}

pub fn all_bets() -> HashSet<Bets> {
    hashset!{ Bets::Banker,Bets::Player,Bets::Tie,
    Bets::BankerN8,Bets::PlayerN8, Bets::BankerN9, Bets::PlayerN9,
    Bets::Super6, Bets::BankerPair,Bets::PlayerPair, Bets::Big, Bets::Small }
}

pub fn bets_after40() -> HashSet<Bets> {
    hashset!{ Bets::Banker,Bets::Player,Bets::Tie,
    Bets::BankerN8,Bets::PlayerN8, Bets::BankerN9, Bets::PlayerN9,
    Bets::Super6, Bets::BankerPair,Bets::PlayerPair}
}

pub fn bets_after70() -> HashSet<Bets> {
    hashset!{ Bets::Banker,Bets::Player,Bets::Tie,
    Bets::BankerN8,Bets::PlayerN8, Bets::BankerN9, Bets::PlayerN9,
    Bets::Super6}
}

fn payout_map(b: &Baccarat) -> HashMap<Bets, f64> {
    let result = b.result();
    let mut map = result_payout_map(result);
    if let Result::Banker(6) = result {
        let r = match b.banker_total_cards() {
            3 => 19.0,
            _ => 13.0,
        };
        map.insert(Bets::Super6, r);
    }
    let (b1, b2) = b.banker_first2();
    if b1.is_same_rank(&b2) {
        map.insert(Bets::BankerPair, 12.0);
    }
    let (p1, p2) = b.player_first2();
    if p1.is_same_rank(&p2) {
        map.insert(Bets::PlayerPair, 12.0);
    }
    map
}

fn result_payout_map(result: Result) -> HashMap<Bets, f64> {
    let mut map = HashMap::<Bets, f64>::new();
    match result {
        Result::Tie(_) => {
            map.insert(Bets::Banker, 1.0);
            map.insert(Bets::Player, 1.0);
            map.insert(Bets::Tie, 9.0);
        }
        Result::Player(t) => {
            if t == 8 {
                map.insert(Bets::PlayerN8, 9.0);
            }
            if t == 9 {
                map.insert(Bets::PlayerN9, 9.0);
            }
            map.insert(Bets::Player, 2.0);
        }
        Result::Banker(t) => {
            if t == 8 {
                map.insert(Bets::BankerN8, 9.0);
            }
            if t == 9 {
                map.insert(Bets::BankerN9, 9.0);
            }
            if t == 6 {
                map.insert(Bets::Banker, 1.5);
            }else {
                map.insert(Bets::Banker, 2.0);
            }
        }
    }
    map
}

impl NonCommissionBaccarat {
    pub fn new() -> NonCommissionBaccarat {
        NonCommissionBaccarat {
            all_bets: all_bets(),
            bets_after40: bets_after40(),
            bets_after70: bets_after70(),
        }
    }

    pub fn valid_bets(&self, hands: usize) -> &HashSet<Bets> {
        if hands <= 40 {
            &self.all_bets
        } else if hands <= 70 {
            &self.bets_after40
        } else {
            &self.bets_after70
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_bets() {
        let b = NonCommissionBaccarat::new();
        let r = b.valid_bets(1);
        assert_eq!(12, r.len());
        let r = b.valid_bets(41);
        assert_eq!(10, r.len());
        let r = b.valid_bets(71);
        assert_eq!(8, r.len())
    }
}