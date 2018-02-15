pub mod baccarat;

trait Value {
    fn value(&self) -> u8;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Suit {
    Diamond,
    Club,
    Heart,
    Spade,
}

pub fn char_to_suit(c: char) -> Option<Suit> {
    match c {
        'D' => Some(Suit::Diamond),
        'C' => Some(Suit::Club),
        'H' => Some(Suit::Heart),
        'S' => Some(Suit::Spade),
        _ => None,
    }
}

impl Into<char> for Suit {
    fn into(self) -> char {
        match self {
            Suit::Diamond => 'D',
            Suit::Club => 'C',
            Suit::Heart => 'H',
            Suit::Spade => 'S',
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

pub fn rank_to_value_2(rank: Rank) -> u8 {
    match rank {
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

pub fn rank_to_value_3(rank: Rank) -> u8 {
    match rank {
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
        Rank::Jack => 10,
        Rank::Queen => 10,
        Rank::King => 10,
    }
}

impl Into<char> for Rank {
    fn into(self) -> char {
        match self {
            Rank::Ace => 'A',
            Rank::Two => '2',
            Rank::Three => '3',
            Rank::Four => '4',
            Rank::Five => '5',
            Rank::Six => '6',
            Rank::Seven => '7',
            Rank::Eight => '8',
            Rank::Nine => '9',
            Rank::Ten => 'T',
            Rank::Jack => 'J',
            Rank::Queen => 'Q',
            Rank::King => 'K',
        }
    }
}

pub fn char_to_rank(c: char) -> Option<Rank> {
    match c {
        'A' => Some(Rank::Ace),
        '2' => Some(Rank::Two),
        '3' => Some(Rank::Three),
        '4' => Some(Rank::Four),
        '5' => Some(Rank::Five),
        '6' => Some(Rank::Six),
        '7' => Some(Rank::Seven),
        '8' => Some(Rank::Eight),
        '9' => Some(Rank::Nine),
        'T' => Some(Rank::Ten),
        'J' => Some(Rank::Jack),
        'Q' => Some(Rank::Queen),
        'K' => Some(Rank::King),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}



impl Card {
    pub fn new(sc: char, rc: char) -> Option<Card> {
        char_to_suit(sc).and_then(|suit| char_to_rank(rc).map(|rank| Card { suit, rank }))
    }

    pub fn from_str(s: &str) -> Option<Card> {
        let bs = s.as_bytes();
        if bs.len() == 2 {
            let sc = bs[0];
            let rc = bs[1];
            return Card::new(sc as char, rc as char);
        }
        None
    }

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
