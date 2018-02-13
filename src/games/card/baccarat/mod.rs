use super::{Card, Value, rank_to_value_1};

impl Value for Card {
    fn value(&self) -> u8 {
        rank_to_value_1(self.rank)
    }
}

fn total_points<T: Value>(cards: &Vec<T>) -> u8 {
    cards.iter().fold(0, |a, c| a + c.value()) % 10
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BaccaratStatus {
    ExpectPlayer,
    ExpectBanker,
    Done,
}

#[derive(Debug)]
pub struct BaccaratDealer {
    banker_cards: Vec<Card>,
    player_cards: Vec<Card>,
    status: BaccaratStatus,
}

impl BaccaratDealer {
    pub fn new() -> BaccaratDealer {
        BaccaratDealer {
            banker_cards: vec![],
            player_cards: vec![],
            status: BaccaratStatus::ExpectPlayer,
        }
    }

    pub fn test(cards: &Vec<Card>) -> bool {
        let mut init = BaccaratDealer::new();
        for c in cards {
            if !init.deal(*c) {
                return false;
            }
        }
        init.status == BaccaratStatus::Done
    }

    pub fn deal(&mut self, card: Card) -> bool {
        println!("cards is {:?}, current state is {:?}", card, &self);
        match self.status {
            BaccaratStatus::Done => false,

            BaccaratStatus::ExpectPlayer => {
                self.player_cards.push(card);
                match self.player_cards.len() {
                    1 | 2 => self.status = BaccaratStatus::ExpectBanker,
                    3 => {
                        let tb = total_points(&self.banker_cards);
                        let cv = card.value();
                        let stand3 = cv == 8;
                        let stand4 = stand3 || cv == 0 || cv == 1 || cv == 9;
                        let stand5 = stand4 || cv == 2 || cv == 3;
                        let stand6 = stand5 || cv == 4 || cv == 5;
                        if (stand3 && tb == 3) || (stand4 && tb == 4) || (stand5 && tb == 5)
                            || (stand6 && tb == 6) || tb ==7
                        {
                            self.status = BaccaratStatus::Done;
                        } else {
                            self.status = BaccaratStatus::ExpectBanker
                        }
                    }
                    _ => panic!("banker cards is more than 3 now"),
                }
                true
            }

            BaccaratStatus::ExpectBanker => {
                self.banker_cards.push(card);
                match self.banker_cards.len() {
                    1 => self.status = BaccaratStatus::ExpectPlayer,
                    2 => {
                        let tp = total_points(&self.player_cards);
                        let tb = total_points(&self.banker_cards);
                        if tp == 8 || tp == 9 || tb == 8 || tp == 9 {
                            self.status = BaccaratStatus::Done;
                        } else {
                            if tp <= 5 {
                                self.status = BaccaratStatus::ExpectPlayer;
                            } else if tb <= 5 {
                                self.status = BaccaratStatus::ExpectBanker;
                            } else {
                                self.status = BaccaratStatus::Done;
                            }
                        }
                    }
                    3 => self.status = BaccaratStatus::Done,
                    _ => panic!("banker cards is more than 3 now"),
                }
                true
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deal_cards_1() {
        let mut bd = BaccaratDealer::new();
        assert_eq!(bd.status, BaccaratStatus::ExpectPlayer);
        let result = bd.deal(Card::new('H', '5').unwrap());
        assert_eq!(result, true);
        let result = bd.deal(Card::new('D', '4').unwrap());
        assert_eq!(result, true);
        let result = bd.deal(Card::new('D', '3').unwrap());
        assert_eq!(result, true);
        let result = bd.deal(Card::new('H', '4').unwrap());
        assert_eq!(result, true);
        assert_eq!(bd.status, BaccaratStatus::Done);
    }

    #[test]
    fn test_deal_cards_2() {
        let mut bd = BaccaratDealer::new();
        assert_eq!(bd.status, BaccaratStatus::ExpectPlayer);
        let result = bd.deal(Card::new('C', '6').unwrap());
        assert_eq!(result, true);
        let result = bd.deal(Card::new('D', 'Q').unwrap());
        assert_eq!(result, true);
        let result = bd.deal(Card::new('D', 'J').unwrap());
        assert_eq!(result, true);
        let result = bd.deal(Card::new('D', 'J').unwrap());
        assert_eq!(result, true);
        assert_eq!(bd.status, BaccaratStatus::ExpectBanker);
        let result = bd.deal(Card::new('D', '3').unwrap());
        assert_eq!(bd.status, BaccaratStatus::Done);
        assert_eq!(result, true);
    }

    fn card(c1:char, c2:char)-> Card {
        Card::new(c1,c2).unwrap()
    }

    #[test]
    fn test_cards(){
        //HJS9CA#H7C3D7
        let cards  = vec![card('C','A'), card('D','7'),card('S','9'), card('C','3'), card('H','J'), card('H','7')];
        let result = BaccaratDealer::test(&cards);
        assert_eq!(result, true);

        //S2SJD2#D6SA
        let cards  = vec![card('D','2'), card('S','A'),card('S','J'), card('D','6'), card('S','2')];
        let result = BaccaratDealer::test(&cards);
        assert_eq!(result, true);
    }
}
