//! # Jokers
//!
//! This module defines how special Joker cards modify chip and multiplier values.
//! Jokers are divided into categories (easy, medium, hard, etc.), each applying
//! different rules depending on the current hand, held cards, or scored cards.
//!
//! These functions are called by the scoring pipeline to apply Joker effects
//! after the base hand value and enhancements/editions are computed.

use crate::poker::{
    apply_edition, compute_card_order, determine_current_suit, determine_total_colors,
};
use ordered_float::OrderedFloat;
use ortalib::{Card, Chips, Enhancement, Joker, JokerCard, Mult, PokerHand, Rank, Suit, SuitColor};

/// Applies the effects of "easy" Jokers to the current score.
///
/// Easy Jokers include:
/// - **Base Joker**: +4 multiplier
/// - **Jolly Joker**: +8 multiplier if the hand is a pair, trips, full house, etc.
/// - **Sly Joker**: +50 chips for the same hand types
/// - **Zany Joker**: +12 multiplier for trips/full house/quads/etc.
/// - **Wily Joker**: +100 chips for the same
/// - **Mad Joker**: +10 multiplier for two pair/full house
/// - **Clever Joker**: +80 chips for the same
/// - **Crazy Joker**: +12 multiplier for straights
/// - **Devious Joker**: +100 chips for straights
/// - **Droll Joker**: +10 multiplier for flushes/quads
/// - **Crafty Joker**: +80 chips for the same
/// - **Abstract Joker**: +3 multiplier per Joker card in play
///
/// # Arguments
/// * `joker` — The Joker being applied.
/// * `hand` — The detected [`PokerHand`].
/// * `joker_cards_len` — Total number of jokers in play.
/// * `chip` — Current chip value.
/// * `mul` — Current multiplier value.
///
/// # Returns
/// A tuple `(Chips, Mult)` with updated values.
///
/// # Example
/// ```
/// use ortalib::{PokerHand, Joker};
/// use ortalab::poker::jokers::apply_easy_jokers;
///
/// // Base Joker always adds +4 multiplier
/// let (chips, mult) = apply_easy_jokers(Joker::Joker, PokerHand::Pair, 1, 100.0, 1.0);
/// assert_eq!(chips, 100.0);
/// assert_eq!(mult, 5.0);
/// ```
pub fn apply_easy_jokers(
    joker: Joker,
    hand: PokerHand,
    joker_cards_len: usize,
    chip: f64,
    mul: f64,
) -> (Chips, Mult) {
    let mut res = (chip, mul);

    // Base Joker
    if joker == Joker::Joker {
        res.1 += 4.0;
        // dbg!(res);
    }

    // Jolly and Sly Joker
    let mut check = vec![
        PokerHand::Pair,
        PokerHand::TwoPair,
        PokerHand::FullHouse,
        PokerHand::ThreeOfAKind,
        PokerHand::FourOfAKind,
        PokerHand::FiveOfAKind,
        PokerHand::FlushHouse,
        PokerHand::FlushFive,
    ];
    if check.contains(&hand) {
        if joker == Joker::JollyJoker {
            res.1 += 8.0;
            // dbg!(res);
        } else if joker == Joker::SlyJoker {
            res.0 += 50.0;
        }
    }

    // Zany and Willy Joker
    check = vec![
        PokerHand::ThreeOfAKind,
        PokerHand::FullHouse,
        PokerHand::FlushHouse,
        PokerHand::FourOfAKind,
        PokerHand::FiveOfAKind,
        PokerHand::FlushFive,
    ];
    if check.contains(&hand) {
        if joker == Joker::ZanyJoker {
            res.1 += 12.0;
            // dbg!(res);
        } else if joker == Joker::WilyJoker {
            res.0 += 100.0;
        }
    }

    // Mad and Clever Joker
    check = vec![
        PokerHand::TwoPair,
        PokerHand::FullHouse,
        PokerHand::FlushHouse,
    ];
    if check.contains(&hand) {
        if joker == Joker::MadJoker {
            res.1 += 10.0;
        } else if joker == Joker::CleverJoker {
            res.0 += 80.0;
        }
    }

    // Crazy and Devious Joker
    check = vec![PokerHand::Straight, PokerHand::StraightFlush];
    if check.contains(&hand) {
        if joker == Joker::CrazyJoker {
            res.1 += 12.0;
        } else if joker == Joker::DeviousJoker {
            res.0 += 100.0;
        }
    }

    // Droll and Crafty Joker
    check = vec![
        PokerHand::Flush,
        PokerHand::FlushFive,
        PokerHand::FlushHouse,
        PokerHand::FourOfAKind,
    ];
    if check.contains(&hand) {
        if joker == Joker::DrollJoker {
            res.1 += 10.0;
        } else if joker == Joker::CraftyJoker {
            res.0 += 80.0;
        }
    }

    // Abstract Joker
    if joker == Joker::AbstractJoker {
        res.1 += 3.0 * joker_cards_len as f64;
        // dbg!(res);
    }

    (res.0, res.1)
}

