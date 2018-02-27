pub mod baccarat;
pub mod dragontiger;

pub mod serde;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Suit {
    Diamond,
    Club,
    Heart,
    Spade,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    pub fn is_same_rank(&self, c: &Card) -> bool {
        self.rank == c.rank
    }

    pub fn is_same_suit(&self, c: &Card) -> bool {
        self.suit == c.suit
    }

    pub fn is_diamond(self: &Card) -> bool {
        self.suit == Suit::Diamond
    }

    fn is_red(&self) -> bool {
        self.suit == Suit::Heart || self.suit == Suit::Diamond
    }

    fn is_black(&self) -> bool {
        !self.is_red()
    }
}
