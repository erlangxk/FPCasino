use std::collections::{HashMap, HashSet};
use super::common::{Baccarat, Result};
use games::card::{Card, Rank};
use games::BetSerde;

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

use self::Bets::*;

impl BetSerde for Bets {
    
    fn to_u16(&self) -> u16 {
        match *self {
            Banker => 1,
            Player => 2,
            Tie => 3,
            BankerBlack => 4,
            BankerRed => 5,
            BankerLuckyPair => 6,
            PlayerBlack => 7,
            PlayerRed => 8,
            PlayerLuckyPair => 9,
            Lucky6 => 10,
            BankerWinsOn123 => 11,
            BankerWinsOn456 => 12,
            BankerWinsOn789 => 13,
            PlayerWinsOn123 => 14,
            PlayerWinsOn456 => 15,
            PlayerWinsOn789 => 16,
            TieOn0123 => 17,
            TieOn456 => 18,
            TieOn789 => 19,
        }
    }
    
    fn from_u16(id: u16) -> Option<Bets> {
        match id {
            1 => Some(Banker),
            2 => Some(Player),
            3 => Some(Tie),
            4 => Some(BankerBlack),
            5 => Some(BankerRed),
            6 => Some(BankerLuckyPair),
            7 => Some(PlayerBlack),
            8 => Some(PlayerRed),
            9 => Some(PlayerLuckyPair),
            10 => Some(Lucky6),
            11 => Some(BankerWinsOn123),
            12 => Some(BankerWinsOn456),
            13 => Some(BankerWinsOn456),
            14 => Some(PlayerWinsOn123),
            15 => Some(PlayerWinsOn456),
            16 => Some(PlayerWinsOn789),
            17 => Some(TieOn0123),
            18 => Some(TieOn456),
            19 => Some(TieOn789),
            _ => None,
        }
    }
}

pub fn all_bets() -> HashSet<Bets> {
    hashset!{
        Banker,Player,Tie,
        BankerBlack, BankerRed, BankerLuckyPair,
        PlayerBlack, PlayerRed, PlayerLuckyPair,
        BankerWinsOn123, BankerWinsOn456, BankerWinsOn789,
        PlayerWinsOn123, PlayerWinsOn456, PlayerWinsOn789,
        TieOn0123, TieOn456,TieOn789,
        Lucky6,
    }
}

pub fn bets_after40() -> HashSet<Bets> {
    hashset!{
        Banker,Player,Tie,
        BankerBlack, BankerRed, BankerLuckyPair,
        PlayerBlack, PlayerRed, PlayerLuckyPair,
    }
}

pub fn bets_after60() -> HashSet<Bets> {
    hashset!{ Banker,Player,Tie }
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
    let mut map = result_payout_map(b.result());
    side_bet(
        b.banker_first2(),
        BankerBlack,
        BankerRed,
        BankerLuckyPair,
        &mut map,
    );
    side_bet(
        b.player_first2(),
        PlayerBlack,
        PlayerRed,
        PlayerLuckyPair,
        &mut map,
    );
    map
}

fn side_bet(pair: (Card, Card), b1: Bets, b2: Bets, b3: Bets, map: &mut HashMap<Bets, f64>) {
    let (c1, c2) = pair;
    if c1.is_black() && c2.is_black() {
        map.insert(b1, 3.0);
    } else if c1.is_red() && c2.is_red() {
        map.insert(b2, 3.0);
    }
    if let Some(r) = ratio_of_lucky_pair(&c1, &c2) {
        map.insert(b3, r);
    }
}

fn result_payout_map(result: Result) -> HashMap<Bets, f64> {
    let mut map = HashMap::<Bets, f64>::new();
    if result.total_points() == 6 {
        map.insert(Lucky6, 7.0);
    }
    match result {
        Result::Tie(_) => {
            map.insert(Tie, 9.0);
            map.insert(Banker, 1.0);
            map.insert(Player, 1.0);
        }
        Result::Banker(6) => {
            map.insert(Banker, 1.5);
        }
        Result::Banker(_) => {
            map.insert(Banker, 2.0);
        }
        _ => {
            map.insert(Player, 2.0);
        }
    }
    let (bets, ratio) = wins_on(result);
    map.insert(bets, ratio);
    map
}

