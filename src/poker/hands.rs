//! # Poker Hands
//!
//! This module defines functions for detecting specific poker hands
//! (e.g. pair, straight, flush, full house). Each function takes a slice
//! of [`Card`]s and returns the subset of cards that form the hand.
//!
//! These helpers are used by the scoring pipeline to determine the
//! strongest possible hand and compute its base chip/multiplier values.

use crate::poker::helpers::compute_most_appear_suit;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use ortalib::{Card, Enhancement, Joker, JokerCard, PokerHand, Rank};
use std::{
    collections::{HashMap, HashSet},
    ptr,
};

/// Detects a *High Card* hand.
///
/// A High Card is the fallback when no other hand is possible. It returns
/// the single highest‑ranked card.
///
/// Base scoring: **5 chips × 1 mult**
///
/// # Arguments
/// * `cards` — The cards to evaluate.
///
/// # Returns
/// A vector containing the highest card.
///
/// # Example
/// ```
/// use ortalib::{Card, Rank, Suit};
/// use ortalab::poker::hands::is_high_card;
///
/// let cards = vec![
///     Card::new(Rank::Two, Suit::Clubs, None, None),
///     Card::new(Rank::Ace, Suit::Hearts, None, None),
/// ];
///
/// let result = is_high_card(&cards);
/// assert_eq!(result.len(), 1);
/// assert_eq!(result[0].rank, Rank::Ace);
/// ```
pub fn is_high_card(cards: &[Card]) -> Vec<Card> {
    let card_to_return = cards
        .iter()
        .max_by_key(|&card| OrderedFloat(card.rank.rank_value()))
        .copied()
        .unwrap();
    let vec_to_return = vec![card_to_return];
    vec_to_return
}

/// Detects a *Pair* hand.
///
/// A Pair consists of two cards with the same rank (suits may differ).
///
/// Base scoring: **10 chips × 2 mult**
///
/// # Example
/// ```
/// use ortalib::{Card, Rank, Suit};
/// use ortalab::poker::hands::is_pair;
///
/// let cards = vec![
///     Card::new(Rank::King, Suit::Hearts, None, None),
///     Card::new(Rank::King, Suit::Spades, None, None),
///     Card::new(Rank::Three, Suit::Clubs, None, None),
/// ];
///
/// let result = is_pair(&cards);
/// assert_eq!(result.len(), 2);
/// assert!(result.iter().all(|c| c.rank == Rank::King));
/// ```
pub fn is_pair(cards: &[Card]) -> Vec<Card> {
    let mut card_to_return: Vec<Card> = Vec::new();

    for (curr, next) in cards.iter().tuple_windows() {
        if compute_card_order(*curr) == compute_card_order(*next) {
            card_to_return.push(*curr);
            card_to_return.push(*next);
            break;
        }
    }

    card_to_return
}

/// Detects a *Two Pair* hand.
///
/// A Two Pair consists of two cards with the same rank, and two cards with
/// another matching rank. Suits may differ.
///
/// Base scoring: **20 chips × 2 mult**
///
/// # Example
/// ```
/// use ortalib::{Card, Rank, Suit};
/// use ortalab::poker::hands::is_two_pair;
///
/// // Construct a hand with two pairs: Kings and Eights
/// let cards = vec![
///     Card::new(Rank::King, Suit::Hearts, None, None),
///     Card::new(Rank::King, Suit::Spades, None, None),
///     Card::new(Rank::Eight, Suit::Clubs, None, None),
///     Card::new(Rank::Eight, Suit::Diamonds, None, None),
///     Card::new(Rank::Two, Suit::Hearts, None, None),
/// ];
///
/// let result = is_two_pair(&cards);
///
/// // Should return exactly 4 cards (the two pairs)
/// assert_eq!(result.len(), 4);
/// assert!(result.iter().any(|c| c.rank == Rank::King));
/// assert!(result.iter().any(|c| c.rank == Rank::Eight));
/// ```
pub fn is_two_pair(cards: &[Card]) -> Vec<Card> {
    let mut card_to_return: Vec<Card> = Vec::new();
    let mut prev_rank = 0.0;

    for (curr, next) in cards.iter().tuple_windows() {
        let curr_order = compute_card_order(*curr);
        let next_order = compute_card_order(*next);
        if prev_rank != 0.0 {
            if curr_order == next_order && curr_order != prev_rank {
                card_to_return.push(*curr);
                card_to_return.push(*next);
            }
        } else if curr_order == next_order {
            card_to_return.push(*curr);
            card_to_return.push(*next);
            prev_rank = curr_order;
        }
    }

    card_to_return
}

