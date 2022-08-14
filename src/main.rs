mod card;
mod deck;
mod rank;
mod suit;

use std::time::Duration;

use rand::seq::SliceRandom;

use crate::deck::Deck;

fn main() {
    let mut deck = Deck::new_full();
    deck.shuffle();
    let (mut deck_a, mut deck_b) = deck.split();

    let mut step = 0;

    loop {
        step += 1;

        if deck_a.is_empty() {
            println!("Player B wins!");
            break;
        } else if deck_b.is_empty() {
            println!("Player A wins!");
            break;
        }

        let card_a = deck_a.deal_card().expect("Deck is empty");
        let card_b = deck_b.deal_card().expect("Deck is empty");

        if card_a == card_b {
            let additional_cards_a = if deck_a.len() >= 3 {
                deck_a.deal_cards(3).unwrap()
            } else {
                deck_b.deal_cards(3).unwrap()
            };

            let additional_cards_b = if deck_b.len() >= 3 {
                deck_b.deal_cards(3).unwrap()
            } else {
                deck_a.deal_cards(3).unwrap()
            };

            let a_wins = additional_cards_a[2] > additional_cards_b[2];
            let mut cards_to_add = vec![
                card_a.clone(),
                additional_cards_a[0].clone(),
                additional_cards_a[1].clone(),
                additional_cards_a[2].clone(),
                card_b.clone(),
                additional_cards_b[0].clone(),
                additional_cards_b[1].clone(),
                additional_cards_b[2].clone(),
            ];
            cards_to_add.shuffle(&mut rand::thread_rng());

            if a_wins {
                cards_to_add.into_iter().for_each(|c| deck_a.add_card(c));
            } else {
                cards_to_add.into_iter().for_each(|c| deck_b.add_card(c));
            }

            let winner_char: char = if a_wins { 'A' } else { 'B' };

            println!(
                "Step {} | {} <-> {} | War! | {}, {}, {} <-> {}, {}, {} | {} wins! | Score is now {} - {}",
                step,
                card_a,
                card_b,
                additional_cards_a[0],
                additional_cards_a[1],
                additional_cards_a[2],
                additional_cards_b[0],
                additional_cards_b[1],
                additional_cards_b[2],
                winner_char,
                deck_a.len(),
                deck_b.len()
            );
        } else {
            let a_wins: bool = card_a > card_b;
            let mut cards_to_add = vec![card_a.clone(), card_b.clone()];
            cards_to_add.shuffle(&mut rand::thread_rng());

            if a_wins {
                cards_to_add.into_iter().for_each(|c| deck_a.add_card(c));
            } else {
                cards_to_add.into_iter().for_each(|c| deck_b.add_card(c));
            }

            let winner_char: char = if a_wins { 'A' } else { 'B' };

            println!(
                "Step {} | {} <-> {} | {} wins! | Score is now {} - {}",
                step,
                card_a,
                card_b,
                winner_char,
                deck_a.len(),
                deck_b.len()
            );
        }

        println!("Deck A: {}", deck_a);
        println!("Deck B: {}", deck_b);
        println!();

        // std::thread::sleep(Duration::from_secs(3));
    }
}