/// Applies the effects of "medium" Jokers to the current score.
///
/// Medium Jokers include:
/// - **Raised Fist**: Adds multiplier based on lowest rank in hand
/// - **Blackboard Joker**: Triples multiplier if all held cards are black suits
/// - **Baron Joker**: Multiplier ×1.5 for each King held
/// - **Greedy/Lusty/Wrathful/Gluttonous Jokers**: +3 multiplier per card of a specific suit
/// - **Fibonacci Joker**: +8 multiplier for cards with Fibonacci ranks (A,2,3,5,8)
/// - **Scary Face**: +30 chips per face card
/// - **Even Steven**: +4 multiplier per even‑ranked card ≤ 10
/// - **Odd Todd**: +31 chips per odd‑ranked card (or Ace)
/// - **Photograph**: Doubles multiplier if at least one face card is present
/// - **Smiley Face**: +5 multiplier per face card
/// - **Flower Pot**: Triples multiplier if all suits (or both colors with Smear) are present
///
/// # Arguments
/// * `joker` — The Joker being applied.
/// * `on_held` — Cards currently held in hand.
/// * `on_scored` — Cards being scored.
/// * `chip` — Current chip value.
/// * `mul` — Current multiplier value.
/// * `is_pareidolia_exists` — Whether Pareidolia Joker is active (treats all cards as faces).
/// * `is_smeared_exists` — Whether Smear Joker is active (treats red/black suits as equivalent).
///
/// # Returns
/// A tuple `(Chips, Mult)` with updated values.
///
/// # Example
/// ```
/// use ortalib::{Card, Rank, Suit, Joker};
/// use ortalab::poker::jokers::apply_medium_jokers;
///
/// // Even Steven: +4 multiplier for even ranks ≤ 10
/// let cards = vec![
///     Card::new(Rank::Two, Suit::Clubs, None, None),
///     Card::new(Rank::Four, Suit::Hearts, None, None),
/// ];
///
/// let (chips, mult) = apply_medium_jokers(
///     Joker::EvenSteven,
///     &[],
///     &cards,
///     100.0,
///     1.0,
///     false,
///     false,
/// );
///
/// assert_eq!(chips, 100.0);
/// assert_eq!(mult, 9.0); // +4 for each even card
/// ```
pub fn apply_medium_jokers(
    joker: Joker,
    on_held: &[Card],
    on_scored: &[Card],
    chip: f64,
    mul: f64,
    is_pareidolia_exists: bool,
    is_smeared_exists: bool,
) -> (Chips, Mult) {
    let mut res = (chip, mul);
    let mut on_held_iter = on_held.iter();

    if joker == Joker::RaisedFist {
        let lowest_rank_card = on_held
            .iter()
            .min_by_key(|&card| OrderedFloat(card.rank.rank_value()))
            .unwrap();
        let mut vec_lowest_rank_card = on_held
            .iter()
            .filter(|&card| card.rank.rank_value() == lowest_rank_card.rank.rank_value());
        res.1 += vec_lowest_rank_card.next_back().unwrap().rank.rank_value() * 2.0;

        // TODO retriggers
    }

    // Blackboard Joker
    if joker == Joker::Blackboard {
        let vec_to_check = [Suit::Spades, Suit::Clubs];
        let check = on_held_iter.all(|c| {
            if let Some(enhance) = c.enhancement
                && enhance == Enhancement::Wild
            {
                return true;
            }
            vec_to_check.contains(&c.suit)
        });

        if check || on_held.is_empty() {
            res.1 *= 3.0;
        }
    }

    // Baron Joker
    if joker == Joker::Baron {
        on_held_iter.for_each(|&card| {
            if card.rank == Rank::King {
                res.1 *= 1.5;
            }
        });
        // dbg!(res);
    }

    // Greedy Joker
    if joker == Joker::GreedyJoker {
        on_scored.iter().for_each(|&card| {
            if card.suit == Suit::Diamonds || (card.suit == Suit::Hearts && is_smeared_exists) {
                res.1 += 3.0;
            } else if let Some(enhance) = card.enhancement
                && enhance == Enhancement::Wild
            {
                res.1 += 3.0;
            }
        })
    }

    // Lusty Joker
    if joker == Joker::LustyJoker {
        on_scored.iter().for_each(|&card| {
            if card.suit == Suit::Hearts || (card.suit == Suit::Diamonds && is_smeared_exists) {
                res.1 += 3.0;
            } else if let Some(enhance) = card.enhancement
                && enhance == Enhancement::Wild
            {
                res.1 += 3.0;
            }
        })
    }

    // Wrathful Joker
    if joker == Joker::WrathfulJoker {
        on_scored.iter().for_each(|&card| {
            if card.suit == Suit::Spades || (card.suit == Suit::Clubs && is_smeared_exists) {
                res.1 += 3.0;
            } else if let Some(enhance) = card.enhancement
                && enhance == Enhancement::Wild
            {
                res.1 += 3.0;
            }
        })
    }

    // Gluttonus Joker
    if joker == Joker::GluttonousJoker {
        on_scored.iter().for_each(|&card| {
            if card.suit == Suit::Clubs || (card.suit == Suit::Spades && is_smeared_exists) {
                res.1 += 3.0;
            } else if let Some(enhance) = card.enhancement
                && enhance == Enhancement::Wild
            {
                res.1 += 3.0;
            }
        })
    }

    // Fibonacci Joker
    if joker == Joker::Fibonacci {
        on_scored.iter().for_each(|&card| {
            if card.rank == Rank::Ace
                || card.rank == Rank::Two
                || card.rank == Rank::Three
                || card.rank == Rank::Five
                || card.rank == Rank::Eight
            {
                res.1 += 8.0;
            }
        })
    }

    // Scary Face
    if joker == Joker::ScaryFace {
        on_scored.iter().for_each(|&card| {
            if card.rank.is_face() || is_pareidolia_exists {
                res.0 += 30.0;
            }
        })
    }

    // Even Steven
    if joker == Joker::EvenSteven {
        on_scored.iter().for_each(|&card| {
            let value = compute_card_order(card);
            if value <= 10.0 && value % 2.0 == 0.0 {
                res.1 += 4.0;
            }
        })
    }

    // Odd Todd
    if joker == Joker::OddTodd {
        on_scored.iter().for_each(|&card| {
            let value = compute_card_order(card);
            if value == 14.0 || (value < 10.0 && value % 2.0 != 0.0) {
                res.0 += 31.0;
            }
        })
    }

    // Photograph
    if joker == Joker::Photograph {
        let mut firt_check = false;
        on_scored.iter().for_each(|&card| {
            if (card.rank.is_face() || is_pareidolia_exists) && !firt_check {
                res.1 *= 2.0;
                firt_check = true;
            }

            // TODO: Handle retriggers
        })
    }

    // Smiley Face
    if joker == Joker::SmileyFace {
        on_scored.iter().for_each(|&card| {
            if card.rank.is_face() || is_pareidolia_exists {
                res.1 += 5.0;
            }
        })
    }

    // Flower pot
    if joker == Joker::FlowerPot && on_scored.len() >= 4 {
        let having_diamonds = determine_current_suit(on_scored, Suit::Diamonds);
        let having_hearts = determine_current_suit(on_scored, Suit::Hearts);
        let having_spades = determine_current_suit(on_scored, Suit::Spades);
        let having_clubs = determine_current_suit(on_scored, Suit::Clubs);
        let having_red_cards = determine_total_colors(on_scored, SuitColor::Red);
        let having_black_cards = determine_total_colors(on_scored, SuitColor::Black);

        if (having_diamonds && having_hearts && having_spades && having_clubs)
            || (is_smeared_exists && having_red_cards && having_black_cards)
        {
            res.1 *= 3.0
        }
    }

    (res.0, res.1)
}

