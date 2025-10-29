use crate::poker::determine_poker_hand;
use crate::poker::jokers::joker_application;
use crate::poker::modifiers::compute_enhancement;
use ortalib::{Chips, Joker, Mult, Round};

/// Computes the final chip score and multiplier for a given poker round.
///
/// This function:
/// - Determines the best poker hand from the cards played and jokers.
/// - Adjusts the base score with rank values of the scored cards.
/// - Applies enhancements from modifiers and held cards.
/// - Applies joker effects to further modify the result.
///
/// # Arguments
/// * `round` - A [`Round`] containing the cards played, jokers, and cards held in hand.
///
/// # Returns
/// A tuple `(Chips, Mult)` where:
/// - `Chips` is the final chip value after all scoring rules.
/// - `Mult` is the multiplier applied to the chip value.
///
/// # Example
/// ```
/// use ortalib::{Round, Chips, Mult, Suit, Rank, Card};
/// use ortalab::poker::scoring::score;
///
/// // Construct a dummy round (details depend on your Round struct)
/// let round = Round {
///     cards_played: vec![
///         Card::new(Rank::Ace, Suit::Hearts, None, None),     // A♥
///         Card::new(Rank::King, Suit::Spades, None, None),    // K♠
///         Card::new(Rank::Eight, Suit::Diamonds, None, None), // 8♦
///         Card::new(Rank::Six, Suit::Clubs, None, None),      // 6♣
///         Card::new(Rank::Four, Suit::Hearts, None, None),    // 4
///     ],      
///     cards_held_in_hand: vec![],
///     jokers: vec![],
/// };
///
/// let (chips, mult): (Chips, Mult) = score(round);
/// assert!(chips >= 1.0);
/// assert!(mult >= 1.0);
/// ```
pub fn score(round: Round) -> (Chips, Mult) {
    let mut result;
    let (hand, return_card) = determine_poker_hand(&round.cards_played, &round.jokers);
    result = hand.hand_value();

    let is_splash_joker_exists = round.jokers.iter().any(|card| card.joker == Joker::Splash);
    let on_scored_cards = if !is_splash_joker_exists {
        &return_card
    } else {
        &round.cards_played
    };

    result.0 = on_scored_cards
        .iter()
        .fold(result.0, |acc, x| acc + x.rank.rank_value());
    result = compute_enhancement(on_scored_cards, result.0, result.1, false);
    result = compute_enhancement(&round.cards_held_in_hand, result.0, result.1, true);
    result = joker_application(
        &round.jokers,
        &round.cards_held_in_hand,
        on_scored_cards,
        hand,
        result.0,
        result.1,
    );

    (result.0, result.1)
}
