use rand::seq::SliceRandom;

use crate::card::card;
use crate::card::Card;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    #[cfg(test)]
    pub fn new() -> Self {
        Deck { cards: vec![] }
    }

    pub fn new_full() -> Self {
        let cards = vec![
            "C1", "C2", "C3", "C4", "C5", "C6", "C7", "C8", "C9", "C0", "CJ", "CQ", "CK", "CA",
            "D1", "D2", "D3", "D4", "D5", "D6", "D7", "D8", "D9", "D0", "DJ", "DQ", "DK", "DA",
            "H1", "H2", "H3", "H4", "H5", "H6", "H7", "H8", "H9", "H0", "HJ", "HQ", "HK", "HA",
            "S1", "S2", "S3", "S4", "S5", "S6", "S7", "S8", "S9", "S0", "SJ", "SQ", "SK", "SA",
        ]
        .iter()
        .map(|&s| card!(s))
        .collect();

        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::thread_rng());
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn deal_card(&mut self) -> Option<Card> {
        self.deal_cards(1).map(|mut c| c.swap_remove(0))
    }

    pub fn deal_cards(&mut self, amount: u32) -> Option<Vec<Card>> {
        if amount as usize > self.cards.len() {
            None
        } else {
            let mut tmp = self.cards.split_off(amount as usize);
            std::mem::swap(&mut self.cards, &mut tmp);
            Some(tmp)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn len(&self) -> usize {
        self.cards.len()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dealing_a_card() {
        let mut deck = Deck::new();
        deck.add_card(card!("C1"));
        deck.add_card(card!("C2"));
        deck.add_card(card!("C3"));

        let dealt_card = deck.deal_card().unwrap();
        assert_eq!(dealt_card, card!("C1"));

        assert_eq!(deck.cards[0], card!("C2"));
        assert_eq!(deck.cards[1], card!("C3"));
    }

    #[test]
    fn test_dealing_cards() {
        let mut deck = Deck::new();
        deck.add_card(card!("C1"));
        deck.add_card(card!("C2"));
        deck.add_card(card!("C3"));
        deck.add_card(card!("C4"));

        let dealt_cards = deck.deal_cards(2).unwrap();
        assert_eq!(dealt_cards[0], card!("C1"));
        assert_eq!(dealt_cards[1], card!("C2"));

        assert_eq!(deck.cards[0], card!("C3"));
        assert_eq!(deck.cards[1], card!("C4"));
    }

    #[test]
    fn test_adding_a_card() {
        let mut deck = Deck::new();
        assert_eq!(deck.deal_cards(1), None);

        deck.add_card(card!("C4"));
        assert_eq!(deck.deal_cards(1).unwrap()[0], card!("C4"));
    }
}