/// Applies all Joker effects to the current score.
///
/// This function iterates over all Joker cards in play and applies their
/// effects depending on their category:
///
/// - **Independent Jokers** (e.g. Base Joker, Jolly Joker, Abstract Joker, Blackboard, Flower Pot)
///   are applied regardless of held/scored context.
/// - **On‑scored Jokers** (e.g. Greedy, Lusty, Wrathful, Gluttonous, Fibonacci, Scary Face, etc.)
///   are applied based on the cards currently being scored.
/// - **On‑held Jokers** (e.g. Raised Fist, Baron, Mime) are applied based on cards still in hand.
/// - **Editions** attached to Joker cards are also applied at the end.
///
/// The function also checks for global Joker effects like **Pareidolia** (treat all cards as faces)
/// and **Smeared Joker** (treat red/black suits as equivalent).
///
/// # Arguments
/// * `joker_cards` — The Joker cards in play.
/// * `on_held_cards` — Cards currently held in hand.
/// * `on_scored_cards` — Cards being scored.
/// * `hand` — The detected [`PokerHand`] for this round.
/// * `chip` — Current chip value before Jokers.
/// * `mul` — Current multiplier value before Jokers.
///
/// # Returns
/// A tuple `(Chips, Mult)` with updated values after all Joker effects.
///
/// # Example
/// ```
/// use ortalib::{Card, Rank, Suit, Joker, JokerCard, PokerHand};
/// use ortalab::poker::jokers::joker_application;
///
/// // Start with a simple hand: Pair of Kings
/// let scored = vec![
///     Card::new(Rank::King, Suit::Hearts, None, None),
///     Card::new(Rank::King, Suit::Spades, None, None),
/// ];
///
/// // Add a Base Joker (always +4 multiplier)
/// let jokers = vec![JokerCard::new(Joker::Joker, None)];
///
/// let (chips, mult) = joker_application(&jokers, &[], &scored, PokerHand::Pair, 100.0, 1.0);
///
/// assert_eq!(chips, 100.0);
/// assert_eq!(mult, 5.0); // base 1.0 + 4.0 from Joker
/// ```
pub fn joker_application(
    joker_cards: &[JokerCard],
    on_held_cards: &[Card],
    on_scored_cards: &[Card],
    hand: PokerHand,
    chip: f64,
    mul: f64,
) -> (Chips, Mult) {
    let mut new_result = (chip, mul);
    let independent_jokers = [
        Joker::Joker,
        Joker::JollyJoker,
        Joker::ZanyJoker,
        Joker::MadJoker,
        Joker::CrazyJoker,
        Joker::DrollJoker,
        Joker::SlyJoker,
        Joker::WilyJoker,
        Joker::CleverJoker,
        Joker::DeviousJoker,
        Joker::CraftyJoker,
        Joker::AbstractJoker,
        Joker::Blackboard,
        Joker::FlowerPot,
    ];
    let on_scored_jokers = [
        Joker::GreedyJoker,
        Joker::LustyJoker,
        Joker::WrathfulJoker,
        Joker::GluttonousJoker,
        Joker::Fibonacci,
        Joker::ScaryFace,
        Joker::EvenSteven,
        Joker::OddTodd,
        Joker::Photograph,
        Joker::SmileyFace,
        Joker::SockAndBuskin,
    ];
    let on_held_jokers = [Joker::RaisedFist, Joker::Baron, Joker::Mime];
    let is_pareidolia_exists = joker_cards
        .iter()
        .any(|card| card.joker == Joker::Pareidolia);
    let is_smeared_exists = joker_cards
        .iter()
        .any(|card| card.joker == Joker::SmearedJoker);

    joker_cards
        .iter()
        .filter(|card| on_scored_jokers.contains(&card.joker))
        .for_each(|card| {
            new_result = apply_medium_jokers(
                card.joker,
                on_held_cards,
                on_scored_cards,
                new_result.0,
                new_result.1,
                is_pareidolia_exists,
                is_smeared_exists,
            );
        });

    joker_cards
        .iter()
        .filter(|card| on_held_jokers.contains(&card.joker))
        .for_each(|card| {
            new_result = apply_medium_jokers(
                card.joker,
                on_held_cards,
                on_scored_cards,
                new_result.0,
                new_result.1,
                is_pareidolia_exists,
                is_smeared_exists,
            );
        });

    joker_cards
        .iter()
        .filter(|card| independent_jokers.contains(&card.joker))
        .for_each(|card| {
            new_result = apply_easy_jokers(
                card.joker,
                hand,
                joker_cards.len(),
                new_result.0,
                new_result.1,
            );
            new_result = apply_medium_jokers(
                card.joker,
                on_held_cards,
                on_scored_cards,
                new_result.0,
                new_result.1,
                is_pareidolia_exists,
                is_smeared_exists,
            );
        });

    // Apply edition
    joker_cards.iter().for_each(|card| {
        new_result = if let Some(edition) = card.edition {
            apply_edition(edition, new_result.0, new_result.1, false)
        } else {
            new_result
        };
    });

    (new_result.0, new_result.1)
}