/// Detects a *Three of a Kind* hand.
///
/// A Three of a Kind consists of three cards with the same rank
/// (suits may differ).
///
/// Base scoring: **30 chips × 3 mult**
///
/// # Arguments
/// * `cards` — The cards to evaluate.
///
/// # Returns
/// A vector containing the three cards that form the hand.  
/// If no three of a kind is found, returns an empty vector.
///
/// # Example
/// ```
/// use ortalib::{Card, Rank, Suit};
/// use ortalab::poker::hands::is_three_of_a_kind;
///
/// // Construct a hand with three Queens
/// let cards = vec![
///     Card::new(Rank::Queen, Suit::Hearts, None, None),
///     Card::new(Rank::Queen, Suit::Spades, None, None),
///     Card::new(Rank::Queen, Suit::Clubs, None, None),
///     Card::new(Rank::Nine, Suit::Diamonds, None, None),
///     Card::new(Rank::Two, Suit::Hearts, None, None),
/// ];
///
/// let result = is_three_of_a_kind(&cards);
///
/// // Should return exactly 3 cards (the three Queens)
/// assert_eq!(result.len(), 3);
/// assert!(result.iter().all(|c| c.rank == Rank::Queen));
/// ```
pub fn is_three_of_a_kind(cards: &[Card]) -> Vec<Card> {
    let mut card_to_return: Vec<Card> = Vec::new();
    let mut prev_rank = 0.0;

    for (curr, next) in cards.iter().tuple_windows() {
        let curr_order = compute_card_order(*curr);
        let next_order = compute_card_order(*next);
        if curr_order == next_order {
            prev_rank = curr_order;
            card_to_return.push(*curr);
            if let Some(last) = cards.last()
                && ptr::eq(next, last)
            {
                card_to_return.push(*next);
            }
        } else if curr_order == prev_rank {
            card_to_return.push(*curr);
        }
    }
    card_to_return
}

/// Detects a *Straight* hand.
///
/// A Straight is five cards in consecutive rank order, not all of the same suit.
/// Aces may be counted high or low.
///
/// Base scoring: **30 chips × 4 mult**
///
/// # Example
/// ```
/// use ortalib::{Card, Rank, Suit};
/// use ortalab::poker::hands::is_straight;
///
/// let cards = vec![
///     Card::new(Rank::Five, Suit::Clubs, None, None),
///     Card::new(Rank::Six, Suit::Hearts, None, None),
///     Card::new(Rank::Seven, Suit::Spades, None, None),
///     Card::new(Rank::Eight, Suit::Diamonds, None, None),
///     Card::new(Rank::Nine, Suit::Clubs, None, None),
/// ];
///
/// let result = is_straight(&cards);
/// assert_eq!(result.len(), 5);
/// ```
pub fn is_straight(cards: &[Card]) -> Vec<Card> {
    let mut card_to_return: HashMap<Card, i32> = HashMap::new();

    // Check for consecutive cards with values from 2 - A
    for (curr, next) in cards.iter().tuple_windows() {
        let curr_order = compute_card_order(*curr);
        let next_order = compute_card_order(*next);
        if next_order - curr_order == 1.0 {
            if !card_to_return.contains_key(curr) {
                card_to_return.insert(*curr, 1);
            }

            if !card_to_return.contains_key(next) {
                card_to_return.insert(*next, 1);
            }
        }
        // Handle case where Ace is the lowest value card (below 2)
        else if next.rank == Rank::Ace && curr.rank == Rank::Five {
            card_to_return.insert(*next, 1);
        } else if next_order - curr_order != 1.0 {
            card_to_return.clear();
        }
    }
    card_to_return.iter().map(|(&card, _)| card).collect_vec()
}

