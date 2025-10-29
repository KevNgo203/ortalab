//! # Poker Helpers
//!
//! This module provides utility functions for evaluating suits and colors
//! in poker hands. These helpers are used by scoring logic, jokers, and
//! modifiers to determine flushes, suit matches, and color counts.

use itertools::Itertools;
use ortalib::{Card, Enhancement, Suit, SuitColor};

/// Computes the most frequently appearing suit among the given cards.
///
/// This is typically used for flush‑based hands to determine the dominant suit.
///
/// # Arguments
/// * `cards` ‑ A slice of [`Card`]s to analyze.
///
/// # Returns
/// The [`Suit`] that appears most often.
///
/// # Panics
/// Panics if `cards` is empty.
///
/// # Example
/// ```
/// use ortalib::{Card, Suit, Rank};
/// use ortalab::poker::helpers::compute_most_appear_suit;
///
/// let cards = vec![
///     Card::new(Rank::Ace, Suit::Hearts, None, None),
///     Card::new(Rank::King, Suit::Hearts, None, None),
///     Card::new(Rank::Two, Suit::Clubs, None, None),
/// ];
///
/// assert_eq!(compute_most_appear_suit(&cards), Suit::Hearts);
/// ```
pub fn compute_most_appear_suit(cards: &[Card]) -> Suit {
    cards
        .iter()
        .counts_by(|c| c.suit)
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(suit, _)| suit)
        .unwrap()
}

/// Determines whether the given suit is present among the scored cards,
/// accounting for wild enhancements.
///
/// This is used by the *Flower Pot Joker* to check if a card of the desired
/// suit (or a wild card) is present.
///
/// # Arguments
/// * `on_scored` ‑ The cards currently being scored.
/// * `suit` ‑ The target suit to check for.
///
/// # Returns
/// `true` if the suit is present or a wild card substitutes for it.
///
/// # Example
/// ```
/// use ortalib::{Card, Suit, Rank, Enhancement};
/// use ortalab::poker::helpers::determine_current_suit;
///
/// let cards = vec![
///     Card::new(Rank::Ace, Suit::Spades, Some(Enhancement::Wild), None),
/// ];
///
/// assert!(determine_current_suit(&cards, Suit::Hearts));
/// ```
pub fn determine_current_suit(on_scored: &[Card], suit: Suit) -> bool {
    on_scored.iter().any(|c| {
        if c.suit == suit {
            return true;
        } else if let Some(enhance) = c.enhancement
            && enhance == Enhancement::Wild
        {
            return true;
        };
        false
    })
}

/// Determines whether at least two cards of the given color are present,
/// accounting for wild enhancements.
///
/// This is used for color‑based scoring rules.
///
/// # Arguments
/// * `on_scored` ‑ The cards currently being scored.
/// * `color` ‑ The target [`SuitColor`] to check for.
///
/// # Returns
/// `true` if at least two cards of the given color (or wilds) are present.
///
/// # Example
/// ```
/// use ortalib::{Card, Suit, Rank, SuitColor};
/// use ortalab::poker::helpers::determine_total_colors;
///
/// let cards = vec![
///     Card::new(Rank::Ace, Suit::Hearts, None, None),
///     Card::new(Rank::King, Suit::Diamonds, None, None),
/// ];
///
/// assert!(determine_total_colors(&cards, SuitColor::Red));
/// ```
pub fn determine_total_colors(on_scored: &[Card], color: SuitColor) -> bool {
    on_scored
        .iter()
        .filter(|card| {
            if card.suit.color() == color {
                return true;
            } else if let Some(enhance) = card.enhancement
                && enhance == Enhancement::Wild
            {
                return true;
            };
            false
        })
        .count()
        >= 2
}
