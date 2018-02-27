use std::collections::HashMap;
use super::{double, pair, simple, single, total, triple, BetKind, Result};

struct Sicbo {
    all_bets: HashMap<u16, Box<BetKind>>,
}

impl Sicbo {
    fn new() -> Sicbo {
        let mut map = HashMap::<u16, Box<BetKind>>::new();
        simple::all_bets(&mut map);
        single::all_bets(&mut map);
        double::all_bets(&mut map);
        triple::all_bets(&mut map);
        pair::all_bets(&mut map);
        total::all_bets(&mut map);
        Sicbo { all_bets: map }
    }

    fn valid_bets(&self, bet_id: u16) -> bool {
        self.all_bets.contains_key(&bet_id)
    }

    pub fn payout_map(&self, d1: u8, d2: u8, d3: u8) -> HashMap<u16, f64> {
        let r = Result::new(d1, d2, d3);
        let mut map = HashMap::<u16, f64>::new();
        for (&id, b) in &self.all_bets {
            let c = b.bingo(&r);
            if c > 0 {
                map.insert(id, b.ratio() * (c as f64) + 1.0);
            }
        }
        map
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payout_map1() {
        let r = Sicbo::new().payout_map(1, 1, 1);
        assert_eq!(r, hashmap!(37=>25.0,31=>151.0,11=>4.0,21=>9.0));

        let r = Sicbo::new().payout_map(2, 2, 2);
        assert_eq!(r, hashmap!(37=>25.0,32=>151.0,12=>4.0,22=>9.0,106=>15.0));

        let r = Sicbo::new().payout_map(3, 3, 3);
        assert_eq!(r, hashmap!(37=>25.0,33=>151.0,13=>4.0,23=>9.0,109=>7.0));

        let r = Sicbo::new().payout_map(4, 4, 4);
        assert_eq!(r, hashmap!(37=>25.0,34=>151.0,14=>4.0,24=>9.0,112=>7.0));

        let r = Sicbo::new().payout_map(5, 5, 5);
        assert_eq!(r, hashmap!(37=>25.0,35=>151.0,15=>4.0,25=>9.0,115=>15.0));

        let r = Sicbo::new().payout_map(6, 6, 6);
        assert_eq!(r, hashmap!(37=>25.0,36=>151.0,16=>4.0,26=>9.0));
    }

    #[test]
    fn test_payout_map2() {
        let r = Sicbo::new().payout_map(1, 1, 2);
        assert_eq!(r, hashmap!(11=>3.0, 12=>2.0, 21=>9.0, 2=>2.0, 4=>2.0, 104=>51.0, 212=>6.0));
    }

    #[test]
    fn test_payout_map3() {
        let r = Sicbo::new().payout_map(1, 2, 6);
        assert_eq!(r, hashmap!(11=>2.0, 12=>2.0, 16=>2.0, 2=>2.0, 3=>2.0, 109=>7.0, 212=>6.0, 216=>6.0, 226=>6.0));
    }

    #[test]
    fn test_payout_map4() {
        let r = Sicbo::new().payout_map(6, 6, 6);
        assert_eq!(r, hashmap!(16=>4.0, 26=>9.0, 37=>25.0, 36=>151.0));
    }

    #[test]
    fn test_payout_map5() {
        let r = Sicbo::new().payout_map(4, 5, 2);
        assert_eq!(r, hashmap!(12=>2.0, 14=>2.0, 15=>2.0, 1=>2.0, 3=>2.0, 111=>7.0, 224=>6.0, 225=>6.0, 245=>6.0 ));
    }

    #[test]
    fn test_payout_map6() {
        let r = Sicbo::new().payout_map(6, 5, 6);
        assert_eq!(r, hashmap!(15=>2.0, 16=>3.0, 26=>9.0, 1=>2.0, 3=>2.0, 117=>51.0, 256=>6.0 ));
    }

    #[test]
    fn test_payout_map7() {
        let r = Sicbo::new().payout_map(1, 2, 3);
        assert_eq!(r, hashmap!(11=>2.0, 12=>2.0, 13=>2.0, 212=>6.0, 213=>6.0, 223=>6.0, 106=>15.0, 2=>2.0, 4=>2.0));
    
        let r = Sicbo::new().payout_map(1, 3, 4);
        assert_eq!(r, hashmap!(11=>2.0, 14=>2.0, 13=>2.0, 213=>6.0, 214=>6.0, 234=>6.0, 108=>9.0, 2=>2.0, 4=>2.0));

        let r = Sicbo::new().payout_map(1, 4, 5);
        assert_eq!(r, hashmap!(11=>2.0, 14=>2.0, 15=>2.0, 214=>6.0, 215=>6.0, 245=>6.0, 110=>7.0, 2=>2.0, 4=>2.0));

        let r = Sicbo::new().payout_map(1, 5, 6);
        assert_eq!(r, hashmap!(11=>2.0, 16=>2.0, 15=>2.0, 215=>6.0, 216=>6.0, 256=>6.0, 112=>7.0, 1=>2.0, 4=>2.0));

        let r = Sicbo::new().payout_map(2, 3, 4);
        assert_eq!(r, hashmap!(12=>2.0, 13=>2.0, 14=>2.0, 223=>6.0, 224=>6.0, 234=>6.0, 109=>7.0, 2=>2.0, 3=>2.0));

        let r = Sicbo::new().payout_map(2, 5, 6);
        assert_eq!(r, hashmap!(12=>2.0, 15=>2.0, 16=>2.0, 225=>6.0, 226=>6.0, 256=>6.0, 113=>9.0, 1=>2.0, 3=>2.0));

        let r = Sicbo::new().payout_map(3, 4, 5);
        assert_eq!(r, hashmap!(13=>2.0, 14=>2.0, 15=>2.0, 234=>6.0, 235=>6.0, 245=>6.0, 112=>7.0, 1=>2.0, 4=>2.0));

        let r = Sicbo::new().payout_map(3, 5, 6);
        assert_eq!(r, hashmap!(13=>2.0, 16=>2.0, 15=>2.0, 235=>6.0, 236=>6.0, 256=>6.0, 114=>13.0, 1=>2.0, 4=>2.0));

        let r = Sicbo::new().payout_map(4, 5, 6);
        assert_eq!(r, hashmap!(14=>2.0, 16=>2.0, 15=>2.0, 245=>6.0, 246=>6.0, 256=>6.0, 115=>15.0, 1=>2.0, 3=>2.0));

        let r = Sicbo::new().payout_map(5, 5, 6);
        assert_eq!(r, hashmap!(16=>2.0, 15=>3.0, 256=>6.0, 116=>19.0, 1=>2.0, 4=>2.0, 25=>9.0));
    }


}
