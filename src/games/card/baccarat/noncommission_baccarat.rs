use std::collections::{HashMap, HashSet};
use super::common::{Baccarat, Result};
use games::BetSerde;

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

use self::Bets::*;

impl BetSerde for Bets {
    fn to_u16(&self) -> u16 {
        match *self {
            Banker => 1,
            Player => 2,
            Tie => 3,
            BankerPair => 4,
            PlayerPair => 5,
            BankerN8 => 6,
            BankerN9 => 7,
            PlayerN8 => 8,
            PlayerN9 => 9,
            Super6 => 10,
            Big => 11,
            Small => 12,
        }
    }

    fn from_u16(id: u16) -> Option<Bets> {
        match id {
            1 => Some(Banker),
            2 => Some(Player),
            3 => Some(Tie),
            4 => Some(BankerPair),
            5 => Some(PlayerPair),
            6 => Some(BankerN8),
            7 => Some(BankerN9),
            8 => Some(PlayerN8),
            9 => Some(PlayerN9),
            10 => Some(Super6),
            11 => Some(Big),
            12 => Some(Small),
            _ => None,
        }
    }
}

lazy_static!{
    static ref ALL_BETS:HashSet<Bets>= hashset!{ Banker, Player, Tie, BankerN8, PlayerN8, BankerN9, PlayerN9, Super6, BankerPair, PlayerPair, Big, Small };
    static ref BETS_AFTER40:HashSet<Bets> = hashset!{ Banker, Player, Tie, BankerN8, PlayerN8, BankerN9, PlayerN9, Super6, BankerPair, PlayerPair};
    static ref BETS_AFTER70:HashSet<Bets> = hashset!{ Banker, Player, Tie, BankerN8, PlayerN8, BankerN9, PlayerN9, Super6};
}

pub struct NonCommissionBaccaratGame;

pub fn valid_bets(hands: usize) -> &'static HashSet<Bets> {
    if hands <= 40 {
        &ALL_BETS
    } else if hands <= 70 {
        &BETS_AFTER40
    } else {
        &BETS_AFTER70
    }
}

pub fn payout_map(b: &Baccarat) -> HashMap<Bets, f64> {
    let result = b.result();
    let mut map = result_payout_map(result);
    side_bet(b, result, &mut map);
    map
}

#[inline]
fn side_bet(b: &Baccarat, result: Result, map: &mut HashMap<Bets, f64>) {
    if let Result::Banker(6) = result {
        let r = match b.banker_total_cards() {
            3 => 19.0,
            _ => 13.0,
        };
        map.insert(Super6, r);
    }
    if b.total_cards() > 4 {
        map.insert(Big, 1.5);
    } else {
        map.insert(Small, 2.5);
    }
    let (b1, b2) = b.banker_first2();
    if b1.is_same_rank(&b2) {
        map.insert(BankerPair, 12.0);
    }
    let (p1, p2) = b.player_first2();
    if p1.is_same_rank(&p2) {
        map.insert(PlayerPair, 12.0);
    }
}

#[inline]
fn result_payout_map(result: Result) -> HashMap<Bets, f64> {
    let mut map = HashMap::<Bets, f64>::new();
    match result {
        Result::Tie(_) => {
            map.insert(Banker, 1.0);
            map.insert(Player, 1.0);
            map.insert(Tie, 9.0);
        }
        Result::Player(t) => {
            if t == 8 {
                map.insert(PlayerN8, 9.0);
            }
            if t == 9 {
                map.insert(PlayerN9, 9.0);
            }
            map.insert(Player, 2.0);
        }
        Result::Banker(t) => {
            if t == 8 {
                map.insert(BankerN8, 9.0);
            }
            if t == 9 {
                map.insert(BankerN9, 9.0);
            }
            if t == 6 {
                map.insert(Banker, 1.5);
            } else {
                map.insert(Banker, 2.0);
            }
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_bets() {
        let r = valid_bets(1);
        assert_eq!(12, r.len());
        let r = valid_bets(41);
        assert_eq!(10, r.len());
        let r = valid_bets(71);
        assert_eq!(8, r.len())
    }

    #[test]
    fn test_result_payout_map_tie() {
        let expected = hashmap!{Tie =>9.0, Player => 1.0, Banker => 1.0};
        assert_eq!(result_payout_map(Result::Tie(0)), expected);
        assert_eq!(result_payout_map(Result::Tie(1)), expected);
        assert_eq!(result_payout_map(Result::Tie(2)), expected);
        assert_eq!(result_payout_map(Result::Tie(3)), expected);
        assert_eq!(result_payout_map(Result::Tie(4)), expected);
        assert_eq!(result_payout_map(Result::Tie(5)), expected);
        assert_eq!(result_payout_map(Result::Tie(6)), expected);
        assert_eq!(result_payout_map(Result::Tie(7)), expected);
        assert_eq!(result_payout_map(Result::Tie(8)), expected);
        assert_eq!(result_payout_map(Result::Tie(9)), expected);
    }

    #[test]
    fn test_result_payout_map_banker() {
        let expected = hashmap!{Banker => 2.0};
        assert_eq!(result_payout_map(Result::Banker(1)), expected);
        assert_eq!(result_payout_map(Result::Banker(2)), expected);
        assert_eq!(result_payout_map(Result::Banker(3)), expected);
        assert_eq!(result_payout_map(Result::Banker(4)), expected);
        assert_eq!(result_payout_map(Result::Banker(5)), expected);
        assert_eq!(
            result_payout_map(Result::Banker(6)),
            hashmap!{Banker => 1.5}
        );
        assert_eq!(result_payout_map(Result::Banker(7)), expected);
        assert_eq!(
            result_payout_map(Result::Banker(8)),
            hashmap!{Banker => 2.0, BankerN8 => 9.0}
        );
        assert_eq!(
            result_payout_map(Result::Banker(9)),
            hashmap!{Banker => 2.0, BankerN9 => 9.0}
        );
    }

    #[test]
    fn test_result_payout_map_player() {
        let expected = hashmap!{Player => 2.0};
        assert_eq!(result_payout_map(Result::Player(1)), expected);
        assert_eq!(result_payout_map(Result::Player(2)), expected);
        assert_eq!(result_payout_map(Result::Player(3)), expected);
        assert_eq!(result_payout_map(Result::Player(4)), expected);
        assert_eq!(result_payout_map(Result::Player(5)), expected);
        assert_eq!(result_payout_map(Result::Player(6)), expected);
        assert_eq!(result_payout_map(Result::Player(7)), expected);
        assert_eq!(
            result_payout_map(Result::Player(8)),
            hashmap!{Player => 2.0, PlayerN8 =>9.0}
        );
        assert_eq!(
            result_payout_map(Result::Player(9)),
            hashmap!{Player => 2.0, PlayerN9 =>9.0}
        );
    }
}