/// Detects a *Flush* hand.
///
/// A Flush is five cards of any rank, all from the same suit.  
/// Wild cards may substitute for missing suits.
///
/// Base scoring: **35 chips × 4 mult**
///
/// # Example
/// ```
/// use ortalib::{Card, Rank, Suit};
/// use ortalab::poker::hands::is_flush;
///
/// let cards = vec![
///     Card::new(Rank::Ace, Suit::Hearts, None, None),
///     Card::new(Rank::Ten, Suit::Hearts, None, None),
///     Card::new(Rank::Four, Suit::Hearts, None, None),
///     Card::new(Rank::Seven, Suit::Hearts, None, None),
///     Card::new(Rank::Two, Suit::Hearts, None, None),
/// ];
///
/// let result = is_flush(&cards);
/// assert_eq!(result.len(), 5);
/// ```
pub fn is_flush(cards: &[Card]) -> Vec<Card> {
    let mut card_to_return: Vec<Card> = Vec::new();
    let base_suit = compute_most_appear_suit(cards);

    cards.iter().for_each(|card| {
        if card.suit == base_suit {
            card_to_return.push(*card);
        } else if let Some(enhance) = card.enhancement
            && enhance == Enhancement::Wild
        {
            card_to_return.push(*card);
        }
    });
    card_to_return
}

/// Detects a *Full House* hand.
///
/// A Full House consists of three cards with the same rank and two cards
/// with another matching rank. Suits may differ, but the hand must contain
/// at least two different suits.
///
/// Base scoring: **40 chips × 4 mult**
///
/// # Arguments
/// * `cards` — The cards to evaluate.
///
/// # Returns
/// A vector containing the five cards that form the full house.  
/// If no full house is found, returns an empty vector.
///
/// # Example
/// ```
/// use ortalib::{Card, Rank, Suit};
/// use ortalab::poker::hands::is_full_house;
///
/// // Construct a hand with a full house: three Tens and two Fours
/// let cards = vec![
///     Card::new(Rank::Ten, Suit::Hearts, None, None),
///     Card::new(Rank::Ten, Suit::Spades, None, None),
///     Card::new(Rank::Ten, Suit::Clubs, None, None),
///     Card::new(Rank::Four, Suit::Diamonds, None, None),
///     Card::new(Rank::Four, Suit::Hearts, None, None),
/// ];
///
/// let result = is_full_house(&cards);
///
/// // Should return exactly 5 cards (the full house)
/// assert_eq!(result.len(), 5);
/// assert!(result.iter().any(|c| c.rank == Rank::Ten));
/// assert!(result.iter().any(|c| c.rank == Rank::Four));
/// ```
pub fn is_full_house(cards: &[Card]) -> Vec<Card> {
    let mut card_to_return: Vec<Card> = Vec::new();
    let mut prev_rank = 0.0;

    for (curr, next) in cards.iter().tuple_windows() {
        let curr_order = compute_card_order(*curr);
        let next_order = compute_card_order(*next);

        if prev_rank != 0.0 && curr_order == next_order && curr_order != prev_rank {
            card_to_return.push(*curr);
            if let Some(last) = cards.last()
                && ptr::eq(next, last)
            {
                card_to_return.push(*next);
            }
            continue;
        }
        if curr_order == next_order {
            prev_rank = curr_order;
            card_to_return.push(*curr);
            if let Some(last) = cards.last()
                && ptr::eq(next, last)
            {
                card_to_return.push(*next);
            }
        } else if curr_order == prev_rank {
            card_to_return.push(*curr);
        }
    }
    card_to_return
}

