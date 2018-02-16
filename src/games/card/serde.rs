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
        return two_char_to_card(sc,rc);
    }
    None
}

fn two_char_to_card(sc:char, rc:char)-> Option<Card> {
    char_to_suit(sc).and_then(|suit| char_to_rank(rc).map(|rank| Card { suit, rank }))
}

pub fn card_to_str(c: Card) -> String {
    let mut s = String::with_capacity(2);
    s.push(suit_to_char(c.suit));
    s.push(rank_to_char(c.rank));
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    static ALL_RANKS: &[Rank] = &[
        Rank::Ace,
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
    ];
    static ALL_SUITS: &[Suit] = &[Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

    #[test]
    fn test_suit_to_char() {
        assert_eq!('D', suit_to_char(Suit::Diamond));
        assert_eq!('C', suit_to_char(Suit::Club));
        assert_eq!('H', suit_to_char(Suit::Heart));
        assert_eq!('S', suit_to_char(Suit::Spade));
    }

    #[test]
    fn test_char_to_suit() {
        assert_eq!(char_to_suit('D'), Some(Suit::Diamond));
        assert_eq!(char_to_suit('C'), Some(Suit::Club));
        assert_eq!(char_to_suit('H'), Some(Suit::Heart));
        assert_eq!(char_to_suit('S'), Some(Suit::Spade));
        assert_eq!(char_to_suit('X'), None);
    }

    #[test]
    fn suit_serde() {
        assert_eq!(4, ALL_SUITS.len());
        let set: HashSet<Suit> = ALL_SUITS.iter().cloned().collect();
        assert_eq!(4, set.len());
        for r in ALL_SUITS {
            let c = suit_to_char(*r);
            assert_eq!(char_to_suit(c), Some(*r));
        }
    }

    #[test]
    fn rank_serde() {
        assert_eq!(13, ALL_RANKS.len());
        let set: HashSet<Rank> = ALL_RANKS.iter().cloned().collect();
        assert_eq!(13, set.len());
        for r in ALL_RANKS {
            let c = rank_to_char(*r);
            assert_eq!(char_to_rank(c), Some(*r));
        }
    }

    #[test]
    fn card_serde() {
        let mut all_cards: Vec<Card> = Vec::new();
        for s in ALL_SUITS {
            for r in ALL_RANKS {
                all_cards.push(Card {
                    suit: *s,
                    rank: *r,
                });
            }
        }
        assert_eq!(52, all_cards.len());
        for c in all_cards {
            let s = card_to_str(c);
            let r = str_to_card(&s);
            assert_eq!(r, Some(c));
        }
    }
}
