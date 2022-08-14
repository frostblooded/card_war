mod card;
mod deck;
mod rank;
mod suit;

use crate::deck::Deck;

fn main() {
    let mut deck_a = Deck::new_full();
    deck_a.shuffle();

    let mut deck_b = Deck::new_full();
    deck_b.shuffle();

    println!("A: {}\nB: {}", deck_a, deck_b);

    loop {
        if deck_a.is_empty() {
            println!("Player B wins!");
            break;
        } else if deck_b.is_empty() {
            println!("Player A wins!");
            break;
        }

        let card_a = deck_a.deal_card().expect("Deck is empty");
        let card_b = deck_b.deal_card().expect("Deck is empty");
        let a_wins: bool = card_a >= card_b;

        if a_wins {
            deck_a.add_card(card_a.clone());
            deck_a.add_card(card_b.clone());
        } else {
            deck_b.add_card(card_a.clone());
            deck_b.add_card(card_b.clone());
        }

        let winner_char: char = if a_wins { 'A' } else { 'B' };

        println!(
            "{} <-> {} | {} wins! | Score is now {} - {}",
            card_a,
            card_b,
            winner_char,
            deck_a.len(),
            deck_b.len()
        );

        println!("{}", deck_a);
        println!("{}", deck_b);
    }
}