/// Detects a *Four of a Kind* hand.
///
/// Four of a Kind consists of four cards with the same rank (suits may differ).
///
/// Base scoring: **60 chips × 4 mult**
///
/// # Arguments
/// * `cards` — The cards to evaluate.
///
/// # Returns
/// A vector containing the four cards that form the hand.  
/// If no four of a kind is found, returns an empty vector.
///
/// # Example
/// ```
/// use ortalib::{Card, Rank, Suit};
/// use ortalab::poker::hands::is_four_of_a_kind;
///
/// // Construct a hand with four Jacks
/// let cards = vec![
///     Card::new(Rank::Jack, Suit::Hearts, None, None),
///     Card::new(Rank::Jack, Suit::Spades, None, None),
///     Card::new(Rank::Jack, Suit::Clubs, None, None),
///     Card::new(Rank::Jack, Suit::Diamonds, None, None),
///     Card::new(Rank::Two, Suit::Hearts, None, None),
/// ];
///
/// let result = is_four_of_a_kind(&cards);
///
/// // Should return exactly 4 cards (the four Jacks)
/// assert_eq!(result.len(), 4);
/// assert!(result.iter().all(|c| c.rank == Rank::Jack));
/// ```
pub fn is_four_of_a_kind(cards: &[Card]) -> Vec<Card> {
    let mut card_to_return: Vec<Card> = Vec::new();
    let mut prev_rank = 0.0;

    for (curr, next) in cards.iter().tuple_windows() {
        let curr_order = compute_card_order(*curr);
        let next_order = compute_card_order(*next);

        if prev_rank == 0.0 && curr_order == next_order {
            prev_rank = curr_order;
            card_to_return.push(*curr);
            continue;
        }

        if curr_order == next_order && curr_order == prev_rank {
            card_to_return.push(*curr);
            if let Some(last) = cards.last()
                && ptr::eq(next, last)
            {
                card_to_return.push(*next);
            }
        } else if curr_order != next_order && curr_order == prev_rank {
            card_to_return.push(*curr);
        }
    }
    card_to_return
}

/// Detects a *Straight Flush* hand.
///
/// A Straight Flush is five cards in consecutive rank order, all from the same suit.  
/// Aces may be counted high or low.
///
/// Base scoring: **100 chips × 8 mult**
///
/// # Arguments
/// * `cards` — The cards to evaluate.
/// * `is_four_finger_exists` — Whether a special joker/condition allows a 4‑card straight flush.
///
/// # Returns
/// A vector containing the cards that form the straight flush.  
/// If no straight flush is found, returns an empty vector.
///
/// # Example
/// ```
/// use ortalib::{Card, Rank, Suit};
/// use ortalab::poker::hands::is_straight_flush;
///
/// // Construct a hand with a straight flush: 5♥, 6♥, 7♥, 8♥, 9♥
/// let cards = vec![
///     Card::new(Rank::Five, Suit::Hearts, None, None),
///     Card::new(Rank::Six, Suit::Hearts, None, None),
///     Card::new(Rank::Seven, Suit::Hearts, None, None),
///     Card::new(Rank::Eight, Suit::Hearts, None, None),
///     Card::new(Rank::Nine, Suit::Hearts, None, None),
/// ];
///
/// let result = is_straight_flush(&cards, false);
///
/// // Should return exactly 5 cards (the straight flush)
/// assert_eq!(result.len(), 5);
/// assert!(result.iter().all(|c| c.suit == Suit::Hearts));
/// ```
pub fn is_straight_flush(cards: &[Card], is_four_finger_exists: bool) -> Vec<Card> {
    // let card_to_return = cards.clone();
    let mut returned_card_from_is_flush = is_flush(cards);
    let mut returned_card_from_is_straight = is_straight(cards);

    if returned_card_from_is_flush.len() == 5 && returned_card_from_is_straight.len() == 5 {
        return cards.to_vec();
    } else if is_four_finger_exists
        && ((returned_card_from_is_flush.len() == 4 && returned_card_from_is_straight.len() == 4)
            || (returned_card_from_is_flush.len() == 5
                && returned_card_from_is_straight.len() == 4)
            || (returned_card_from_is_flush.len() == 4
                && returned_card_from_is_straight.len() == 5))
    {
        returned_card_from_is_flush.append(&mut returned_card_from_is_straight);
        let returned_card_set = returned_card_from_is_flush.iter().collect::<HashSet<_>>();
        return returned_card_set.iter().map(|&&card| card).collect_vec();
    }

    vec![]
}

