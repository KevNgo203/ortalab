use crate::poker::determine_poker_hand;
use crate::poker::jokers::joker_application;
use crate::poker::modifiers::compute_enhancement;
use ortalib::{Chips, Mult, Round};

pub fn score(round: Round) -> (Chips, Mult) {
    // ------------------ Stage 01 ------------------
    // println!("{:?}", sorted_cards_played);
    let (hand, return_card) = determine_poker_hand(round.cards_played);
    // println!("return: {:?}", return_card);
    // println!("in hand: {:?}", round.cards_held_in_hand);
    let mut result = hand.hand_value();
    result.0 = return_card
        .iter()
        .fold(result.0, |acc, x| acc + x.rank.rank_value());
    // println!("{} {}", result.0, result.1);

    // ------------------ Stage 02 ------------------
    // Apply modifiers for cards played
    result = compute_enhancement(&return_card, result.0, result.1, false);
    // println!("{} {}", enhanced_chip, enhanced_mul);

    // Apply modifiers for cards held in hand
    result = compute_enhancement(&round.cards_held_in_hand, result.0, result.1, true);
    // println!("{} {}", chip, mul);

    // ------------------ Stage 03 ------------------
    // Apply jokers effect for joker cards
    // println!("{:?}", round.jokers);
    result = joker_application(
        round.jokers,
        &round.cards_held_in_hand,
        &return_card,
        hand,
        result.0,
        result.1,
    );

    (result.0, result.1)
    // (1.0, 1.0)
}
