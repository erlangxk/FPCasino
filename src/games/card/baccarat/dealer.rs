use games::card::baccarat::{total_points, value_of_card};
use games::card::{Card, Rank, Suit};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BaccaratStatus {
    ExpectPlayer,
    ExpectBanker,
    Done,
}

#[derive(Debug)]
pub struct BaccaratDealer {
    pub banker_cards: Vec<Card>,
    pub player_cards: Vec<Card>,
    pub status: BaccaratStatus,
}

pub fn test_baccarat_cards(cards: &Vec<Card>) -> bool {
    let mut init = init_baccarat_dealer();
    for c in cards {
        let result = init.deal(*c);
        if !result {
            return false;
        }
    }
    init.status == BaccaratStatus::Done
}

pub fn init_baccarat_dealer() -> BaccaratDealer {
    BaccaratDealer {
        banker_cards: vec![],
        player_cards: vec![],
        status: BaccaratStatus::ExpectPlayer,
    }
}

pub fn init_sevenup_dealer() -> BaccaratDealer {
    BaccaratDealer {
        banker_cards: vec![],
        player_cards: vec![
            Card {
                suit: Suit::Diamond,
                rank: Rank::Seven,
            },
        ],
        status: BaccaratStatus::ExpectBanker,
    }
}

impl BaccaratDealer {
    pub fn is_done(&self) -> bool {
        self.status == BaccaratStatus::Done
    }
    pub fn deal(&mut self, card: Card) -> bool {
        match self.status {
            BaccaratStatus::Done => false,

            BaccaratStatus::ExpectPlayer => {
                self.player_cards.push(card);
                match self.player_cards.len() {
                    1 | 2 => self.status = BaccaratStatus::ExpectBanker,
                    3 => {
                        let tb = total_points(&self.banker_cards);
                        let cv = value_of_card(&card);
                        let stand3 = cv == 8;
                        let stand4 = stand3 || cv == 0 || cv == 1 || cv == 9;
                        let stand5 = stand4 || cv == 2 || cv == 3;
                        let stand6 = stand5 || cv == 4 || cv == 5;
                        if (stand3 && tb == 3) || (stand4 && tb == 4) || (stand5 && tb == 5)
                            || (stand6 && tb == 6) || tb == 7
                        {
                            self.status = BaccaratStatus::Done;
                        } else {
                            self.status = BaccaratStatus::ExpectBanker
                        }
                    }
                    _ => panic!("player cards is more than 3 now"),
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
                        if tp == 8 || tp == 9 || tb == 8 || tb == 9 {
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
    use games::card::serde::str_to_card;
    use games::card::Card;

    fn card(s: &str) -> Card {
        str_to_card(s).unwrap()
    }

    #[test]
    fn test_deal_cards_1() {
        let mut bd = init_baccarat_dealer();
        assert_eq!(bd.status, BaccaratStatus::ExpectPlayer);
        let result = bd.deal(card("H5"));
        assert_eq!(result, true);
        let result = bd.deal(card("D4"));
        assert_eq!(result, true);
        let result = bd.deal(card("D3"));
        assert_eq!(result, true);
        let result = bd.deal(card("H4"));
        assert_eq!(result, true);
        assert_eq!(bd.status, BaccaratStatus::Done);
    }

    #[test]
    fn test_deal_cards_2() {
        let mut bd = init_baccarat_dealer();
        assert_eq!(bd.status, BaccaratStatus::ExpectPlayer);
        let result = bd.deal(card("C6"));
        assert_eq!(result, true);
        let result = bd.deal(card("DQ"));
        assert_eq!(result, true);
        let result = bd.deal(card("DJ"));
        assert_eq!(result, true);
        let result = bd.deal(card("DJ"));
        assert_eq!(result, true);
        assert_eq!(bd.status, BaccaratStatus::ExpectBanker);
        let result = bd.deal(card("D3"));
        assert_eq!(bd.status, BaccaratStatus::Done);
        assert_eq!(result, true);
    }

    #[test]
    fn test_cards_1() {
        let cards = vec![card("ST"), card("S9"), card("H2"), card("DQ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);
    }

    #[test]
    fn test_cards() {
        //HJS9CA#H7C3D7
        let cards = vec![
            card("CA"),
            card("D7"),
            card("S9"),
            card("C3"),
            card("HJ"),
            card("H7"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);

        //S2SJD2#D6SA
        let cards = vec![card("D2"), card("SA"), card("SJ"), card("D6"), card("S2")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);

        let cards = vec![card("D9"), card("ST"), card("SQ"), card("H9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("HJ"),
            card("DJ"),
            card("HK"),
            card("CA"),
            card("HJ"),
            card("H9"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("C3"),
            card("C8"),
            card("HA"),
            card("D2"),
            card("HT"),
            card("C6"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S2"), card("C4"), card("C5"), card("DK"), card("H6")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S3"), card("H4"), card("HQ"), card("H4")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("C2"),
            card("D2"),
            card("CT"),
            card("SJ"),
            card("HA"),
            card("HJ"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DJ"), card("S6"), card("C3"), card("SK"), card("DJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("SA"),
            card("CK"),
            card("C9"),
            card("CK"),
            card("C8"),
            card("D3"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("H2"),
            card("C7"),
            card("DK"),
            card("C4"),
            card("CT"),
            card("DQ"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("SK"),
            card("CQ"),
            card("SK"),
            card("CJ"),
            card("D5"),
            card("C6"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H7"), card("HJ"), card("DK"), card("SQ"), card("HT")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H4"), card("S4"), card("S2"), card("C4")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("ST"), card("S9"), card("H2"), card("DQ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("HQ"),
            card("S3"),
            card("CK"),
            card("H3"),
            card("C7"),
            card("H3"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("HK"), card("D4"), card("HK"), card("HT"), card("CQ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("DA"),
            card("DT"),
            card("HA"),
            card("C3"),
            card("D7"),
            card("H2"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("HA"), card("H9"), card("C6"), card("CA"), card("C7")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C2"), card("D2"), card("DA"), card("C2"), card("H8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DK"), card("D6"), card("H9"), card("CA")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("C5"),
            card("HT"),
            card("DJ"),
            card("H4"),
            card("D5"),
            card("S9"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("D9"),
            card("CA"),
            card("D3"),
            card("SQ"),
            card("ST"),
            card("HJ"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C8"), card("H6"), card("D8"), card("S3")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S5"), card("D8"), card("S4"), card("CK")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("C5"),
            card("DQ"),
            card("CQ"),
            card("SJ"),
            card("C7"),
            card("H5"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DT"), card("HA"), card("H9"), card("SK")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S8"), card("C4"), card("H3"), card("D5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S8"), card("SA"), card("D8"), card("DJ"), card("SK")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DA"), card("HT"), card("H3"), card("C8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("DA"),
            card("DT"),
            card("D2"),
            card("CK"),
            card("C5"),
            card("S4"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("D8"),
            card("DA"),
            card("H6"),
            card("C5"),
            card("C6"),
            card("C3"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DQ"), card("H6"), card("D9"), card("D5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("CA"), card("D7"), card("H6"), card("S4"), card("D4")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H7"), card("D6"), card("DK"), card("H9"), card("D7")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S5"), card("D8"), card("ST"), card("DJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D6"), card("C8"), card("SK"), card("S7"), card("S6")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H5"), card("D3"), card("H2"), card("S3")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("H2"),
            card("S7"),
            card("C2"),
            card("S5"),
            card("HA"),
            card("C9"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C6"), card("H7"), card("D2"), card("HJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("DK"),
            card("D6"),
            card("CA"),
            card("S6"),
            card("D6"),
            card("D5"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S9"), card("CJ"), card("HK"), card("SQ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("CJ"), card("CA"), card("C6"), card("D4"), card("S4")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("SA"), card("DA"), card("H4"), card("C4"), card("DA")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C4"), card("H5"), card("ST"), card("HQ"), card("DJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("S2"),
            card("CT"),
            card("D2"),
            card("CK"),
            card("CQ"),
            card("HA"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("CT"), card("S6"), card("D7"), card("HK")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C9"), card("S7"), card("D4"), card("S2")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DJ"), card("S6"), card("S4"), card("H3")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C7"), card("C9"), card("CJ"), card("DQ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H3"), card("S8"), card("H6"), card("SJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("HQ"), card("H8"), card("DT"), card("DT")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H7"), card("C8"), card("C9"), card("D7"), card("DT")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C7"), card("HA"), card("SJ"), card("C8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("S8"),
            card("S3"),
            card("H7"),
            card("D9"),
            card("HK"),
            card("H6"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C3"), card("D6"), card("S6"), card("DT")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("ST"),
            card("S5"),
            card("D4"),
            card("C5"),
            card("H4"),
            card("SQ"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("HQ"), card("C9"), card("C8"), card("HQ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DK"), card("HT"), card("SA"), card("D9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("HJ"), card("S7"), card("C6"), card("ST")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("H7"),
            card("CK"),
            card("H8"),
            card("CT"),
            card("H8"),
            card("H8"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("HJ"), card("D6"), card("H4"), card("CT"), card("SJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D3"), card("H8"), card("C6"), card("DQ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DK"), card("C3"), card("D9"), card("S8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("SJ"), card("CJ"), card("S7"), card("D8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("CJ"),
            card("H8"),
            card("D3"),
            card("H4"),
            card("H2"),
            card("SA"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("HT"), card("H2"), card("CQ"), card("D6")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C2"), card("S8"), card("S9"), card("H9"), card("C3")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S5"), card("SJ"), card("C6"), card("S6"), card("D4")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("SA"), card("SJ"), card("HK"), card("D7"), card("C5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H5"), card("D7"), card("DQ"), card("CT"), card("H8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D5"), card("SA"), card("ST"), card("H6"), card("S9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S4"), card("C7"), card("S3"), card("H9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C2"), card("DK"), card("H9"), card("D9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C5"), card("S2"), card("SK"), card("S7")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("S4"),
            card("CJ"),
            card("CA"),
            card("DT"),
            card("DA"),
            card("S3"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C7"), card("H7"), card("S9"), card("H9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DJ"), card("SQ"), card("C9"), card("HQ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H7"), card("H7"), card("C7"), card("S7"), card("DJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("CJ"), card("S9"), card("C9"), card("D5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("C8"),
            card("D7"),
            card("C7"),
            card("C3"),
            card("CQ"),
            card("DT"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("H4"),
            card("S9"),
            card("DT"),
            card("HA"),
            card("C4"),
            card("S5"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("HA"),
            card("DJ"),
            card("D4"),
            card("DK"),
            card("C6"),
            card("H8"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("S3"),
            card("H8"),
            card("HK"),
            card("H2"),
            card("H4"),
            card("D4"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("H7"),
            card("C6"),
            card("D4"),
            card("H4"),
            card("DQ"),
            card("H2"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C9"), card("C8"), card("HQ"), card("S7")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D6"), card("DK"), card("DA"), card("SJ"), card("C2")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S2"), card("C8"), card("C5"), card("SJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D9"), card("SK"), card("S8"), card("D9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("CA"), card("D6"), card("HJ"), card("SJ"), card("SJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C5"), card("HQ"), card("D9"), card("D8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("S4"),
            card("D6"),
            card("CQ"),
            card("D7"),
            card("DK"),
            card("DA"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DQ"), card("C4"), card("S8"), card("DA")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("SQ"), card("H6"), card("C8"), card("SK")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("SQ"), card("S6"), card("D5"), card("CA"), card("SQ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("CK"), card("S9"), card("CA"), card("DK")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S2"), card("H9"), card("HQ"), card("DJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("HT"), card("ST"), card("D7"), card("DJ"), card("S3")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H9"), card("H3"), card("S7"), card("DK"), card("C2")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H3"), card("CA"), card("H6"), card("H2")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("DK"),
            card("D3"),
            card("C3"),
            card("HA"),
            card("C3"),
            card("H4"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C7"), card("S7"), card("D5"), card("DJ"), card("D7")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("CK"), card("C4"), card("S2"), card("D2"), card("HK")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("ST"), card("SA"), card("D6"), card("H5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D5"), card("D2"), card("S7"), card("C2"), card("D8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H6"), card("D7"), card("SK"), card("CK")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("CK"), card("S6"), card("H6"), card("S6"), card("S8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D8"), card("D8"), card("SA"), card("S8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("D5"),
            card("S3"),
            card("DK"),
            card("SK"),
            card("S5"),
            card("HT"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("ST"),
            card("D4"),
            card("DT"),
            card("S6"),
            card("HK"),
            card("HA"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("HT"), card("CA"), card("CK"), card("H6"), card("S4")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D6"), card("D5"), card("C9"), card("DT"), card("DA")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C3"), card("C9"), card("ST"), card("HJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DA"), card("HK"), card("S8"), card("C9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C4"), card("HT"), card("H2"), card("S8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("S2"),
            card("C4"),
            card("CT"),
            card("H8"),
            card("D6"),
            card("H9"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("S5"),
            card("S2"),
            card("D8"),
            card("S3"),
            card("H5"),
            card("D4"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C3"), card("S7"), card("D2"), card("CQ"), card("SQ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H2"), card("SQ"), card("S4"), card("CT"), card("H4")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D9"), card("ST"), card("S7"), card("D9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("H7"),
            card("CK"),
            card("D4"),
            card("SA"),
            card("H8"),
            card("D2"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("SK"), card("C7"), card("HA"), card("DQ"), card("HK")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("HQ"), card("ST"), card("S6"), card("CQ"), card("S4")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("H9"),
            card("DT"),
            card("D3"),
            card("CQ"),
            card("HK"),
            card("HJ"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("CT"), card("S3"), card("CJ"), card("S3"), card("S5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("D3"),
            card("S5"),
            card("C9"),
            card("CJ"),
            card("C4"),
            card("D8"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H5"), card("HJ"), card("D2"), card("H8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("CQ"), card("H3"), card("ST"), card("C5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C7"), card("HJ"), card("S9"), card("DJ"), card("H3")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C8"), card("DT"), card("S4"), card("C9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("D9"),
            card("SQ"),
            card("D2"),
            card("C6"),
            card("H7"),
            card("CK"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("S2"),
            card("DJ"),
            card("DQ"),
            card("SA"),
            card("HQ"),
            card("DT"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("SK"), card("D2"), card("S6"), card("HQ"), card("C6")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("D6"),
            card("HK"),
            card("H5"),
            card("D2"),
            card("HQ"),
            card("CT"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C4"), card("H5"), card("H4"), card("S2")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C8"), card("H9"), card("H8"), card("SJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D7"), card("D9"), card("H6"), card("S9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DQ"), card("SK"), card("C8"), card("C3")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S6"), card("D5"), card("HQ"), card("D3")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S4"), card("H8"), card("C2"), card("C2"), card("H3")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C5"), card("D3"), card("CJ"), card("H3"), card("HJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("H5"),
            card("HJ"),
            card("H6"),
            card("HT"),
            card("C8"),
            card("C7"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("H2"),
            card("DK"),
            card("DJ"),
            card("SA"),
            card("C3"),
            card("H4"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("S5"),
            card("DA"),
            card("S9"),
            card("CT"),
            card("H2"),
            card("H4"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H7"), card("DT"), card("H8"), card("C4"), card("S8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("CJ"), card("C5"), card("DQ"), card("CT"), card("H9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D3"), card("DQ"), card("SA"), card("H6"), card("H5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("SJ"), card("H3"), card("DQ"), card("S5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C5"), card("SK"), card("SA"), card("DA"), card("D3")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("C9"),
            card("HA"),
            card("HA"),
            card("HT"),
            card("C6"),
            card("CJ"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("SA"),
            card("CK"),
            card("HQ"),
            card("H5"),
            card("S4"),
            card("S5"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S7"), card("HJ"), card("H7"), card("C8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C7"), card("H9"), card("S3"), card("HJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("H7"),
            card("DT"),
            card("S3"),
            card("D3"),
            card("DA"),
            card("C9"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("SJ"), card("D6"), card("H3"), card("DQ"), card("D9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("H3"),
            card("HT"),
            card("CQ"),
            card("C2"),
            card("CJ"),
            card("C9"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("CQ"),
            card("HT"),
            card("DQ"),
            card("H3"),
            card("SQ"),
            card("D2"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("CK"), card("H7"), card("C7"), card("D8"), card("CQ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("HA"),
            card("S8"),
            card("DK"),
            card("C5"),
            card("C3"),
            card("D3"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C4"), card("H3"), card("D5"), card("D2")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DJ"), card("S7"), card("C2"), card("DJ"), card("H8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("SQ"), card("H3"), card("H8"), card("HA")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DJ"), card("HT"), card("ST"), card("D6"), card("C8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C8"), card("CK"), card("S5"), card("D8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S8"), card("D4"), card("S9"), card("S6"), card("SJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C9"), card("CA"), card("CA"), card("H6"), card("S5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C8"), card("D4"), card("DT"), card("CA")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("S9"),
            card("DJ"),
            card("D6"),
            card("CJ"),
            card("SA"),
            card("H9"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("HK"),
            card("D4"),
            card("DK"),
            card("SA"),
            card("H5"),
            card("C7"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("ST"),
            card("DA"),
            card("D5"),
            card("S9"),
            card("D8"),
            card("SQ"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C7"), card("C5"), card("D9"), card("S3")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H4"), card("CT"), card("S2"), card("C4"), card("DK")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("SA"),
            card("CJ"),
            card("HQ"),
            card("SA"),
            card("D7"),
            card("H5"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H8"), card("H4"), card("SQ"), card("H5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("D4"),
            card("HQ"),
            card("DT"),
            card("H6"),
            card("H7"),
            card("CK"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DA"), card("S2"), card("C9"), card("S6")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DT"), card("DQ"), card("S9"), card("HJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D9"), card("S4"), card("D3"), card("C3"), card("D5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C7"), card("HJ"), card("C4"), card("H4"), card("HK")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S4"), card("HJ"), card("C4"), card("H4")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("CQ"),
            card("CT"),
            card("SK"),
            card("D2"),
            card("D9"),
            card("ST"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DJ"), card("C5"), card("HK"), card("C2"), card("H8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DK"), card("S8"), card("D9"), card("SK")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S7"), card("C2"), card("SA"), card("HJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DA"), card("CK"), card("S6"), card("CT"), card("H9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("DQ"),
            card("ST"),
            card("DA"),
            card("SJ"),
            card("DK"),
            card("H5"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D5"), card("HK"), card("S8"), card("C8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("D4"),
            card("ST"),
            card("D7"),
            card("DJ"),
            card("C3"),
            card("D8"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D7"), card("C4"), card("H2"), card("H4")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("S2"),
            card("H7"),
            card("DK"),
            card("H6"),
            card("C2"),
            card("S8"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("HA"), card("H4"), card("C7"), card("S3")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C6"), card("S4"), card("DJ"), card("HK"), card("SJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DQ"), card("D5"), card("DT"), card("S4")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("S7"),
            card("S5"),
            card("S7"),
            card("H5"),
            card("S2"),
            card("CT"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C7"), card("D4"), card("H3"), card("S3"), card("H8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H8"), card("D2"), card("CQ"), card("C2")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C9"), card("HA"), card("DT"), card("SK")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("H5"),
            card("CJ"),
            card("S9"),
            card("HQ"),
            card("C8"),
            card("S4"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S6"), card("DJ"), card("CA"), card("H2"), card("S5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C3"), card("HK"), card("C2"), card("C7"), card("C3")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H6"), card("S7"), card("S2"), card("D8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C5"), card("HA"), card("H7"), card("D6"), card("DA")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S6"), card("D3"), card("S5"), card("C6")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C9"), card("D9"), card("SQ"), card("C3")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C6"), card("D4"), card("D2"), card("CT")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("SK"), card("H6"), card("H9"), card("C5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("CJ"), card("DQ"), card("S6"), card("C9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("SK"), card("D7"), card("CA"), card("C8"), card("C9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D8"), card("H9"), card("CQ"), card("S9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H6"), card("H6"), card("C4"), card("S2")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("HK"),
            card("C2"),
            card("H2"),
            card("HT"),
            card("S2"),
            card("H2"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H3"), card("D3"), card("D3"), card("HT"), card("D5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("H6"),
            card("CA"),
            card("C8"),
            card("HA"),
            card("C5"),
            card("H3"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S6"), card("H7"), card("D8"), card("CT"), card("S5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("H4"),
            card("C6"),
            card("HQ"),
            card("C4"),
            card("H7"),
            card("D2"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("HA"),
            card("S3"),
            card("DA"),
            card("SJ"),
            card("D3"),
            card("S8"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D2"), card("C5"), card("D4"), card("CJ"), card("D5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("S4"),
            card("C3"),
            card("SA"),
            card("HA"),
            card("H2"),
            card("SA"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("CA"), card("D7"), card("C4"), card("CQ"), card("ST")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H9"), card("DQ"), card("HT"), card("CA")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D9"), card("D8"), card("SQ"), card("CT")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D8"), card("S7"), card("D5"), card("CT"), card("S5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("HT"), card("DQ"), card("H4"), card("S6"), card("H4")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("SK"), card("D7"), card("H7"), card("D2")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C8"), card("DK"), card("SA"), card("S8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("H2"),
            card("HK"),
            card("ST"),
            card("D4"),
            card("C3"),
            card("D5"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D2"), card("C7"), card("D6"), card("S5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("C2"),
            card("DA"),
            card("HK"),
            card("C3"),
            card("H6"),
            card("SA"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("D2"),
            card("ST"),
            card("CK"),
            card("H2"),
            card("C3"),
            card("DJ"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("C7"),
            card("C4"),
            card("H3"),
            card("C8"),
            card("C8"),
            card("S9"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("H3"),
            card("S6"),
            card("HT"),
            card("D5"),
            card("CQ"),
            card("DK"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H6"), card("DT"), card("SA"), card("CK"), card("H4")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C4"), card("D2"), card("D9"), card("H6")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H5"), card("D9"), card("DA"), card("HA"), card("SK")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C4"), card("H8"), card("C3"), card("D3"), card("S2")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("CK"),
            card("SQ"),
            card("DA"),
            card("H5"),
            card("H7"),
            card("S5"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("CA"),
            card("C6"),
            card("SA"),
            card("SJ"),
            card("C6"),
            card("D5"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("CJ"),
            card("H6"),
            card("CJ"),
            card("H8"),
            card("C7"),
            card("H9"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C8"), card("H5"), card("S2"), card("CT"), card("D9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C4"), card("S6"), card("S2"), card("H8"), card("S9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("SQ"), card("S2"), card("C7"), card("C9"), card("S4")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("D9"),
            card("D3"),
            card("S4"),
            card("CA"),
            card("D7"),
            card("CQ"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("DJ"),
            card("D2"),
            card("S2"),
            card("CA"),
            card("D6"),
            card("SK"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C5"), card("HK"), card("HK"), card("D8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S4"), card("S3"), card("C5"), card("HJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D8"), card("CJ"), card("CT"), card("DJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("HA"), card("H6"), card("DT"), card("ST"), card("C9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("H2"),
            card("S4"),
            card("H2"),
            card("D7"),
            card("C8"),
            card("D3"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("HA"), card("H3"), card("C8"), card("DT")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("CQ"), card("DQ"), card("C9"), card("C5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("DA"),
            card("D8"),
            card("HJ"),
            card("C3"),
            card("H4"),
            card("DA"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S8"), card("S5"), card("D3"), card("D2"), card("DT")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("SJ"),
            card("H6"),
            card("SQ"),
            card("C7"),
            card("DT"),
            card("H4"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H9"), card("SA"), card("SK"), card("D6")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("CT"), card("D9"), card("H7"), card("H2"), card("S9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("SA"), card("D3"), card("D5"), card("H5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DQ"), card("D3"), card("DK"), card("C4"), card("HK")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C7"), card("D9"), card("C7"), card("H9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("CK"), card("S8"), card("S6"), card("D7"), card("DQ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("SQ"), card("S5"), card("CA"), card("C8"), card("H8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D7"), card("D8"), card("S7"), card("SK")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("CK"), card("HK"), card("C9"), card("DK")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("HJ"), card("S2"), card("H8"), card("D7")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("C5"),
            card("SK"),
            card("SJ"),
            card("HK"),
            card("SJ"),
            card("D9"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("D4"),
            card("DA"),
            card("DQ"),
            card("DK"),
            card("C5"),
            card("S5"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("CQ"), card("HQ"), card("C3"), card("S7"), card("DJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H5"), card("S7"), card("H2"), card("C2")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D6"), card("C2"), card("ST"), card("HQ"), card("S8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("CK"), card("CJ"), card("CJ"), card("C6"), card("H3")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("C5"),
            card("SJ"),
            card("SJ"),
            card("C3"),
            card("DA"),
            card("D4"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("S3"),
            card("DJ"),
            card("CQ"),
            card("S3"),
            card("CT"),
            card("D6"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C2"), card("C9"), card("ST"), card("S5"), card("H9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DK"), card("HA"), card("C6"), card("S3"), card("CT")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C7"), card("C9"), card("CK"), card("S9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("CJ"), card("H3"), card("CA"), card("D4"), card("S4")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("ST"), card("S7"), card("H9"), card("S4")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H5"), card("H7"), card("HQ"), card("S8"), card("HJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D7"), card("SJ"), card("CJ"), card("C9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("D4"),
            card("HT"),
            card("CA"),
            card("HQ"),
            card("HQ"),
            card("D2"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DT"), card("D2"), card("S6"), card("SA"), card("C4")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H3"), card("HQ"), card("H9"), card("C5"), card("SQ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DJ"), card("C8"), card("S9"), card("S3")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("HQ"), card("H4"), card("CQ"), card("H3"), card("S7")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("S3"),
            card("C2"),
            card("ST"),
            card("SK"),
            card("HA"),
            card("S6"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H6"), card("HT"), card("S3"), card("CK")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("HA"), card("SK"), card("CQ"), card("S4"), card("DK")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("CT"), card("CA"), card("DJ"), card("D7")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("DK"),
            card("DQ"),
            card("HT"),
            card("S3"),
            card("C4"),
            card("C6"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("H6"),
            card("C3"),
            card("H4"),
            card("HT"),
            card("S5"),
            card("D4"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("H4"),
            card("H9"),
            card("CQ"),
            card("HA"),
            card("DT"),
            card("C6"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S4"), card("S7"), card("C5"), card("D8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("HA"), card("ST"), card("S2"), card("H7"), card("S8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S9"), card("D5"), card("SK"), card("S2")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S9"), card("HJ"), card("DK"), card("S6")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("DK"),
            card("S3"),
            card("HA"),
            card("D7"),
            card("CQ"),
            card("DA"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D8"), card("C7"), card("C9"), card("DJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H2"), card("C3"), card("DQ"), card("C6")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S9"), card("SJ"), card("SQ"), card("HK")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S6"), card("H7"), card("CK"), card("H9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("S3"),
            card("HQ"),
            card("SA"),
            card("CK"),
            card("CJ"),
            card("H6"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("HA"),
            card("HT"),
            card("C4"),
            card("SJ"),
            card("SQ"),
            card("SA"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D4"), card("D9"), card("ST"), card("SK")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("C8"),
            card("S3"),
            card("D2"),
            card("SA"),
            card("D4"),
            card("DQ"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C7"), card("HK"), card("DT"), card("S5"), card("D7")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("ST"), card("S7"), card("H9"), card("DQ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("SA"), card("CK"), card("CA"), card("S8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("C9"),
            card("S4"),
            card("C5"),
            card("C7"),
            card("DT"),
            card("HQ"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("CJ"),
            card("D4"),
            card("SJ"),
            card("D7"),
            card("H4"),
            card("DK"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C5"), card("H9"), card("C8"), card("H8"), card("DT")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C3"), card("D4"), card("C6"), card("DQ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D2"), card("S8"), card("H6"), card("DJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H9"), card("D6"), card("H5"), card("SJ"), card("H3")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("C9"),
            card("C8"),
            card("H2"),
            card("S2"),
            card("HJ"),
            card("C4"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("DT"),
            card("CJ"),
            card("HJ"),
            card("CQ"),
            card("H2"),
            card("C8"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C3"), card("DK"), card("D5"), card("H8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C2"), card("H5"), card("H4"), card("D9"), card("S7")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("HA"), card("H6"), card("C2"), card("CA"), card("H7")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S6"), card("H9"), card("ST"), card("HQ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("HK"), card("C6"), card("ST"), card("D2")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("D8"),
            card("CK"),
            card("C6"),
            card("C3"),
            card("D3"),
            card("HA"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H3"), card("D8"), card("DT"), card("D6"), card("HT")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H4"), card("C7"), card("D6"), card("D9"), card("C5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C3"), card("ST"), card("S5"), card("HT")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DJ"), card("HQ"), card("S8"), card("D2")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S9"), card("H6"), card("DJ"), card("SK")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H5"), card("C8"), card("DA"), card("HJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S5"), card("SA"), card("C4"), card("C4")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("CQ"), card("S3"), card("D8"), card("H3")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("S4"),
            card("CJ"),
            card("SK"),
            card("D2"),
            card("DA"),
            card("S3"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("S5"),
            card("D7"),
            card("SQ"),
            card("S6"),
            card("HA"),
            card("C4"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("CA"),
            card("S7"),
            card("C4"),
            card("S8"),
            card("D4"),
            card("C7"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C5"), card("D5"), card("CT"), card("D9"), card("C9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("HQ"),
            card("D3"),
            card("DA"),
            card("DJ"),
            card("S2"),
            card("CA"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("DQ"),
            card("SJ"),
            card("HK"),
            card("DK"),
            card("C2"),
            card("D9"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DA"), card("D7"), card("D8"), card("H2")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("SJ"), card("DJ"), card("D6"), card("S5"), card("D5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("CA"), card("S9"), card("S2"), card("HJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H7"), card("H6"), card("C2"), card("S2")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("CQ"),
            card("SQ"),
            card("DK"),
            card("HK"),
            card("HK"),
            card("D4"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H7"), card("SQ"), card("CA"), card("S6")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H3"), card("H7"), card("HA"), card("S7"), card("HT")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S7"), card("C5"), card("HK"), card("C2")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("SJ"),
            card("CK"),
            card("CT"),
            card("S2"),
            card("DA"),
            card("S4"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("CT"),
            card("H5"),
            card("C2"),
            card("DQ"),
            card("H6"),
            card("CT"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C9"), card("HQ"), card("CT"), card("S3")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("H2"),
            card("S6"),
            card("HA"),
            card("C4"),
            card("C8"),
            card("HK"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("CK"),
            card("DT"),
            card("SQ"),
            card("C2"),
            card("CQ"),
            card("SA"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("HJ"), card("D9"), card("SQ"), card("D8"), card("D3")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("SJ"), card("C7"), card("H8"), card("D8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D8"), card("H9"), card("S6"), card("H8"), card("D2")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("ST"), card("DA"), card("C6"), card("S4"), card("S5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H8"), card("D7"), card("DQ"), card("H4")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DJ"), card("C9"), card("S5"), card("C6"), card("H2")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H8"), card("H5"), card("H7"), card("D3")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S8"), card("CJ"), card("D5"), card("C5"), card("C8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H9"), card("S4"), card("S3"), card("CJ"), card("SK")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("HT"), card("SA"), card("SQ"), card("C8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("HJ"), card("C7"), card("ST"), card("HT"), card("D5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("CT"), card("DQ"), card("D2"), card("C4"), card("CJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("C3"),
            card("D3"),
            card("CQ"),
            card("HQ"),
            card("HT"),
            card("D2"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("H2"),
            card("C9"),
            card("CK"),
            card("D6"),
            card("H6"),
            card("DK"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("C6"),
            card("S7"),
            card("D5"),
            card("H4"),
            card("CT"),
            card("DJ"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("CT"), card("H3"), card("H6"), card("DT"), card("SK")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C9"), card("D6"), card("H7"), card("S4"), card("S2")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H5"), card("S7"), card("D4"), card("C3")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("S9"),
            card("C5"),
            card("H3"),
            card("D7"),
            card("S6"),
            card("H4"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("H7"),
            card("S4"),
            card("C3"),
            card("C7"),
            card("S7"),
            card("H5"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D9"), card("D3"), card("HK"), card("CQ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("HK"), card("C5"), card("HK"), card("ST"), card("H3")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H7"), card("C4"), card("H2"), card("CA")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DQ"), card("D3"), card("C7"), card("S6")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DJ"), card("H5"), card("C6"), card("S3")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DJ"), card("C4"), card("CQ"), card("D5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("HT"), card("H3"), card("H6"), card("CQ"), card("S2")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S3"), card("D4"), card("C7"), card("S5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D6"), card("HJ"), card("H2"), card("S2")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H3"), card("S6"), card("C2"), card("CA"), card("SA")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DA"), card("SK"), card("ST"), card("H7"), card("CA")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H3"), card("S9"), card("HJ"), card("S9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D6"), card("D3"), card("S4"), card("D3"), card("D5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D2"), card("S9"), card("CJ"), card("H5"), card("CT")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H6"), card("HQ"), card("CT"), card("S3"), card("DT")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S2"), card("CK"), card("C8"), card("C6"), card("HA")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D5"), card("D4"), card("H2"), card("S8"), card("C9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S5"), card("DT"), card("H2"), card("S7")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DK"), card("S7"), card("H4"), card("CJ"), card("S7")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("D6"),
            card("S9"),
            card("H9"),
            card("C4"),
            card("HK"),
            card("C8"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H8"), card("S6"), card("D2"), card("S2")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("S3"),
            card("SQ"),
            card("D7"),
            card("SK"),
            card("H7"),
            card("CA"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("SJ"), card("C9"), card("SJ"), card("D9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("H4"),
            card("D2"),
            card("D7"),
            card("H8"),
            card("D2"),
            card("D8"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("D4"),
            card("SQ"),
            card("SJ"),
            card("C3"),
            card("SA"),
            card("H3"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("DT"),
            card("D7"),
            card("C4"),
            card("H6"),
            card("CJ"),
            card("S4"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("SK"), card("D9"), card("S8"), card("CT")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("DQ"),
            card("CQ"),
            card("C3"),
            card("HJ"),
            card("S8"),
            card("D8"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C6"), card("C2"), card("HT"), card("CK"), card("S4")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H5"), card("SA"), card("D2"), card("CT"), card("ST")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C3"), card("C8"), card("C5"), card("ST")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("HA"), card("SJ"), card("C6"), card("S6")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S8"), card("S3"), card("S9"), card("H8"), card("CA")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("C8"),
            card("S7"),
            card("S6"),
            card("C9"),
            card("D7"),
            card("C9"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("S4"),
            card("DA"),
            card("HQ"),
            card("DJ"),
            card("H9"),
            card("D4"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("D6"),
            card("SA"),
            card("D7"),
            card("CJ"),
            card("SK"),
            card("HA"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("SQ"), card("DT"), card("CJ"), card("C5"), card("C9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("S2"),
            card("SJ"),
            card("D8"),
            card("HA"),
            card("DQ"),
            card("HK"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("C4"),
            card("S3"),
            card("SQ"),
            card("H2"),
            card("C5"),
            card("DQ"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("SJ"),
            card("D5"),
            card("CK"),
            card("C7"),
            card("C2"),
            card("CT"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("D9"),
            card("H5"),
            card("S6"),
            card("H6"),
            card("DQ"),
            card("D9"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("ST"),
            card("D9"),
            card("DJ"),
            card("D4"),
            card("CJ"),
            card("SA"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D7"), card("H9"), card("HQ"), card("DQ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("HK"),
            card("SQ"),
            card("SJ"),
            card("H6"),
            card("C7"),
            card("H5"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("CK"),
            card("H3"),
            card("DA"),
            card("H9"),
            card("D8"),
            card("S4"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C8"), card("S8"), card("DA"), card("D5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S9"), card("C9"), card("H7"), card("CA"), card("HJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("SJ"), card("C3"), card("H3"), card("S6")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H6"), card("H6"), card("CA"), card("HQ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("ST"), card("H7"), card("H7"), card("H4"), card("DJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("CA"), card("C8"), card("D2"), card("H9"), card("C7")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DK"), card("C6"), card("SA"), card("SA"), card("H8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("H4"), card("D3"), card("C4"), card("H9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D9"), card("HT"), card("DQ"), card("D2")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C2"), card("D6"), card("D5"), card("CT")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DA"), card("S5"), card("C2"), card("DK"), card("CJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("HQ"), card("CJ"), card("CK"), card("C9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("SQ"),
            card("H2"),
            card("DK"),
            card("HT"),
            card("H4"),
            card("DA"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D4"), card("H2"), card("S5"), card("H9")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S7"), card("CQ"), card("HQ"), card("H5"), card("D8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("HJ"),
            card("CT"),
            card("S5"),
            card("CK"),
            card("S8"),
            card("S9"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C8"), card("C5"), card("S4"), card("HT"), card("DT")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D8"), card("DT"), card("CK"), card("ST")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DJ"), card("D5"), card("S8"), card("H6")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DA"), card("H8"), card("C8"), card("SA")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("HQ"),
            card("S5"),
            card("CQ"),
            card("D5"),
            card("C3"),
            card("HT"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C7"), card("C3"), card("HQ"), card("H9"), card("C5")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("SK"), card("HT"), card("H6"), card("C7")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("D4"), card("C2"), card("H9"), card("HA"), card("C8")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("C6"), card("D3"), card("C3"), card("D7")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![
            card("SA"),
            card("S6"),
            card("HK"),
            card("H9"),
            card("D6"),
            card("CQ"),
        ];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S6"), card("DA"), card("C5"), card("C6"), card("CJ")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S9"), card("DK"), card("HJ"), card("D3")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("S9"), card("CK"), card("CQ"), card("SA")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);


        let cards = vec![card("DQ"), card("C8"), card("S3"), card("H8"), card("CK")];
        let result = test_baccarat_cards(&cards);
        assert_eq!(result, true);
    }
}
