use super::{Card, Rank};

pub fn value_of_card(card: &Card) -> u8 {
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
        Rank::Ten => 0,
        Rank::Jack => 0,
        Rank::Queen => 0,
        Rank::King => 0,
    }
}

fn total_points(cards: &Vec<Card>) -> u8 {
    cards.iter().fold(0, |a, c| a + value_of_card(c)) % 10
}

pub mod dealer;
pub mod common;
pub mod classical_baccarat;
pub mod fabulous_baccarat;
pub mod lucky_baccarat;
pub mod sevenup_baccarat;