use crate::poker::determine_poker_hand;
use crate::poker::jokers::joker_application;
use crate::poker::modifiers::compute_enhancement;
use ortalib::{Chips, Joker, Mult, Round};

pub fn score(round: Round) -> (Chips, Mult) {
    let mut result;
    let (hand, return_card) = determine_poker_hand(&round.cards_played, &round.jokers);
    // println!("return: {:?}", return_card);
    result = hand.hand_value();
    
    let on_scored_cards;
    let is_splash_joker_exists = round.jokers.iter().any(|card| card.joker == Joker::Splash);
    if !is_splash_joker_exists {
        on_scored_cards = &return_card;
    } else {
        on_scored_cards = &round.cards_played;
    }

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