/// Detects a *Five of a Kind* hand (illegal in standard poker).
///
/// Five of a Kind consists of five cards with the same rank, not all of the same suit.
/// This hand is only possible with wild cards or jokers.
///
/// Base scoring: **120 chips × 12 mult**
///
/// # Arguments
/// * `cards` — The cards to evaluate.
///
/// # Returns
/// A vector containing the five cards that form the hand.  
/// If no five of a kind is found, returns an empty vector.
///
/// # Example
/// ```
/// use ortalib::{Card, Rank, Suit};
/// use ortalab::poker::hands::is_five_of_a_kind;
///
/// // Construct a hand with five Aces (requires wilds/jokers in real play)
/// let cards = vec![
///     Card::new(Rank::Ace, Suit::Hearts, None, None),
///     Card::new(Rank::Ace, Suit::Spades, None, None),
///     Card::new(Rank::Ace, Suit::Clubs, None, None),
///     Card::new(Rank::Ace, Suit::Diamonds, None, None),
///     Card::new(Rank::Ace, Suit::Hearts, None, None),
/// ];
///
/// let result = is_five_of_a_kind(&cards);
///
/// assert_eq!(result.len(), 5);
/// assert!(result.iter().all(|c| c.rank == Rank::Ace));
/// ```
pub fn is_five_of_a_kind(cards: &[Card]) -> Vec<Card> {
    let mut card_to_return: Vec<Card> = Vec::new();
    for (curr, next) in cards.iter().tuple_windows() {
        let curr_order = compute_card_order(*curr);
        let next_order = compute_card_order(*next);
        if curr_order == next_order {
            card_to_return.push(*curr);
            if let Some(last) = cards.last()
                && ptr::eq(next, last)
            {
                card_to_return.push(*next);
            }
        }
    }
    card_to_return
}

/// Detects a *Flush House* hand (illegal in standard poker).
///
/// A Flush House consists of three cards with one rank and two cards with another rank,  
/// all from the same suit (or with wilds substituting).
///
/// Base scoring: **140 chips × 14 mult**
///
/// # Arguments
/// * `cards` — The cards to evaluate.
///
/// # Returns
/// A vector containing the five cards that form the flush house.  
/// If no flush house is found, returns an empty vector.
///
/// # Example
/// ```
/// use ortalib::{Card, Rank, Suit};
/// use ortalab::poker::hands::is_flush_house;
///
/// // Construct a flush house: three Tens and two Fours, all Hearts
/// let cards = vec![
///     Card::new(Rank::Ten, Suit::Hearts, None, None),
///     Card::new(Rank::Ten, Suit::Hearts, None, None),
///     Card::new(Rank::Ten, Suit::Hearts, None, None),
///     Card::new(Rank::Four, Suit::Hearts, None, None),
///     Card::new(Rank::Four, Suit::Hearts, None, None),
/// ];
///
/// let result = is_flush_house(&cards);
///
/// assert_eq!(result.len(), 5);
/// assert!(result.iter().any(|c| c.rank == Rank::Ten));
/// assert!(result.iter().any(|c| c.rank == Rank::Four));
/// assert!(result.iter().all(|c| c.suit == Suit::Hearts));
/// ```
pub fn is_flush_house(cards: &[Card]) -> Vec<Card> {
    let mut card_to_return: Vec<Card> = Vec::new();
    let mut prev_rank = 0.0;
    let base_suit = compute_most_appear_suit(cards);

    for (curr, next) in cards.iter().tuple_windows() {
        let curr_order = compute_card_order(*curr);
        let next_order = compute_card_order(*next);

        if prev_rank != 0.0 && curr_order == next_order && curr_order != prev_rank {
            if curr.suit == base_suit {
                card_to_return.push(*curr);
            } else if let Some(enhance) = curr.enhancement
                && enhance == Enhancement::Wild
            {
                card_to_return.push(*curr);
            }

            if let Some(last) = cards.last()
                && ptr::eq(next, last)
                && next.suit == base_suit
            {
                card_to_return.push(*next);
            } else if let Some(enhance) = next.enhancement
                && enhance == Enhancement::Wild
            {
                card_to_return.push(*next);
            }
            continue;
        }

        if curr_order == next_order {
            prev_rank = curr_order;
            if curr.suit == base_suit {
                card_to_return.push(*curr);
            } else if let Some(enhance) = curr.enhancement
                && enhance == Enhancement::Wild
            {
                card_to_return.push(*curr);
            }
        } else if curr_order == prev_rank && curr.suit == base_suit {
            card_to_return.push(*curr);
        } else if let Some(enhance) = curr.enhancement
            && enhance == Enhancement::Wild
        {
            card_to_return.push(*curr);
        }
    }
    card_to_return
}

