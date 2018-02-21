pub mod card;
pub mod dice;

use std::collections::HashMap;
use std::hash::Hash;

pub trait BetSerde: Sized {
    fn from_u16(u16) -> Option<Self>;
    fn to_u16(&self) -> u16;
}

pub trait Game {
    type B: BetSerde + Eq + Hash;
    
    fn from_raw_bets(&self, bets: &HashMap<u16, f64>) -> Option<HashMap<Self::B, f64>> {
        let mut m = HashMap::<Self::B, f64>::new();
        for (k, v) in bets {
            if let Some(b) = Self::B::from_u16(*k) {
                m.insert(b, *v);
            } else {
                return None;
            }
        }
        Some(m)
    }
}
