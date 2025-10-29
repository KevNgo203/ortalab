use itertools::Itertools;
use ortalib::{Card, Suit};

// Helper function for Flush-based Hand, which computes the most appear suit
pub fn compute_most_appear_suit(cards: &[Card]) -> Suit {
    cards
        .iter()
        .counts_by(|c| c.suit)
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(suit, _)| suit)
        .unwrap()
}
