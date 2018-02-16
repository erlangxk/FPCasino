use std::collections::{HashMap, HashSet};
use super::common::Baccarat;
use games::card::{Card, Rank};

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Bets {
    Banker,
    Player,
    Tie,
    BankerBlack,
    BankerRed,
    BankerLuckyPair,
    PlayerBlack,
    PlayerRed,
    PlayerLuckyPair,
    Lucky6,
    BankerWinsOn123,
    BankerWinsOn456,
    BankerWinsOn789,
    PlayerWinsOn123,
    PlayerWinsOn456,
    PlayerWinsOn789,
    TieOn0123,
    TieOn456,
    TieOn789,
}

pub fn all_bets() -> HashSet<Bets> {
    hashset!{
        Bets::Banker,Bets::Player,Bets::Tie,Bets::Lucky6,
        Bets::BankerBlack, Bets::BankerRed, Bets::BankerLuckyPair,
        Bets::PlayerBlack, Bets::PlayerRed, Bets::PlayerLuckyPair,
        Bets::BankerWinsOn123, Bets::BankerWinsOn456, Bets::BankerWinsOn789,
        Bets::PlayerWinsOn123, Bets::PlayerWinsOn456, Bets::PlayerWinsOn789,
        Bets::TieOn0123, Bets::TieOn456,Bets::TieOn789,
    }
}

pub fn bets_after40() -> HashSet<Bets> {
    hashset!{
        Bets::Banker,Bets::Player,Bets::Tie,
        Bets::BankerBlack, Bets::BankerRed, Bets::BankerLuckyPair,
        Bets::PlayerBlack, Bets::PlayerRed, Bets::PlayerLuckyPair,
    }
}

pub fn bets_after60() -> HashSet<Bets> {
    hashset!{ Bets::Banker,Bets::Player,Bets::Tie}
}

struct LuckyBaccarat {
    all_bets: HashSet<Bets>,
    bets_after40: HashSet<Bets>,
    bets_after60: HashSet<Bets>,
}

impl LuckyBaccarat {
    pub fn new() -> LuckyBaccarat {
        LuckyBaccarat {
            all_bets: all_bets(),
            bets_after40: bets_after40(),
            bets_after60: bets_after60(),
        }
    }

    pub fn valid_bets(&self, hands: usize) -> &HashSet<Bets> {
        if hands <= 40 {
            &self.all_bets
        } else if hands <= 60 {
            &self.bets_after40
        } else {
            &self.bets_after60
        }
    }
}

pub fn payout_map(b: &Baccarat) -> HashMap<Bets, f64> {
    let (tb, tp, is_banker, is_player, is_tie) = b.result();
    let mut result = HashMap::<Bets, f64>::new();

    if (is_banker && tb == 6) || (is_player && tp == 6) || (is_tie && tb == 6) {
        result.insert(Bets::Lucky6, 7.0);
    }

    if is_tie {
        result.insert(Bets::Tie, 9.0);
        result.insert(Bets::Banker, 1.0);
        result.insert(Bets::Player, 1.0);
    } else if is_banker {
        if tb == 6 {
            result.insert(Bets::Banker, 1.5);
        } else {
            result.insert(Bets::Banker, 2.0);
        }
    } else {
        result.insert(Bets::Player, 2.0);
    }
    let (bets, ratio) = {
        if is_tie {
            cmp(
                tb,
                (Bets::TieOn0123, 46.0),
                (Bets::TieOn456, 25.0),
                (Bets::TieOn789, 20.0),
            )
        } else if is_banker {
            cmp(
                tb,
                (Bets::BankerWinsOn123, 32.0),
                (Bets::BankerWinsOn456, 7.0),
                (Bets::BankerWinsOn789, 3.0),
            )
        } else {
            cmp(
                tp,
                (Bets::PlayerWinsOn123, 32.0),
                (Bets::PlayerWinsOn456, 7.0),
                (Bets::PlayerWinsOn789, 3.0),
            )
        }
    };
    result.insert(bets, ratio);

    {
        let mut side_bet = |pair: (Card, Card), bets: (Bets, Bets, Bets)| {
            let (c1, c2) = pair;
            if c1.is_black() && c2.is_black() {
                result.insert(bets.0, 3.0);
            } else if c1.is_red() && c2.is_red() {
                result.insert(bets.1, 3.0);
            }
            if let Some(r) = ratio_of_lucky_pair(&c1, &c2) {
                result.insert(bets.2, r);
            }
        };
        side_bet(
            b.banker_first2(),
            (Bets::BankerBlack, Bets::BankerRed, Bets::BankerLuckyPair),
        );
        side_bet(
            b.player_first2(),
            (Bets::PlayerBlack, Bets::PlayerRed, Bets::PlayerLuckyPair),
        );
    }
    result
}

fn cmp<T>(total: u8, f1: T, f2: T, f3: T) -> T {
    if total <= 3 {
        f1
    } else if total <= 6 {
        f2
    } else {
        f3
    }
}

fn ratio_of_lucky_pair(c1: &Card, c2: &Card) -> Option<f64> {
    if c1.is_same_rank(c2) {
        let is_diamond = c1.is_diamond() && c1.is_same_suit(c2);
        let is_four = c1.rank == Rank::Four;
        if is_diamond && is_four {
            return Some(31.0);
        }
        if is_four {
            return Some(16.0);
        }
        if is_diamond {
            return Some(13.0);
        }
        return Some(10.0);
    }
    None
}
