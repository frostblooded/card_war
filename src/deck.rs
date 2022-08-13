use crate::card::Card;
use std::fmt;

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new_full() -> Self {
        let cards = vec![
            "C1", "C2", "C3", "C4", "C5", "C6", "C7", "C8", "C9", "C0", "CJ", "CQ", "CK", "CA",
            "D1", "D2", "D3", "D4", "D5", "D6", "D7", "D8", "D9", "D0", "DJ", "DQ", "DK", "DA",
            "H1", "H2", "H3", "H4", "H5", "H6", "H7", "H8", "H9", "H0", "HJ", "HQ", "HK", "HA",
            "S1", "S2", "S3", "S4", "S5", "S6", "S7", "S8", "S9", "S0", "SJ", "SQ", "SK", "SA",
        ]
        .iter()
        .map(|&s| {
            s.try_into()
                .expect("Malformed configuration of default deck. Couldn't convert string to card.")
        })
        .collect();

        Deck { cards }
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut cards_iter = self.cards.iter().peekable();

        while let Some(card) = cards_iter.next() {
            let s: String = card.clone().into();
            write!(f, "{}", s)?;

            if cards_iter.peek().is_some() {
                write!(f, ", ")?;
            }
        }

        Ok(())
    }
}
