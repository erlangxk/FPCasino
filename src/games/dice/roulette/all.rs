use std::collections::HashMap;
use super::{corner, line, split, straight, street, simple, BetKind};

struct Roulette {
    all_bets: HashMap<u16, Box<BetKind>>,
}

impl Roulette {
    fn new() -> Roulette {
        let mut map = HashMap::<u16, Box<BetKind>>::new();
        simple::all_bets(&mut map);
        straight::all_bets(&mut map);
        split::all_bets(&mut map);
        street::all_bets(&mut map);
        corner::all_bets(&mut map);
        line::all_bets(&mut map);
        Roulette { all_bets: map }
    }

    fn valid_bets(&self, bet_id: u16) -> bool {
        self.all_bets.contains_key(&bet_id)
    }

    fn payout_map(&self, ball: u8) -> HashMap<u16, f64> {
        self.all_bets
            .iter()
            .filter(|&(_, b)| b.bingo(ball))
            .map(|(&id, b)| (id, b.ratio()+1.0))
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payout_map_0(){
        let g = Roulette::new();
        let r = g.payout_map(0);
        assert_eq!(r, hashmap!{100=>36.0, 200=>18.0, 201=>18.0, 202=>18.0, 300=>12.0, 301=>12.0, 422=>9.0,});
    }

    #[test]
    fn test_payout_map_1(){
        let g = Roulette::new();
        let r = g.payout_map(1);
        assert_eq!(r, hashmap!{1=>2.0, 3=>2.0, 6=>2.0, 7=>3.0, 10=>3.0, 101=>36.0, 200=>18.0, 203=>18.0, 236=>18.0, 300=>12.0, 302=>12.0, 400=>9.0, 422=>9.0, 600=>6.0});
    }

    #[test]
    fn test_payout_map_2(){
        let g = Roulette::new();
        let r = g.payout_map(2);
        assert_eq!(r, hashmap!{2=>2.0, 4=>2.0, 6=>2.0, 7=>3.0, 11=>3.0, 102=>36.0, 201=>18.0, 204=>18.0, 236=>18.0, 237=>18.0, 300=>12.0, 301=>12.0, 302=>12.0, 400=>9.0, 401=>9.0, 422=>9.0, 600=>6.0});
    }

    #[test]
    fn test_payout_map_3(){
        let g = Roulette::new();
        let r = g.payout_map(3);
        assert_eq!(r, hashmap!{1=>2.0, 3=>2.0, 6=>2.0, 7=>3.0, 12=>3.0, 103=>36.0, 202=>18.0, 205=>18.0, 237=>18.0, 301=>12.0, 302=>12.0, 401=>9.0, 422=>9.0, 600=>6.0});
    }
}


