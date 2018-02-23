pub mod straight;
pub mod split;
pub mod street;
pub mod corner;
pub mod line;
pub mod simple;

pub mod all;

use std::collections::HashMap;
use super::{BetId, Ratio};

pub trait BetKind: BetId + Ratio {
    fn bingo(&self, d: u8) -> bool;
}

fn add(b: Box<BetKind>, map: &mut HashMap<u16, Box<BetKind>>)
{
    map.insert(b.id(), b);
}
