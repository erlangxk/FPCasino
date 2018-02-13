pub mod baccarat;

trait Value {
    fn value(&self) -> u8;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Suit {
    DIAMOND,
    CLUB,
    HEART,
    SPADE,
}

pub fn char_to_suit(c: char) -> Option<Suit> {
    match c {
        'D' => Some(Suit::DIAMOND),
        'C' => Some(Suit::CLUB),
        'H' => Some(Suit::HEART),
        'S' => Some(Suit::SPADE),
        _ => None,
    }
}

pub fn suit_to_char(suit: &Suit) -> char {
    match *suit {
        Suit::DIAMOND => 'D',
        Suit::CLUB => 'C',
        Suit::HEART => 'H',
        Suit::SPADE => 'S',
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

pub fn rank_to_value_1(rank: Rank) -> u8 {
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
        Rank::Ten => 0,
        Rank::Jack => 0,
        Rank::Queen => 0,
        Rank::King => 0,
    }
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

pub fn rank_to_char(rank: Rank) -> char {
    match rank {
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

#[derive(Clone, Copy,Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}



impl Card {
    pub fn new(s: char, r: char) -> Option<Card> {
        char_to_suit(s).and_then(|suit| char_to_rank(r).map(|rank| Card { suit, rank }))
    }

    pub fn is_same_rank(c1: Card, c2: Card) -> bool {
        c1.rank == c2.rank
    }
    
    pub fn is_same_suit(c1: Card, c2: Card) -> bool {
        c1.suit == c2.suit
    }
}