fn wins_on(result: Result) -> (Bets, f64) {
    match result {
        Result::Banker(1...3) => (BankerWinsOn123, 32.0),
        Result::Banker(4...6) => (BankerWinsOn456, 7.0),
        Result::Banker(_) => (BankerWinsOn789, 3.0),
        Result::Player(1...3) => (PlayerWinsOn123, 32.0),
        Result::Player(4...6) => (PlayerWinsOn456, 9.0),
        Result::Player(_) => (PlayerWinsOn789, 3.0),
        Result::Tie(0...3) => (TieOn0123, 46.0),
        Result::Tie(4...6) => (TieOn456, 25.0),
        Result::Tie(_) => (TieOn789, 20.0),
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


#[cfg(test)]
mod tests {
    use super::*;
    use super::Bets::*;
    use games::card::serde::str_to_card;
    use games::card::Card;
    fn card(s: &str) -> Card {
        str_to_card(s).unwrap()
    }

    #[test]
    fn test_ratio_of_lucky_pair() {
        let d4 = card("D4");
        let c4 = card("C4");
        let h4 = card("H4");
        let c5 = card("C5");
        let s5 = card("S5");
        let d7 = card("D7");
        assert_eq!(ratio_of_lucky_pair(&d4, &d4), Some(31.0));
        assert_eq!(ratio_of_lucky_pair(&d4, &c4), Some(16.0));
        assert_eq!(ratio_of_lucky_pair(&h4, &c4), Some(16.0));
        assert_eq!(ratio_of_lucky_pair(&h4, &h4), Some(16.0));
        assert_eq!(ratio_of_lucky_pair(&c5, &c5), Some(10.0));
        assert_eq!(ratio_of_lucky_pair(&s5, &c5), Some(10.0));
        assert_eq!(ratio_of_lucky_pair(&d7, &d7), Some(13.0));
        assert_eq!(ratio_of_lucky_pair(&d4, &d7), None);
    }

    #[test]
    fn test_wins_on() {
        assert_eq!(wins_on(Result::Banker(1)), (BankerWinsOn123, 32.0));
        assert_eq!(wins_on(Result::Banker(2)), (BankerWinsOn123, 32.0));
        assert_eq!(wins_on(Result::Banker(3)), (BankerWinsOn123, 32.0));

        assert_eq!(wins_on(Result::Banker(4)), (BankerWinsOn456, 7.0));
        assert_eq!(wins_on(Result::Banker(5)), (BankerWinsOn456, 7.0));
        assert_eq!(wins_on(Result::Banker(6)), (BankerWinsOn456, 7.0));

        assert_eq!(wins_on(Result::Banker(7)), (BankerWinsOn789, 3.0));
        assert_eq!(wins_on(Result::Banker(8)), (BankerWinsOn789, 3.0));
        assert_eq!(wins_on(Result::Banker(9)), (BankerWinsOn789, 3.0));

        assert_eq!(wins_on(Result::Player(1)), (PlayerWinsOn123, 32.0));
        assert_eq!(wins_on(Result::Player(2)), (PlayerWinsOn123, 32.0));
        assert_eq!(wins_on(Result::Player(3)), (PlayerWinsOn123, 32.0));

        assert_eq!(wins_on(Result::Player(4)), (PlayerWinsOn456, 9.0));
        assert_eq!(wins_on(Result::Player(5)), (PlayerWinsOn456, 9.0));
        assert_eq!(wins_on(Result::Player(6)), (PlayerWinsOn456, 9.0));

        assert_eq!(wins_on(Result::Player(7)), (PlayerWinsOn789, 3.0));
        assert_eq!(wins_on(Result::Player(8)), (PlayerWinsOn789, 3.0));
        assert_eq!(wins_on(Result::Player(9)), (PlayerWinsOn789, 3.0));

        assert_eq!(wins_on(Result::Tie(0)), (TieOn0123, 46.0));
        assert_eq!(wins_on(Result::Tie(1)), (TieOn0123, 46.0));
        assert_eq!(wins_on(Result::Tie(2)), (TieOn0123, 46.0));
        assert_eq!(wins_on(Result::Tie(3)), (TieOn0123, 46.0));

        assert_eq!(wins_on(Result::Tie(4)), (TieOn456, 25.0));
        assert_eq!(wins_on(Result::Tie(5)), (TieOn456, 25.0));
        assert_eq!(wins_on(Result::Tie(6)), (TieOn456, 25.0));

        assert_eq!(wins_on(Result::Tie(7)), (TieOn789, 20.0));
        assert_eq!(wins_on(Result::Tie(8)), (TieOn789, 20.0));
        assert_eq!(wins_on(Result::Tie(9)), (TieOn789, 20.0));
    }

    #[test]
    fn test_valid_bets() {
        let b = LuckyBaccarat::new();

        let all = b.valid_bets(40);
        assert_eq!(19, all.len());
        assert_eq!(*all, all_bets());

        let after40 = b.valid_bets(41);
        assert_eq!(9, after40.len());
        assert_eq!(false, after40.contains(&BankerWinsOn123));
        assert_eq!(false, after40.contains(&BankerWinsOn456));
        assert_eq!(false, after40.contains(&BankerWinsOn789));
        assert_eq!(false, after40.contains(&PlayerWinsOn123));
        assert_eq!(false, after40.contains(&PlayerWinsOn456));
        assert_eq!(false, after40.contains(&PlayerWinsOn789));
        assert_eq!(false, after40.contains(&TieOn0123));
        assert_eq!(false, after40.contains(&TieOn456));
        assert_eq!(false, after40.contains(&TieOn789));
        assert_eq!(false, after40.contains(&Lucky6));


        let after40 = b.valid_bets(60);
        assert_eq!(9, after40.len());
        assert_eq!(false, after40.contains(&BankerWinsOn123));
        assert_eq!(false, after40.contains(&BankerWinsOn456));
        assert_eq!(false, after40.contains(&BankerWinsOn789));
        assert_eq!(false, after40.contains(&PlayerWinsOn123));
        assert_eq!(false, after40.contains(&PlayerWinsOn456));
        assert_eq!(false, after40.contains(&PlayerWinsOn789));
        assert_eq!(false, after40.contains(&TieOn0123));
        assert_eq!(false, after40.contains(&TieOn456));
        assert_eq!(false, after40.contains(&TieOn789));
        assert_eq!(false, after40.contains(&Lucky6));

        let after60 = b.valid_bets(61);
        assert_eq!(3, after60.len());
        assert_eq!(false, after60.contains(&BankerBlack));
        assert_eq!(false, after60.contains(&BankerRed));
        assert_eq!(false, after60.contains(&BankerLuckyPair));
        assert_eq!(false, after60.contains(&PlayerBlack));
        assert_eq!(false, after60.contains(&PlayerRed));
        assert_eq!(false, after60.contains(&PlayerLuckyPair));
    }

    #[test]
    fn test_result_payout_map_tie(){
        let r = result_payout_map(Result::Tie(0));
        assert_eq!(r, hashmap!{TieOn0123 => 46.0, Tie =>9.0, Player => 1.0, Banker => 1.0});
        let r = result_payout_map(Result::Tie(1));
        assert_eq!(r, hashmap!{TieOn0123 => 46.0, Tie =>9.0, Player => 1.0, Banker => 1.0});
        let r = result_payout_map(Result::Tie(2));
        assert_eq!(r, hashmap!{TieOn0123 => 46.0, Tie =>9.0, Player => 1.0, Banker => 1.0});
        let r = result_payout_map(Result::Tie(3));
        assert_eq!(r, hashmap!{TieOn0123 => 46.0, Tie =>9.0, Player => 1.0, Banker => 1.0});
        let r = result_payout_map(Result::Tie(4));
        assert_eq!(r, hashmap!{TieOn456 => 25.0, Tie =>9.0, Player => 1.0, Banker => 1.0});
        let r = result_payout_map(Result::Tie(5));
        assert_eq!(r, hashmap!{TieOn456 => 25.0, Tie =>9.0, Player => 1.0, Banker => 1.0});
        let r = result_payout_map(Result::Tie(6));
        assert_eq!(r, hashmap!{TieOn456 => 25.0, Tie =>9.0, Player => 1.0, Banker => 1.0, Lucky6 =>7.0});
        let r = result_payout_map(Result::Tie(7));
        assert_eq!(r, hashmap!{TieOn789 => 20.0, Tie =>9.0, Player => 1.0, Banker => 1.0});
        let r = result_payout_map(Result::Tie(8));
        assert_eq!(r, hashmap!{TieOn789 => 20.0, Tie =>9.0, Player => 1.0, Banker => 1.0});
        let r = result_payout_map(Result::Tie(9));
        assert_eq!(r, hashmap!{TieOn789 => 20.0, Tie =>9.0, Player => 1.0, Banker => 1.0});
    }

    #[test]
    fn test_result_payout_map_banker(){
        let r = result_payout_map(Result::Banker(1));
        assert_eq!(r, hashmap!{BankerWinsOn123 => 32.0, Banker => 2.0});

        let r = result_payout_map(Result::Banker(2));
        assert_eq!(r, hashmap!{BankerWinsOn123 => 32.0, Banker => 2.0});

        let r = result_payout_map(Result::Banker(3));
        assert_eq!(r, hashmap!{BankerWinsOn123 => 32.0, Banker => 2.0});

        let r = result_payout_map(Result::Banker(4));
        assert_eq!(r, hashmap!{BankerWinsOn456 => 7.0, Banker => 2.0});
        
        let r = result_payout_map(Result::Banker(5));
        assert_eq!(r, hashmap!{BankerWinsOn456 => 7.0, Banker => 2.0});
        
        let r = result_payout_map(Result::Banker(6));
        assert_eq!(r, hashmap!{BankerWinsOn456 => 7.0, Banker => 1.5, Lucky6 => 7.0});
        
        let r = result_payout_map(Result::Banker(7));
        assert_eq!(r, hashmap!{BankerWinsOn789 => 3.0, Banker => 2.0});
        
        let r = result_payout_map(Result::Banker(8));
        assert_eq!(r, hashmap!{BankerWinsOn789 => 3.0, Banker => 2.0});

        let r = result_payout_map(Result::Banker(9));
        assert_eq!(r, hashmap!{BankerWinsOn789 => 3.0, Banker => 2.0});
    }

     #[test]
    fn test_result_payout_map_player(){
        let r = result_payout_map(Result::Player(1));
        assert_eq!(r, hashmap!{PlayerWinsOn123 => 32.0, Player => 2.0});

        let r = result_payout_map(Result::Player(2));
        assert_eq!(r, hashmap!{PlayerWinsOn123 => 32.0, Player => 2.0});

        let r = result_payout_map(Result::Player(3));
        assert_eq!(r, hashmap!{PlayerWinsOn123 => 32.0, Player => 2.0});

        let r = result_payout_map(Result::Player(4));
        assert_eq!(r, hashmap!{PlayerWinsOn456 => 9.0, Player => 2.0});

        let r = result_payout_map(Result::Player(5));
        assert_eq!(r, hashmap!{PlayerWinsOn456 => 9.0, Player => 2.0});

        let r = result_payout_map(Result::Player(6));
        assert_eq!(r, hashmap!{PlayerWinsOn456 => 9.0, Player => 2.0, Lucky6 => 7.0});

        let r = result_payout_map(Result::Player(7));
        assert_eq!(r, hashmap!{PlayerWinsOn789 => 3.0, Player => 2.0});

        let r = result_payout_map(Result::Player(8));
        assert_eq!(r, hashmap!{PlayerWinsOn789 => 3.0, Player => 2.0});
        
        let r = result_payout_map(Result::Player(9));
        assert_eq!(r, hashmap!{PlayerWinsOn789 => 3.0, Player => 2.0});
    }
}
