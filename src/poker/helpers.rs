use itertools::Itertools;
use ortalib::{Card, Enhancement, Suit, SuitColor};

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

// Helper function for FLower Pot Joker, which determines whether the current 
// card having the wanted suit
pub fn determine_current_suit(on_scored: &[Card], suit: Suit) -> bool {
    on_scored.iter().any(|c| {
        if c.suit == suit {
            return true;
        } else if let Some(enhance) = c.enhancement 
        && enhance == Enhancement::Wild {
            return true;
        };
        false
    })
}

pub fn determine_total_colors(on_scored: &[Card], color: SuitColor) -> bool {
    on_scored.iter().filter(|card| {
        if card.suit.color() == color {
            return true;
        } else if let Some(enhance) = card.enhancement 
        && enhance == Enhancement::Wild {
            return true;
        };
        false
    }).count() >= 2
}