mod card;
mod deck;
mod rank;
mod suit;

use crate::deck::Deck;

fn main() {
    let deck = Deck::new_full();
    println!("{}", deck);
}