/// Detects a *Flush Five* hand (illegal in standard poker).
///
/// A Flush Five consists of five cards with the same rank and the same suit.  
/// This is only possible with wilds/jokers in real play.
///
/// Base scoring: **160 chips × 16 mult**
///
/// # Arguments
/// * `cards` — The cards to evaluate.
///
/// # Returns
/// A vector containing the five cards that form the flush five.  
/// If no flush five is found, returns an empty vector.
///
/// # Example
/// ```
/// use ortalib::{Card, Rank, Suit};
/// use ortalab::poker::hands::is_flush_five;
///
/// // Construct a flush five: five Aces of Hearts (requires wilds/jokers in real play)
/// let cards = vec![
///     Card::new(Rank::Ace, Suit::Hearts, None, None),
///     Card::new(Rank::Ace, Suit::Hearts, None, None),
///     Card::new(Rank::Ace, Suit::Hearts, None, None),
///     Card::new(Rank::Ace, Suit::Hearts, None, None),
///     Card::new(Rank::Ace, Suit::Hearts, None, None),
/// ];
///
/// let result = is_flush_five(&cards);
///
/// assert_eq!(result.len(), 5);
/// assert!(result.iter().all(|c| c.rank == Rank::Ace && c.suit == Suit::Hearts));
/// ```
pub fn is_flush_five(cards: &[Card]) -> Vec<Card> {
    let mut card_to_return: Vec<Card> = Vec::new();
    let base_suit = cards.first().unwrap().suit;

    for (curr, next) in cards.iter().tuple_windows() {
        let curr_order = compute_card_order(*curr);
        let next_order = compute_card_order(*next);
        if curr_order == next_order && curr.suit == base_suit {
            card_to_return.push(*curr);
            if let Some(last) = cards.last()
                && ptr::eq(next, last)
            {
                card_to_return.push(*next);
            }
        }
    }
    card_to_return
}

pub fn compute_card_order(card: Card) -> f64 {
    match card.rank {
        Rank::Two => 2.0,
        Rank::Three => 3.0,
        Rank::Four => 4.0,
        Rank::Five => 5.0,
        Rank::Six => 6.0,
        Rank::Seven => 7.0,
        Rank::Eight => 8.0,
        Rank::Nine => 9.0,
        Rank::Ten => 10.0,
        Rank::Jack => 11.0,
        Rank::Queen => 12.0,
        Rank::King => 13.0,
        Rank::Ace => 14.0,
    }
}

