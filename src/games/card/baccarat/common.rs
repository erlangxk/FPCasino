use super::dealer::init_baccarat_dealer;
use super::total_points;
use games::card::Card;

#[derive(Debug)]
pub struct Baccarat {
    banker_cards: Vec<Card>,
    player_cards: Vec<Card>,
}

fn first2(cards: &Vec<Card>) -> (Card, Card) {
    (cards[0], cards[1])
}

impl Baccarat {
    pub fn banker_total_points(&self) -> u8 {
        total_points(&self.banker_cards)
    }

    pub fn player_total_points(&self) -> u8 {
        total_points(&self.player_cards)
    }

    pub fn is_banker_pair(&self) -> bool {
        let f2 = first2(&self.banker_cards);
        Card::is_same_rank(f2.0, f2.1)
    }

    pub fn is_player_pair(&self) -> bool {
        let f2 = first2(&self.player_cards);
        Card::is_same_rank(f2.0, f2.1)
    }

    pub fn banker_total_cards(&self) -> usize {
        self.banker_cards.len()
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
    fn card(s: &str) -> Card {
        Card::from_str(s).unwrap()
    }

    #[test]
    fn test_baccarat_from_cards() {
        let cards = vec![card("ST"), card("S9"), card("H2"), card("DQ")];
        let result = Baccarat::from(&cards).unwrap();
        assert_eq!(9, result.banker_total_points());
        assert_eq!(2, result.player_total_points());
        assert_eq!(false, result.is_banker_pair());
        assert_eq!(false, result.is_player_pair());
        assert_eq!(2, result.banker_total_cards());
    }
}