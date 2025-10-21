use ortalib::{Chips, Mult, Round};
use crate::poker::determine_poker_hand;
use crate::poker::hands::compute_card_order;
use ordered_float::OrderedFloat;
use itertools::Itertools;

pub fn score(round: Round) -> (Chips, Mult) {
  let sorted_cards_played: Vec<_> = round.cards_played
    .iter()
    .sorted_by_key(|&card| OrderedFloat(compute_card_order(*card)))
    .map(|&card| card)
    .collect();

  // println!("{:?}", sorted_cards_played);
  let (hand, return_card) = determine_poker_hand(sorted_cards_played);

  // println!("{:?}", return_card);


  let (base_chip, base_mul) = hand.hand_value();
  let compute_chip = return_card.iter().fold(base_chip, |acc, x| acc + x.rank.rank_value());

  (compute_chip, base_mul)
    // (1.0, 1.0)
}

