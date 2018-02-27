use std::collections::HashMap;
use super::{Card, Rank};

fn value_of_card(card: &Card) -> u8 {
    match card.rank {
        Rank::Ace => 1,
        Rank::Two => 2,
        Rank::Three => 3,
        Rank::Four => 4,
        Rank::Five => 5,
        Rank::Six => 6,
        Rank::Seven => 7,
        Rank::Eight => 8,
        Rank::Nine => 9,
        Rank::Ten => 10,
        Rank::Jack => 11,
        Rank::Queen => 12,
        Rank::King => 13,
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Bets {
    Dragon,
    Tiger,
    Tie,
    DragonOdd,
    DragonEven,
    TigerOdd,
    TigerEven,
}

#[derive(Debug)]
pub struct DragonTiger {
    dragon_card: Card,
    tiger_card: Card,
}

impl DragonTiger {
    fn result(&self) -> (Result, u8, u8) {
        let d = value_of_card(&self.dragon_card);
        let t = value_of_card(&self.tiger_card);
        if d > t {
            (Result::Dragon, d, t)
        } else if d < t {
            (Result::Tiger, d, t)
        } else {
            (Result::Tie, d, t)
        }
    }
}

fn parity(t: u8) -> Parity {
    if t % 2 == 0 {
        Parity::Even
    } else {
        if t == 7 {
            Parity::None
        } else {
            Parity::Odd
        }
    }
}

enum Parity {
    Odd,
    Even,
    None,
}

enum Result {
    Dragon,
    Tiger,
    Tie,
}

pub fn payout_map(b: &DragonTiger) -> HashMap<Bets, f64> {
    let mut map = HashMap::<Bets, f64>::new();
    let (r, d, t) = b.result();
    match r {
        Result::Dragon => {
            map.insert(Bets::Dragon, 2.0);
        }
        Result::Tiger => {
            map.insert(Bets::Tiger, 2.0);
        }
        Result::Tie => {
            map.insert(Bets::Tie, 9.0);
            map.insert(Bets::Dragon, 1.0);
            map.insert(Bets::Tiger, 1.0);
        }
    }
   add_odd_even(d, Bets::DragonOdd,Bets::DragonEven,&mut map);
   add_odd_even(t, Bets::TigerOdd,Bets::TigerEven,&mut map);
   map
}

fn add_odd_even(n:u8, odd:Bets, even:Bets, map:&mut HashMap<Bets, f64>){
    match parity(n) {
        Parity::Odd => {
            map.insert(odd, 2.0);
        }
        Parity::Even => {
            map.insert(even, 2.0);
        }
        Parity::None => {}
    }
}