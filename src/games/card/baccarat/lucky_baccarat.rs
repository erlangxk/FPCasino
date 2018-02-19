use std::collections::{HashMap, HashSet};
use super::common::{Baccarat, Result};
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
        Bets::Banker,Bets::Player,Bets::Tie,
        Bets::BankerBlack, Bets::BankerRed, Bets::BankerLuckyPair,
        Bets::PlayerBlack, Bets::PlayerRed, Bets::PlayerLuckyPair,
        Bets::BankerWinsOn123, Bets::BankerWinsOn456, Bets::BankerWinsOn789,
        Bets::PlayerWinsOn123, Bets::PlayerWinsOn456, Bets::PlayerWinsOn789,
        Bets::TieOn0123, Bets::TieOn456,Bets::TieOn789,
        Bets::Lucky6,
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
    hashset!{ Bets::Banker,Bets::Player,Bets::Tie }
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
    let mut map = result_payout_map(b.result2());
    side_bet(
        b.banker_first2(),
        Bets::BankerBlack,
        Bets::BankerRed,
        Bets::BankerLuckyPair,
        &mut map,
    );
    side_bet(
        b.player_first2(),
        Bets::PlayerBlack,
        Bets::PlayerRed,
        Bets::PlayerLuckyPair,
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
        map.insert(Bets::Lucky6, 7.0);
    }
    match result {
        Result::Tie(_) => {
            map.insert(Bets::Tie, 9.0);
            map.insert(Bets::Banker, 1.0);
            map.insert(Bets::Player, 1.0);
        }
        Result::Banker(6) => {
            map.insert(Bets::Banker, 1.5);
        }
        Result::Banker(_) => {
            map.insert(Bets::Banker, 2.0);
        }
        _ => {
            map.insert(Bets::Player, 2.0);
        }
    }
    let (bets, ratio) = wins_on(result);
    map.insert(bets, ratio);
    map
}

