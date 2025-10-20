use ortalib::{Chips, Mult, Round, Card};
use crate::poker::determine_poker_hand;
use ordered_float::OrderedFloat;
use itertools::Itertools;

pub fn score(round: Round) -> (Chips, Mult) {
  let sorted_cards_played: Vec<_> = round.cards_played
    .iter()
    .sorted_by_key(|&card| OrderedFloat(card.rank.rank_value()))
    .map(|&card| card)
    .collect();

  println!("{:?}", sorted_cards_played);
  let (hand, return_card) = determine_poker_hand(sorted_cards_played);



  let (base_chip, base_mul) = hand.hand_value();
  let compute_chip = return_card.iter().fold(base_chip, |acc, x| acc + x.rank.rank_value());

  (compute_chip, base_mul)
    // (1.0, 1.0)
}