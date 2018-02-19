use super::dealer::init_baccarat_dealer;
use super::{total_points, value_of_card};
use games::card::Card;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Result {
    Player(u8),
    Banker(u8),
    Tie(u8),
}

impl Result {
    pub fn total_points(&self) -> u8 {
        match *self {
            Result::Player(n) => n,
            Result::Banker(n) => n,
            Result::Tie(n) => n,
        }
    }
}

#[derive(Debug)]
pub struct Baccarat {
    banker_cards: Vec<Card>,
    player_cards: Vec<Card>,
}

fn first2(cards: &Vec<Card>) -> (Card, Card) {
    (cards[0], cards[1])
}

fn count(cards: &[Card], v: u8) -> usize {
    cards
        .iter()
        .fold(0, |a, c| if value_of_card(c) == v { a + 1 } else { a })
}

impl Baccarat {
    pub fn result(&self) -> Result {
        let tb = total_points(&self.banker_cards);
        let tp = total_points(&self.player_cards);
        if tb > tp {
            Result::Banker(tb)
        } else if tb < tp {
            Result::Player(tp)
        } else {
            Result::Tie(tb)
        }
    }

    pub fn banker_first2(&self) -> (Card, Card) {
        first2(&self.banker_cards)
    }

    pub fn player_first2(&self) -> (Card, Card) {
        first2(&self.player_cards)
    }

    pub fn banker_total_cards(&self) -> usize {
        self.banker_cards.len()
    }

    pub fn total_cards(&self) -> usize {
        self.banker_total_cards() + self.player_cards.len()
    }

    pub fn count_cards(&self, v: u8) -> usize {
        count(&self.banker_cards, v) + count(&self.player_cards, v)
    }

    pub fn from(cards: &Vec<Card>) -> Option<Baccarat> {
        let mut init = init_baccarat_dealer();
        for c in cards {
            let result = init.deal(*c);
            if !result {
                return None;
            }
        }
        if init.is_done() {
            return Some(Baccarat {
                banker_cards: init.banker_cards,
                player_cards: init.player_cards,
            });
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use games::card::serde::str_to_card;

    fn card(s: &str) -> Card {
        str_to_card(s).unwrap()
    }

    #[test]
    fn test_baccarat_from_cards() {
        let cards = vec![card("ST"), card("S9"), card("H2"), card("DQ")];
        let b = Baccarat::from(&cards).unwrap();
        assert_eq!(Result::Banker(9), b.result());
        assert_eq!(2, b.banker_total_cards());
    }
}
