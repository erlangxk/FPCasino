use std::collections::{HashMap, HashSet};
use super::common::Baccarat;
use games::card::Card;

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

pub struct CommissionBaccarat {
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

fn banker_noncommission(result: &mut HashMap<Bets, f64>, tb: u8) {
    if tb == 6 {
        result.insert(Bets::Banker, 1.5);
    } else {
        result.insert(Bets::Banker, 2.0);
    }
}

fn banker_commission(result: &mut HashMap<Bets, f64>, _: u8) {
    result.insert(Bets::Banker, 1.95);
}

fn payout_map_common(
    b: &Baccarat,
    banker_fn: fn(&mut HashMap<Bets, f64>, u8),
) -> HashMap<Bets, f64> {
    let tb = b.banker_total_points();
    let tp = b.player_total_points();
    let mut result = HashMap::<Bets, f64>::new();
    if tb == tp {
        result.insert(Bets::Banker, 1.0);
        result.insert(Bets::Player, 1.0);
        result.insert(Bets::Tie, 9.0);
    } else if tb > tp {
        banker_fn(&mut result, tb);
        if tb == 8 {
            result.insert(Bets::BankerN8, 9.0);
        }
        if tb == 9 {
            result.insert(Bets::BankerN9, 9.0);
        }
        if tb == 6 {
            result.insert(
                Bets::Super6,
                match b.banker_total_cards() {
                    3 => 19.0,
                    _ => 13.0,
                },
            );
        }
    } else {
        result.insert(Bets::Player, 2.0);
        if tp == 8 {
            result.insert(Bets::PlayerN8, 9.0);
        }
        if tp == 9 {
            result.insert(Bets::PlayerN9, 9.0);
        }
    }
    let (b1, b2) = b.banker_first2();
    if Card::is_same_rank(b1, b2) {
        result.insert(Bets::BankerPair, 12.0);
    }
    let (p1, p2) = b.player_first2();
    if Card::is_same_rank(p1, p2) {
        result.insert(Bets::PlayerPair, 12.0);
    }
    result
}

pub fn payout_map_commission(b: &Baccarat) -> HashMap<Bets, f64> {
    payout_map_common(b, banker_commission)
}

pub fn payout_map_noncommission(b: &Baccarat) -> HashMap<Bets, f64> {
    payout_map_common(b, banker_noncommission)
}

impl CommissionBaccarat {
    pub fn new() -> CommissionBaccarat {
        CommissionBaccarat {
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
    use games::card::Card;

    fn card(s: &str) -> Card {
        Card::from_str(s).unwrap()
    }

    #[test]
    fn test_valid_bets() {
        let b = CommissionBaccarat::new();
        let r = b.valid_bets(1);
        assert_eq!(12, r.len());
        let r = b.valid_bets(41);
        assert_eq!(10, r.len());
        let r = b.valid_bets(71);
        assert_eq!(8, r.len())
    }

    #[test]
    fn test_payout_map() {
        let cards = vec![card("ST"), card("S9"), card("H2"), card("DQ")];
        let result = Baccarat::from(&cards).unwrap();
        let pm = payout_map_commission(&result);
        assert_eq!(pm, hashmap!{ Bets::Banker => 1.95, Bets::BankerN9 => 9.0 });
        let pm = payout_map_noncommission(&result);
        assert_eq!(pm, hashmap!{ Bets::Banker => 2.0, Bets::BankerN9 => 9.0 });
    }
}
