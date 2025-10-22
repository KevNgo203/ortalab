use ortalib::{Chips, Joker, JokerCard, Mult, PokerHand};
use crate::poker::apply_edition;

// ------------------------------- Easy Joker -------------------------------



fn apply_easy_jokers(joker: Joker, hand: PokerHand, joker_cards_len: usize, chip: f64, mul: f64) -> (Chips, Mult) {
  let mut res = (chip, mul);

  // Base Joker
  if joker == Joker::Joker {
    res.1 += 4.0;
  } 

  // Jolly and Sly Joker
  let mut check = vec![PokerHand::Pair, PokerHand::TwoPair, PokerHand::FullHouse, PokerHand::ThreeOfAKind, PokerHand::FourOfAKind, PokerHand::FiveOfAKind, PokerHand::FlushHouse, PokerHand::FlushFive];
  if check.contains(&hand) {
    if joker == Joker::JollyJoker {
      res.1 += 8.0;
    } else if joker == Joker::SlyJoker {
      res.0 += 50.0;
    } 
  }

  // Zany and Willy Joker
  check = vec![PokerHand::ThreeOfAKind, PokerHand::FullHouse, PokerHand::FlushHouse, PokerHand::FourOfAKind, PokerHand::FiveOfAKind, PokerHand::FlushFive];
  if check.contains(&hand) {
    if joker == Joker::ZanyJoker {
      res.1 += 12.0;
    } else if joker == Joker::WilyJoker {
      res.0 += 100.0;
    } 
  }

  // Mad and Clever Joker
  check = vec![PokerHand::TwoPair, PokerHand::FullHouse, PokerHand::FlushHouse];
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
  check = vec![PokerHand::Flush, PokerHand::FlushFive, PokerHand::FlushHouse, PokerHand::FourOfAKind];
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
  }

  (res.0, res.1)
}

// ------------------------------- Medium Joker -------------------------------


// ------------------------------- Hard Joker -------------------------------


pub fn joker_application(cards: Vec<JokerCard>, hand: PokerHand, chip: f64, mul: f64) -> (Chips, Mult) {
  let mut new_result = (chip, mul);

  cards.iter().for_each(|card| {
    new_result = apply_easy_jokers(card.joker, hand, cards.len(), new_result.0, new_result.1);
    new_result = if let Some(edition) = card.edition {
      apply_edition(edition, new_result.0, new_result.1, false)
    } else {
      new_result
    };
  });
  
  (new_result.0, new_result.1)
}