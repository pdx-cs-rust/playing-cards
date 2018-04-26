// Copyright © 2018 Roey Darwish Dror, Bart Massey
// This work is available under the terms of the "GPL v3.0".
// Please see the file LICENSE in this distribution for
// license information.

// Playing Cards, with a sample driver.

// Derived from
//     https://github.com/r-darwish/war/tree/
//       a43e4723898ae5f48fe1608f9622168a8aa2ca41

extern crate rand;

use rand::{thread_rng, Rng};
use std::vec::Vec;
use std::iter::Iterator;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}
use Suit::*;

const SUITS: &[Suit] = &[
    Clubs,
    Diamonds,
    Hearts,
    Spades,
];

const SUIT_NAMES: &[char] = &[
    'C',
    'D',
    'H',
    'S',
];

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", SUIT_NAMES[*self as usize])
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    Joker,
}
use Rank::*;

const RANKS: &[Rank] = &[
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
    Joker,
];

const RANK_NAMES: &[char] = &[
    'T',
    'J',
    'Q',
    'K',
    'A',
    '?',
];

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let names_start = RANKS.len() - RANK_NAMES.len();
        if *self as usize >= names_start {
            write!(f, "{}", RANK_NAMES[*self as usize - names_start])
        } else {
            write!(f, "{}", *self as usize)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Color {
    Black,
    Red,
}
use Color::*;

const COLORS: &[Color] = &[
    Black,
    Red,
];

const COLOR_NAMES: &[char] = &[
    'R',
    'B',
];

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", COLOR_NAMES[*self as usize])
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Card {
    SuitCard(Suit, Rank),
    SuitJoker(Color),
}
use Card::*;

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SuitCard(suit, rank) =>
                write!(f, "{}{}", rank, suit),
            SuitJoker(color) =>
                write!(f, "{}{}", color, Joker),
        }
    }
}

impl From<Suit> for Color {
    fn from(suit: Suit) -> Self {
        match suit {
            Clubs => Black,
            Diamonds => Red,
            Hearts => Red,
            Spades => Black,
        }
    }
}

pub struct IterCards {
    rank: usize,
    suit: usize,
    jokers: bool,
}

impl IterCards {
    pub fn standard() -> IterCards {
        IterCards {
            rank: 0,
            suit: 0,
            jokers: false,
        }
    }

    pub fn full() -> IterCards {
        IterCards {
            rank: 0,
            suit: 0,
            jokers: true,
        }
    }
}

impl Iterator for IterCards {
    type Item = Card;
    fn next(&mut self) -> Option<Self::Item> {
        if self.rank >= RANKS.len() {
            return None;
        }
        if self.rank == RANKS.len() - 1 {
            if !self.jokers || self.suit >= COLORS.len() {
                return None;
            }
            let result = SuitJoker(COLORS[self.suit]);
            self.suit += 1;
            return Some(result);
        }
        let result = SuitCard(SUITS[self.suit], RANKS[self.rank]);
        self.suit += 1;
        if self.suit >= SUITS.len() {
            self.suit = 0;
            self.rank += 1;
        }
        Some(result)
    }
}

impl Card {
    pub fn rank(&self) -> Rank {
        match self {
            &Card::SuitJoker(_) => Rank::Joker,
            &Card::SuitCard(_, ref rank) => *rank,
        }
    }

    pub fn suit(&self) -> Option<Suit> {
        match self {
            &Card::SuitJoker(_) => None,
            &Card::SuitCard(ref suit, _) => Some(*suit),
        }
    }

    pub fn color(&self) -> Color {
        match self {
            &Card::SuitJoker(color) => color,
            &Card::SuitCard(ref suit, _) => Color::from(*suit),
        }
    }

    pub fn iter_standard() -> IterCards {
        IterCards::standard()
    }

    pub fn iter_full() -> IterCards {
        IterCards::full()
    }
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    pub fn standard() -> Self {
        Self {
            cards: Card::iter_standard().collect(),
        }
    }

    pub fn full() -> Self {
        Self {
            cards: Card::iter_full().collect(),
        }
    }

    pub fn iter(&self) -> std::slice::Iter<Card> {
        self.cards.iter()
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        rng.shuffle(&mut self.cards)
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn put(&mut self, card: Card) {
        self.cards.push(card)
    }

    pub fn cards(&self) -> &Vec<Card> {
        &self.cards
    }

    pub fn append(&mut self, other: &mut Self) {
        self.cards.append(&mut other.cards);
    }
}

fn main() {
    let deck = Deck::full();
    for card in deck.iter() {
        println!("{}", card);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranks() {
        assert_eq!(
            Card::SuitJoker(Color::Black).rank(),
            Card::SuitJoker(Color::Red).rank()
        );

        assert!(
            Card::SuitJoker(Color::Black).rank() > Card::SuitCard(Suit::Spades, SuitRank::Ace).rank()
        );
    }
}