/// Determines the strongest poker hand from a set of cards (and optional jokers).
///
/// This function sorts the cards, then checks for each possible hand type
/// in descending order of strength (from Flush Five down to High Card).
/// It returns the first matching hand along with the cards that form it.
///
/// # Arguments
/// * `cards` — The cards to evaluate.
/// * `jokers` — Any joker cards in play, which may affect hand detection.
///
/// # Returns
/// A tuple `(PokerHand, Vec<Card>)` where:
/// - `PokerHand` is the detected hand type.
/// - `Vec<Card>` is the subset of cards that form the hand.
///
/// # Example
/// ```
/// use ortalib::{Card, Rank, Suit, JokerCard, PokerHand};
/// use ortalab::poker::hands::determine_poker_hand;
///
/// // Construct a simple hand: Pair of Kings
/// let cards = vec![
///     Card::new(Rank::King, Suit::Hearts, None, None),
///     Card::new(Rank::King, Suit::Spades, None, None),
///     Card::new(Rank::Three, Suit::Clubs, None, None),
///     Card::new(Rank::Seven, Suit::Diamonds, None, None),
///     Card::new(Rank::Nine, Suit::Hearts, None, None),
/// ];
///
/// let jokers: Vec<JokerCard> = vec![];
///
/// let (hand, selected) = determine_poker_hand(&cards, &jokers);
///
/// assert_eq!(hand, PokerHand::Pair);
/// assert_eq!(selected.len(), 2);
/// assert!(selected.iter().all(|c| c.rank == Rank::King));
/// ```
pub fn determine_poker_hand(cards: &[Card], jokers: &[JokerCard]) -> (PokerHand, Vec<Card>) {
    let mut return_card;
    let sorted_cards_played: Vec<_> = cards
        .iter()
        .sorted_by_key(|&card| OrderedFloat(compute_card_order(*card)))
        .copied()
        .collect();
    let is_four_finger_exists = jokers.iter().any(|card| card.joker == Joker::FourFingers);

    // Check if a flush five  exists
    return_card = is_flush_five(&sorted_cards_played);
    if return_card.len() == 5 || (return_card.len() == 4 && is_four_finger_exists) {
        return (PokerHand::FlushFive, cards.to_vec());
    }

    // Check if a flush house  exists
    return_card = is_flush_house(&sorted_cards_played);
    if return_card.len() == 5 {
        return (PokerHand::FlushHouse, cards.to_vec());
    }

    // Check if a five of a kind  exists
    return_card = is_five_of_a_kind(&sorted_cards_played);
    if return_card.len() == 5 {
        return (PokerHand::FiveOfAKind, cards.to_vec());
    }

    // Check if a straight flush exists
    return_card = is_straight_flush(&sorted_cards_played, is_four_finger_exists);
    // println!("return: {:?}", return_card);
    if return_card.len() == 5 {
        return (PokerHand::StraightFlush, cards.to_vec());
    } else if return_card.len() == 4 && is_four_finger_exists {
        return (PokerHand::StraightFlush, return_card);
    }

    // Check if a four of a kind  exists
    return_card = is_four_of_a_kind(&sorted_cards_played);
    if return_card.len() == 4 {
        return (PokerHand::FourOfAKind, return_card);
    }

    // Check if a full house exists
    return_card = is_full_house(&sorted_cards_played);
    if return_card.len() == 5 {
        return (PokerHand::FullHouse, cards.to_vec());
    }

    // Check if a flush exists
    return_card = is_flush(&sorted_cards_played);
    if return_card.len() == 5 || (return_card.len() == 4 && is_four_finger_exists) {
        return (PokerHand::Flush, cards.to_vec());
    }

    // Check if a straight exists
    return_card = is_straight(&sorted_cards_played);
    if return_card.len() == 5 {
        return (PokerHand::Straight, cards.to_vec());
    } else if return_card.len() == 4 && is_four_finger_exists {
        return (PokerHand::Straight, return_card);
    }

    // Check if a three of a kind exists
    return_card = is_three_of_a_kind(&sorted_cards_played);
    if return_card.len() == 3 {
        return (PokerHand::ThreeOfAKind, return_card);
    }

    // Check if a pair exists
    return_card = is_two_pair(&sorted_cards_played);
    if return_card.len() == 4 {
        return (PokerHand::TwoPair, return_card);
    }

    // Check if a pair exists
    return_card = is_pair(&sorted_cards_played);
    if return_card.len() == 2 {
        return (PokerHand::Pair, return_card);
    }

    // Default/base case when no other poker hands exist
    (PokerHand::HighCard, is_high_card(&sorted_cards_played))
}
