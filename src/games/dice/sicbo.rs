use std::collections::{HashMap,HashSet};
use games::BetSerde;

#[derive(Hash, PartialEq, Eq, Debug,Copy,Clone)]
enum Bets {
    Big,
    Small,
    Odd,
    Even,
    Single(u8),
    Double(u8),
    Triple(u8),
    AnyTriple,
    Total(u8),
    Pair(u8, u8),
}

use self::Bets::*;

impl BetSerde for Bets {
    fn to_u16(&self) -> u16 {
        match *self {
            Big => 1,
            Small => 2,
            Odd => 3,
            Even => 4,
            Single(1) => 11,
            Single(2) => 12,
            Single(3) => 13,
            Single(4) => 14,
            Single(5) => 15,
            Single(6) => 16,
            Single(_) =>unreachable!(),
            Double(1) => 21,
            Double(2) => 22,
            Double(3) => 23,
            Double(4) => 24,
            Double(5) => 25,
            Double(6) => 26,
            Double(_) => unreachable!(),
            Triple(1) => 31,
            Triple(2) => 32,
            Triple(3) => 33,
            Triple(4) => 34,
            Triple(5) => 35,
            Triple(6) => 36,
            Triple(_) => unreachable!(),
            AnyTriple => 37,
            Total(4) => 104,
            Total(5) => 105,
            Total(6) => 106,
            Total(7) => 107,
            Total(8) => 108,
            Total(9) => 109,
            Total(10) => 110,
            Total(11) => 111,
            Total(12) => 112,
            Total(13) => 113,
            Total(14) => 114,
            Total(15) => 115,
            Total(16) => 116,
            Total(17) => 117,
            Total(_) => unreachable!(),
            Pair(1, 2) => 212,
            Pair(1, 3) => 213,
            Pair(1, 4) => 214,
            Pair(1, 5) => 215,
            Pair(1, 6) => 216,
            Pair(2, 3) => 223,
            Pair(2, 4) => 224,
            Pair(2, 5) => 225,
            Pair(2, 6) => 226,
            Pair(3, 4) => 234,
            Pair(3, 5) => 235,
            Pair(3, 6) => 236,
            Pair(4, 5) => 245,
            Pair(4, 6) => 246,
            Pair(5, 6) => 256,
            Pair(_,_)=> unreachable!(),
        }
    }

    fn from_u16(id: u16) -> Option<Bets> {
        match id {
            1 => Some(Big),
            2 => Some(Small),
            3 => Some(Odd),
            4 => Some(Even),
            11 => Some(Single(1)),
            12 => Some(Single(2)),
            13 => Some(Single(3)),
            14 => Some(Single(4)),
            15 => Some(Single(5)),
            16 => Some(Single(6)),
            21 => Some(Double(1)),
            22 => Some(Double(2)),
            23 => Some(Double(3)),
            24 => Some(Double(4)),
            25 => Some(Double(5)),
            26 => Some(Double(6)),
            31 => Some(Triple(1)),
            32 => Some(Triple(2)),
            33 => Some(Triple(3)),
            34 => Some(Triple(4)),
            35 => Some(Triple(5)),
            36 => Some(Triple(6)),
            37 => Some(AnyTriple),
            104 => Some(Total(4)),
            105 => Some(Total(5)),
            106 => Some(Total(6)),
            107 => Some(Total(7)),
            108 => Some(Total(8)),
            109 => Some(Total(9)),
            110 => Some(Total(10)),
            111 => Some(Total(11)),
            112 => Some(Total(12)),
            113 => Some(Total(13)),
            114 => Some(Total(14)),
            115 => Some(Total(15)),
            116 => Some(Total(16)),
            117 => Some(Total(17)),
            212 => Some(Pair(1,2)),
            213 => Some(Pair(1,3)),
            214 => Some(Pair(1,4)),
            215 => Some(Pair(1,5)),
            216 => Some(Pair(1,6)),
            223 => Some(Pair(2,3)),
            224 => Some(Pair(2,4)),
            225 => Some(Pair(2,5)),
            226 => Some(Pair(2,6)),
            234 => Some(Pair(3,4)),
            235 => Some(Pair(3,5)),
            236 => Some(Pair(3,6)),
            245 => Some(Pair(4,5)),
            246 => Some(Pair(4,6)),
            256 => Some(Pair(5,6)),
            _ => None,
        }
    }
}

#[inline]
fn count(b:bool)-> u8 {
    if b {1} else {0}
}

impl Bets {
    fn result(&self, d1:u8, d2:u8,d3:u8)-> (f64, u8) {
        let sum = d1 + d2 + d3;
        let is_triple = d1 == d2 && d2==d3;
        match *self {
            Big => (1.0, count(sum >=11 && sum<=17 && !is_triple)),
            Small => (1.0, count(sum>=4 && sum<=10 && !is_triple)),
            Even => (1.0, count(sum %2 ==0 && !is_triple)),
            Odd =>(1.0, count(sum %2 !=0 && !is_triple)),
            Single(n) => (1.0, count(d1 == n) + count(d2==n) + count(d3==n)),
            Double(n) => (8.0, count((d1==n && d2 ==n) || (d1==n && d3==n) ||(d2==n && d3==n))),
            Triple(n) => (150.0, count(is_triple && d1 ==n)),
            AnyTriple => (24.0, count(is_triple)),
            Total(n) => {
                let r = match n {
                    4 | 17 => 50.0,
                    5 | 16 => 18.0,
                    6 | 15 => 14.0,
                    7 | 14 => 12.0,
                    8 | 13 => 8.0,
                    9 | 10 | 11 | 12 => 6.0,
                    _ => unreachable!(),
                };
                (r, count(n == sum))
            }
            Pair(a,b) => (5.0, count((d1 == a || d2==a || d3==a) && (d1==b || d2==b ||d3 == b))),
        }
    }
}

fn all_bets()-> HashSet<Bets>{
    hashset!{
        Big,Small,Odd,Even,
        Single(1),Single(2),Single(3),Single(4),Single(5),Single(6),
        Double(1),Double(2),Double(3),Double(4),Double(5),Double(6),
        Triple(1),Triple(2),Triple(3),Triple(4),Triple(5),Triple(6),AnyTriple,
        Total(4),Total(5),Total(6),Total(7),Total(8),Total(9),Total(10),Total(11),Total(12),Total(13),Total(14),Total(15),Total(16),Total(17),
        Pair(1,2),Pair(1,3),Pair(1,4),Pair(1,5),Pair(1,6),Pair(2,3),Pair(2,4),Pair(2,5),Pair(2,6),Pair(3,4),Pair(3,5),Pair(3,6),Pair(4,5),Pair(4,6),Pair(5,6),
    }
}

struct Sicbo {
    all_bets:HashSet<Bets>,
}

impl Sicbo {
    fn payout_map(&self,d1:u8, d2:u8, d3:u8)-> HashMap<Bets, f64> {
        let mut map = HashMap::<Bets,f64>::new();
        for b in self.all_bets.iter().cloned() {
            let (r, t) = b.result(d1,d2,d3);
            if t>0 {
                map.insert(b, 1.0 + r);
            }
        }
        map
    }
}
