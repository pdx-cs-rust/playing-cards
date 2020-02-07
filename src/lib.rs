// Copyright © 2018 Roey Darwish Dror, Bart Massey
// This work is available under the terms of the "GPL v3.0".
// Please see the file LICENSE in this distribution for
// license information.

//! Playing Cards. Derived from
//! https://github.com/r-darwish/war/tree/

//
// https://github.com/r-darwish/war/tree/
//   a43e4723898ae5f48fe1608f9622168a8aa2ca41

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;
use std::iter::Iterator;
use std::vec::Vec;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}
use Suit::*;

const SUITS: &[Suit] = &[Clubs, Diamonds, Hearts, Spades];

const SUIT_NAMES: &[char] = &['C', 'D', 'H', 'S'];

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
    Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen,
    King, Ace, Joker,
];

const RANK_NAMES: &[char] = &['T', 'J', 'Q', 'K', 'A', '?'];

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let names_start = RANKS.len() - RANK_NAMES.len();
        if *self as usize >= names_start {
            write!(f, "{}", RANK_NAMES[*self as usize - names_start])
        } else {
            write!(f, "{}", 2 + *self as usize)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Color {
    Black,
    Red,
}
use Color::*;

const COLORS: &[Color] = &[Black, Red];

const COLOR_NAMES: &[char] = &['R', 'B'];

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", COLOR_NAMES[*self as usize])
    }
}

/// A playing card.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Card {
    /// A "standard" card.
    SuitCard(Suit, Rank),
    /// A red or black Joker.
    JokerCard(Color),
}
use Card::*;

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SuitCard(suit, rank) => write!(f, "{}{}", rank, suit),
            JokerCard(color) => write!(f, "{}{}", color, Joker),
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

/// Iterator delivering the possible cards in rank-suit
/// order.
pub struct IterCards {
    rank: usize,
    suit: usize,
    jokers: bool,
}

impl IterCards {
    /// Get an iterator over the standard cards.
    pub fn standard() -> IterCards {
        IterCards {
            rank: 0,
            suit: 0,
            jokers: false,
        }
    }

    /// Get an iterator over the standard cards and jokers.
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
            let result = JokerCard(COLORS[self.suit]);
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
    /// Rank of this card.
    pub fn rank(self) -> Rank {
        match self {
            JokerCard(_) => Rank::Joker,
            SuitCard(_, ref rank) => *rank,
        }
    }

    /// Suit of this card.
    pub fn suit(self) -> Option<Suit> {
        match self {
            JokerCard(_) => None,
            SuitCard(ref suit, _) => Some(*suit),
        }
    }

    /// Color of this card: red or black.
    pub fn color(self) -> Color {
        match self {
            JokerCard(color) => color,
            SuitCard(ref suit, _) => Color::from(*suit),
        }
    }

    /// Get an iterator over the standard cards.
    pub fn iter_standard() -> IterCards {
        IterCards::standard()
    }

    /// Get an iterator over the standard cards and jokers.
    pub fn iter_full() -> IterCards {
        IterCards::full()
    }
}

/// A "deck" of cards is an ordered collection.
#[derive(Debug, Clone, Default)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// Make a new empty deck.
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    /// Make a new empty deck with reserved space
    /// for `capacity` cards.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            cards: Vec::with_capacity(capacity),
        }
    }

    /// Make a new standard deck.
    pub fn standard() -> Self {
        Self {
            cards: Card::iter_standard().collect(),
        }
    }

    /// Make a new standard deck with jokers.
    pub fn full() -> Self {
        Self {
            cards: Card::iter_full().collect(),
        }
    }

    /// Iterator over the current deck.
    pub fn iter(&self) -> std::slice::Iter<Card> {
        self.cards.iter()
    }

    /// Shuffle the current deck.
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng)
    }

    /// Pop the top card off the current deck.
    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /// Put `card` on top of the current deck.
    pub fn put(&mut self, card: Card) {
        self.cards.push(card)
    }

    /// Get the underlying deck as a vector.
    pub fn into_inner(self) -> Vec<Card> {
        self.cards
    }

    /// Make a vector of cards into a deck.
    pub fn from_inner(cards: Vec<Card>) -> Self {
        Self { cards }
    }

    /// Move all the cards of `other` onto the top
    /// of this deck, in the order they appear.
    pub fn append(&mut self, other: Self) {
        self.cards.extend(other.cards.into_iter());
    }

    /// Reverse the order of cards in this deck.
    pub fn reverse(&mut self) {
        self.cards.reverse();
    }
}

#[test]
fn ranks() {
    assert_eq!(
        JokerCard(Color::Black).rank(),
        JokerCard(Color::Red).rank()
    );

    assert!(JokerCard(Black).rank() > SuitCard(Spades, Ace).rank());
}
