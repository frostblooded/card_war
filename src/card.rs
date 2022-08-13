use crate::rank::Rank;
use crate::suit::Suit;

#[derive(Clone)]
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
