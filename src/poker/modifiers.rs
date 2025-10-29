//! # Poker Modifiers
//!
//! This module defines how card **enhancements** and **editions** affect
//! chip values and multipliers during scoring. Enhancements and editions
//! are applied to cards either in play or in hand, and modify the scoring
//! pipeline accordingly.
//!
//! - [`apply_enhancement`] — applies a single [`Enhancement`] to chip/mult values.
//! - [`apply_edition`] — applies a single [`Edition`] to chip/mult values.
//! - [`compute_enhancement`] — applies all enhancements and editions across a set of cards.

use ortalib::{Card, Chips, Edition, Enhancement, Mult};

/// Applies a single [`Enhancement`] to the given chip and multiplier values.
///
/// If the card is in play (`in_hand == false`), most enhancements apply directly.
/// If the card is in hand, only [`Enhancement::Steel`] has an effect.
///
/// # Arguments
/// * `enhancement` — The enhancement to apply.
/// * `input_chip` — Current chip value.
/// * `input_mul` — Current multiplier value.
/// * `in_hand` — Whether the card is still in hand.
///
/// # Returns
/// A tuple `(Chips, Mult)` with updated values.
///
/// # Example
/// ```
/// use ortalib::{Enhancement};
/// use ortalab::poker::modifiers::apply_enhancement;
///
/// let (chips, mult) = apply_enhancement(Enhancement::Bonus, 100.0, 1.0, false);
/// assert_eq!(chips, 130.0);
/// assert_eq!(mult, 1.0);
/// ```
pub fn apply_enhancement(
    enhancement: Enhancement,
    input_chip: f64,
    input_mul: f64,
    in_hand: bool,
) -> (Chips, Mult) {
    let (mut chip, mut mul) = (input_chip, input_mul);

    if !in_hand {
        match enhancement {
            Enhancement::Bonus => chip += 30.0,
            Enhancement::Mult => mul += 4.0,
            Enhancement::Wild => {}
            Enhancement::Glass => mul *= 2.0,
            Enhancement::Steel => {}
        }
    } else if enhancement == Enhancement::Steel {
        mul *= 1.5;
    }

    (chip, mul)
}

/// Applies a single [`Edition`] to the given chip and multiplier values.
///
/// Editions only apply when the card is in play (`in_hand == false`).
///
/// # Arguments
/// * `edition` — The edition to apply.
/// * `input_chip` — Current chip value.
/// * `input_mul` — Current multiplier value.
/// * `in_hand` — Whether the card is still in hand.
///
/// # Returns
/// A tuple `(Chips, Mult)` with updated values.
///
/// # Example
/// ```
/// use ortalib::{Edition};
/// use ortalab::poker::modifiers::apply_edition;
///
/// let (chips, mult) = apply_edition(Edition::Foil, 100.0, 1.0, false);
/// assert_eq!(chips, 150.0);
/// assert_eq!(mult, 1.0);
/// ```
pub fn apply_edition(
    edition: Edition,
    input_chip: f64,
    input_mul: f64,
    in_hand: bool,
) -> (Chips, Mult) {
    let (mut chip, mut mul) = (input_chip, input_mul);

    if !in_hand {
        match edition {
            Edition::Foil => chip += 50.0,
            Edition::Holographic => mul += 10.0,
            Edition::Polychrome => mul *= 1.5,
        }
    }
    (chip, mul)
}

/// Applies all enhancements and editions across a slice of cards.
///
/// Each card may have an [`Enhancement`] and/or an [`Edition`]. Both are
/// applied in sequence to update the chip and multiplier values.
///
/// # Arguments
/// * `cards` — The cards to process.
/// * `chip` — Starting chip value.
/// * `mul` — Starting multiplier value.
/// * `in_hand` — Whether the cards are in hand or in play.
///
/// # Returns
/// A tuple `(Chips, Mult)` with updated values.
///
/// # Example
/// ```
/// use ortalib::{Card, Rank, Suit, Enhancement};
/// use ortalab::poker::modifiers::compute_enhancement;
///
/// let mut card = Card::new(Rank::Ace, Suit::Spades, None, None);
/// card.enhancement = Some(Enhancement::Bonus);
///
/// let (chips, mult) = compute_enhancement(&[card], 100.0, 1.0, false);
/// assert_eq!(chips, 130.0);
/// assert_eq!(mult, 1.0);
/// ```
pub fn compute_enhancement(cards: &[Card], chip: f64, mul: f64, in_hand: bool) -> (Chips, Mult) {
    let (mut new_chip, mut new_mul) = (chip, mul);

    cards.iter().for_each(|card| {
        if let Some(enhancement) = card.enhancement {
            let result = apply_enhancement(enhancement, new_chip, new_mul, in_hand);
            new_chip = result.0;
            new_mul = result.1;
        }

        if let Some(edition) = card.edition {
            let result = apply_edition(edition, new_chip, new_mul, in_hand);
            new_chip = result.0;
            new_mul = result.1;
        }
    });

    (new_chip, new_mul)
}
