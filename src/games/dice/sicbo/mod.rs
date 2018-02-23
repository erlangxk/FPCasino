pub mod single;
pub mod double;
pub mod triple;
pub mod total;
pub mod pair;
pub mod simple;
pub mod all;

use std::collections::HashMap;
use super::{BetId, Ratio};
pub struct Result {
    d1: u8,
    d2: u8,
    d3: u8,
    is_triple: bool,
    sum: u8,
}

impl Result {
    fn new(d1: u8, d2: u8, d3: u8) -> Result {
        Result {
            d1,
            d2,
            d3,
            is_triple: d1 == d2 && d2 == d3,
            sum: d1 + d2 + d3,
        }
    }
}

#[inline]
fn count(b: bool) -> u8 {
    if b {
        1
    } else {
        0
    }
}

pub trait BetKind: BetId + Ratio {
    fn bingo(&self, r: &Result) -> u8;
}

fn add(b: Box<BetKind>, map: &mut HashMap<u16, Box<BetKind>>) {
    map.insert(b.id(), b);
}
