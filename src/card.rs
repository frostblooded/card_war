use std::cmp::Ordering;
use std::fmt::Display;

use crate::rank::Rank;
use crate::suit::Suit;

macro_rules! card {
    ($s:expr) => {
        Card::try_from($s).expect("Card couldn't be created from passed in shortstring")
    };
}

pub(crate) use card;

#[derive(Clone, Debug)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card { suit, rank }
    }
}

impl TryFrom<&str> for Card {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            Err(())
        } else if let (Ok(suit), Ok(rank)) = (
            value.chars().next().unwrap().try_into(),
            value.chars().nth(1).unwrap().try_into(),
        ) {
            Ok(Card::new(suit, rank))
        } else {
            Err(())
        }
    }
}

impl From<Card> for String {
    fn from(card: Card) -> Self {
        let s: char = card.suit.into();
        let r: char = card.rank.into();
        format!("{}{}", s, r)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self.clone()))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

impl Eq for Card {}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cards_equate_correctly() {
        assert_eq!(card!("C1"), card!("D1"));
        assert_eq!(card!("C2"), card!("H2"));
        assert_ne!(card!("C1"), card!("D2"));
        assert_ne!(card!("C1"), card!("H3"));
    }

    #[test]
    fn cards_are_ordered_correctly() {
        assert!(card!("C2") > card!("C1"));
        assert!(!(card!("C2") <= card!("C1")));
        assert!(card!("C0") > card!("C1"));
        assert!(card!("C0") > card!("C9"));
        assert!(card!("C0") < card!("CJ"));
    }
}
