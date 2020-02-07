// Build and print a deck.

use cards::Deck;

fn main() {
    let mut deck = Deck::full();

    match std::env::args().nth(1).as_deref() {
        Some("bare") => (),
        None => deck.shuffle(),
        _ => panic!("unknown argument"),
    }

    for card in deck.iter() {
        println!("{}", card);
    }
}