fn wins_on(result: Result) -> (Bets, f64) {
    match result {
        Result::Banker(1...3) => (Bets::BankerWinsOn123, 32.0),
        Result::Banker(4...6) => (Bets::BankerWinsOn456, 7.0),
        Result::Banker(_) => (Bets::BankerWinsOn789, 3.0),
        Result::Player(1...3) => (Bets::PlayerWinsOn123, 32.0),
        Result::Player(4...6) => (Bets::PlayerWinsOn456, 9.0),
        Result::Player(_) => (Bets::PlayerWinsOn789, 3.0),
        Result::Tie(0...3) => (Bets::TieOn0123, 46.0),
        Result::Tie(4...6) => (Bets::TieOn456, 25.0),
        Result::Tie(_) => (Bets::TieOn789, 20.0),
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
        assert_eq!(wins_on(Result::Banker(1)), (Bets::BankerWinsOn123, 32.0));
        assert_eq!(wins_on(Result::Banker(2)), (Bets::BankerWinsOn123, 32.0));
        assert_eq!(wins_on(Result::Banker(3)), (Bets::BankerWinsOn123, 32.0));

        assert_eq!(wins_on(Result::Banker(4)), (Bets::BankerWinsOn456, 7.0));
        assert_eq!(wins_on(Result::Banker(5)), (Bets::BankerWinsOn456, 7.0));
        assert_eq!(wins_on(Result::Banker(6)), (Bets::BankerWinsOn456, 7.0));

        assert_eq!(wins_on(Result::Banker(7)), (Bets::BankerWinsOn789, 3.0));
        assert_eq!(wins_on(Result::Banker(8)), (Bets::BankerWinsOn789, 3.0));
        assert_eq!(wins_on(Result::Banker(9)), (Bets::BankerWinsOn789, 3.0));

        assert_eq!(wins_on(Result::Player(1)), (Bets::PlayerWinsOn123, 32.0));
        assert_eq!(wins_on(Result::Player(2)), (Bets::PlayerWinsOn123, 32.0));
        assert_eq!(wins_on(Result::Player(3)), (Bets::PlayerWinsOn123, 32.0));

        assert_eq!(wins_on(Result::Player(4)), (Bets::PlayerWinsOn456, 9.0));
        assert_eq!(wins_on(Result::Player(5)), (Bets::PlayerWinsOn456, 9.0));
        assert_eq!(wins_on(Result::Player(6)), (Bets::PlayerWinsOn456, 9.0));

        assert_eq!(wins_on(Result::Player(7)), (Bets::PlayerWinsOn789, 3.0));
        assert_eq!(wins_on(Result::Player(8)), (Bets::PlayerWinsOn789, 3.0));
        assert_eq!(wins_on(Result::Player(9)), (Bets::PlayerWinsOn789, 3.0));

        assert_eq!(wins_on(Result::Tie(0)), (Bets::TieOn0123, 46.0));
        assert_eq!(wins_on(Result::Tie(1)), (Bets::TieOn0123, 46.0));
        assert_eq!(wins_on(Result::Tie(2)), (Bets::TieOn0123, 46.0));
        assert_eq!(wins_on(Result::Tie(3)), (Bets::TieOn0123, 46.0));

        assert_eq!(wins_on(Result::Tie(4)), (Bets::TieOn456, 25.0));
        assert_eq!(wins_on(Result::Tie(5)), (Bets::TieOn456, 25.0));
        assert_eq!(wins_on(Result::Tie(6)), (Bets::TieOn456, 25.0));

        assert_eq!(wins_on(Result::Tie(7)), (Bets::TieOn789, 20.0));
        assert_eq!(wins_on(Result::Tie(8)), (Bets::TieOn789, 20.0));
        assert_eq!(wins_on(Result::Tie(9)), (Bets::TieOn789, 20.0));
    }

    #[test]
    fn test_valid_bets() {
        let b = LuckyBaccarat::new();

        let all = b.valid_bets(40);
        assert_eq!(19, all.len());
        assert_eq!(*all, all_bets());

        let after40 = b.valid_bets(41);
        assert_eq!(9, after40.len());
        assert_eq!(false, after40.contains(&Bets::BankerWinsOn123));
        assert_eq!(false, after40.contains(&Bets::BankerWinsOn456));
        assert_eq!(false, after40.contains(&Bets::BankerWinsOn789));
        assert_eq!(false, after40.contains(&Bets::PlayerWinsOn123));
        assert_eq!(false, after40.contains(&Bets::PlayerWinsOn456));
        assert_eq!(false, after40.contains(&Bets::PlayerWinsOn789));
        assert_eq!(false, after40.contains(&Bets::TieOn0123));
        assert_eq!(false, after40.contains(&Bets::TieOn456));
        assert_eq!(false, after40.contains(&Bets::TieOn789));
        assert_eq!(false, after40.contains(&Bets::Lucky6));


        let after40 = b.valid_bets(60);
        assert_eq!(9, after40.len());
        assert_eq!(false, after40.contains(&Bets::BankerWinsOn123));
        assert_eq!(false, after40.contains(&Bets::BankerWinsOn456));
        assert_eq!(false, after40.contains(&Bets::BankerWinsOn789));
        assert_eq!(false, after40.contains(&Bets::PlayerWinsOn123));
        assert_eq!(false, after40.contains(&Bets::PlayerWinsOn456));
        assert_eq!(false, after40.contains(&Bets::PlayerWinsOn789));
        assert_eq!(false, after40.contains(&Bets::TieOn0123));
        assert_eq!(false, after40.contains(&Bets::TieOn456));
        assert_eq!(false, after40.contains(&Bets::TieOn789));
        assert_eq!(false, after40.contains(&Bets::Lucky6));

        let after60 = b.valid_bets(61);
        assert_eq!(3, after60.len());
        assert_eq!(false, after60.contains(&Bets::BankerBlack));
        assert_eq!(false, after60.contains(&Bets::BankerRed));
        assert_eq!(false, after60.contains(&Bets::BankerLuckyPair));
        assert_eq!(false, after60.contains(&Bets::PlayerBlack));
        assert_eq!(false, after60.contains(&Bets::PlayerRed));
        assert_eq!(false, after60.contains(&Bets::PlayerLuckyPair));
    }

    #[test]
    fn test_result_payout_map_tie(){
        let r = result_payout_map(Result::Tie(0));
        assert_eq!(r, hashmap!{Bets::TieOn0123 => 46.0, Bets::Tie =>9.0, Bets::Player => 1.0, Bets::Banker => 1.0});
        let r = result_payout_map(Result::Tie(1));
        assert_eq!(r, hashmap!{Bets::TieOn0123 => 46.0, Bets::Tie =>9.0, Bets::Player => 1.0, Bets::Banker => 1.0});
        let r = result_payout_map(Result::Tie(2));
        assert_eq!(r, hashmap!{Bets::TieOn0123 => 46.0, Bets::Tie =>9.0, Bets::Player => 1.0, Bets::Banker => 1.0});
        let r = result_payout_map(Result::Tie(3));
        assert_eq!(r, hashmap!{Bets::TieOn0123 => 46.0, Bets::Tie =>9.0, Bets::Player => 1.0, Bets::Banker => 1.0});
        let r = result_payout_map(Result::Tie(4));
        assert_eq!(r, hashmap!{Bets::TieOn456 => 25.0, Bets::Tie =>9.0, Bets::Player => 1.0, Bets::Banker => 1.0});
        let r = result_payout_map(Result::Tie(5));
        assert_eq!(r, hashmap!{Bets::TieOn456 => 25.0, Bets::Tie =>9.0, Bets::Player => 1.0, Bets::Banker => 1.0});
        let r = result_payout_map(Result::Tie(6));
        assert_eq!(r, hashmap!{Bets::TieOn456 => 25.0, Bets::Tie =>9.0, Bets::Player => 1.0, Bets::Banker => 1.0, Bets::Lucky6 =>7.0});
        let r = result_payout_map(Result::Tie(7));
        assert_eq!(r, hashmap!{Bets::TieOn789 => 20.0, Bets::Tie =>9.0, Bets::Player => 1.0, Bets::Banker => 1.0});
        let r = result_payout_map(Result::Tie(8));
        assert_eq!(r, hashmap!{Bets::TieOn789 => 20.0, Bets::Tie =>9.0, Bets::Player => 1.0, Bets::Banker => 1.0});
        let r = result_payout_map(Result::Tie(9));
        assert_eq!(r, hashmap!{Bets::TieOn789 => 20.0, Bets::Tie =>9.0, Bets::Player => 1.0, Bets::Banker => 1.0});
    }

    #[test]
    fn test_result_payout_map_banker(){
        let r = result_payout_map(Result::Banker(1));
        assert_eq!(r, hashmap!{Bets::BankerWinsOn123 => 32.0, Bets::Banker => 2.0});

        let r = result_payout_map(Result::Banker(2));
        assert_eq!(r, hashmap!{Bets::BankerWinsOn123 => 32.0, Bets::Banker => 2.0});

        let r = result_payout_map(Result::Banker(3));
        assert_eq!(r, hashmap!{Bets::BankerWinsOn123 => 32.0, Bets::Banker => 2.0});

        let r = result_payout_map(Result::Banker(4));
        assert_eq!(r, hashmap!{Bets::BankerWinsOn456 => 7.0, Bets::Banker => 2.0});
        
        let r = result_payout_map(Result::Banker(5));
        assert_eq!(r, hashmap!{Bets::BankerWinsOn456 => 7.0, Bets::Banker => 2.0});
        
        let r = result_payout_map(Result::Banker(6));
        assert_eq!(r, hashmap!{Bets::BankerWinsOn456 => 7.0, Bets::Banker => 1.5, Bets::Lucky6 => 7.0});
        
        let r = result_payout_map(Result::Banker(7));
        assert_eq!(r, hashmap!{Bets::BankerWinsOn789 => 3.0, Bets::Banker => 2.0});
        
        let r = result_payout_map(Result::Banker(8));
        assert_eq!(r, hashmap!{Bets::BankerWinsOn789 => 3.0, Bets::Banker => 2.0});

        let r = result_payout_map(Result::Banker(9));
        assert_eq!(r, hashmap!{Bets::BankerWinsOn789 => 3.0, Bets::Banker => 2.0});
    }

     #[test]
    fn test_result_payout_map_player(){
        let r = result_payout_map(Result::Player(1));
        assert_eq!(r, hashmap!{Bets::PlayerWinsOn123 => 32.0, Bets::Player => 2.0});

        let r = result_payout_map(Result::Player(2));
        assert_eq!(r, hashmap!{Bets::PlayerWinsOn123 => 32.0, Bets::Player => 2.0});

        let r = result_payout_map(Result::Player(3));
        assert_eq!(r, hashmap!{Bets::PlayerWinsOn123 => 32.0, Bets::Player => 2.0});

        let r = result_payout_map(Result::Player(4));
        assert_eq!(r, hashmap!{Bets::PlayerWinsOn456 => 9.0, Bets::Player => 2.0});

        let r = result_payout_map(Result::Player(5));
        assert_eq!(r, hashmap!{Bets::PlayerWinsOn456 => 9.0, Bets::Player => 2.0});

        let r = result_payout_map(Result::Player(6));
        assert_eq!(r, hashmap!{Bets::PlayerWinsOn456 => 9.0, Bets::Player => 2.0, Bets::Lucky6 => 7.0});

        let r = result_payout_map(Result::Player(7));
        assert_eq!(r, hashmap!{Bets::PlayerWinsOn789 => 3.0, Bets::Player => 2.0});

        let r = result_payout_map(Result::Player(8));
        assert_eq!(r, hashmap!{Bets::PlayerWinsOn789 => 3.0, Bets::Player => 2.0});
        
        let r = result_payout_map(Result::Player(9));
        assert_eq!(r, hashmap!{Bets::PlayerWinsOn789 => 3.0, Bets::Player => 2.0});
    }

}
