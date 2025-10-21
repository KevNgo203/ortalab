use ortalib::{Chips, Mult, Round};
use crate::poker::determine_poker_hand;
use crate::poker::modifiers::compute_enhancement;

pub fn score(round: Round) -> (Chips, Mult) {
  // ------------------ Stage 01 ------------------
  // println!("{:?}", sorted_cards_played);
  let (hand, return_card) = determine_poker_hand(round.cards_played);
  // println!("return: {:?}", return_card);
  // println!("in hand: {:?}", round.cards_held_in_hand);
  let (base_chip, base_mul) = hand.hand_value();
  // println!("{} {}", base_chip, base_mul);
  let compute_chip = return_card.iter().fold(base_chip, |acc, x| acc + x.rank.rank_value());
 
 
  // ------------------ Stage 02 ------------------ 
  // Apply modifiers for cards played
  let (enhanced_chip, enhanced_mul) = compute_enhancement(return_card, compute_chip, base_mul, false);
  // println!("{} {}", enhanced_chip, enhanced_mul);

  // Apply modifiers for cards held in hand
  let (chip, mul) = compute_enhancement(round.cards_held_in_hand, enhanced_chip, enhanced_mul, true);
  // println!("{} {}", chip, mul);


  (chip, mul)
    // (1.0, 1.0)
}

