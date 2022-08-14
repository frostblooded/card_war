#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl TryFrom<char> for Suit {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'D' => Ok(Suit::Diamonds),
            'C' => Ok(Suit::Clubs),
            'H' => Ok(Suit::Hearts),
            'S' => Ok(Suit::Spades),
            _ => Err(()),
        }
    }
}

impl From<Suit> for char {
    fn from(suit: Suit) -> char {
        match suit {
            Suit::Diamonds => 'D',
            Suit::Clubs => 'C',
            Suit::Hearts => 'H',
            Suit::Spades => 'S',
        }
    }
}
