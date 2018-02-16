use super::*;

pub fn char_to_suit(c: char) -> Option<Suit> {
    match c {
        'D' => Some(Suit::Diamond),
        'C' => Some(Suit::Club),
        'H' => Some(Suit::Heart),
        'S' => Some(Suit::Spade),
        _ => None,
    }
}

pub fn suit_to_char(suit: Suit) -> char {
    match suit {
        Suit::Diamond => 'D',
        Suit::Club => 'C',
        Suit::Heart => 'H',
        Suit::Spade => 'S',
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

pub fn str_to_card(s: &str) -> Option<Card> {
    let bs = s.as_bytes();
    if bs.len() == 2 {
        let sc: char = bs[0] as char;
        let rc: char = bs[1] as char;
        return char_to_suit(sc).and_then(|suit| char_to_rank(rc).map(|rank| Card { suit, rank }));
    }
    None
}

pub fn card_to_str(c: Card) -> String {
    let mut s = String::with_capacity(2);
    s.push(suit_to_char(c.suit));
    s.push(rank_to_char(c.rank));
    s
}
