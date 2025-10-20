use std::collections::HashSet;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use ortalib::{Card, PokerHand};

// Poker Hand: High Card
// When no other poker hand is possible, the one highest card in your played hand. 
// Base scoring: 5 chips x 1 mult
pub fn is_high_card(cards: &Vec<Card>) -> Vec<Card> {
    let card_to_return = cards
        .iter()
        .max_by_key(|&card| OrderedFloat(card.rank.rank_value()))
        .copied()
        .unwrap();
    let vec_to_return = vec![card_to_return];
    return vec_to_return;
}


// Poker Hand: Pair
// Two cards with a matching rank. Suits may differ.
// Base scoring: 10 chips x 2 mult
pub fn is_pair(cards: &Vec<Card>) -> Vec<Card> {
    let mut card_to_return: Vec<Card> = Vec::new();

     for (curr, next) in cards.iter().tuple_windows() {
        if curr.rank.rank_value() == next.rank.rank_value() {
            card_to_return.push(*curr);
            card_to_return.push(*next);
            break;
        }
    }

    card_to_return
}

// Poker Hand: Two Pair
// Two cards with a matching rank, and two cards with any other matching rank. Suits may differ.
// Base scoring: 20 chips x 2 mult
pub fn is_two_pair(cards: &Vec<Card>) -> Vec<Card> {
    let mut card_to_return: Vec<Card> = Vec::new();

     for (curr, next) in cards.iter().tuple_windows() {
        if curr.rank.rank_value() == next.rank.rank_value() {
            card_to_return.push(*curr);
            card_to_return.push(*next);
        }
    }

    card_to_return
}


// Poker Hand: Three of a Kind
// Three cards with a matching rank. Suits may differ.
// Base scoring: 30 chips x 3 mult
pub fn is_three_of_a_kind(cards: &Vec<Card>) -> Vec<Card> {
    let mut card_to_return = HashSet::new();

     for (curr, next) in cards.iter().tuple_windows() {
        if curr.rank.rank_value() == next.rank.rank_value() {
            card_to_return.insert(*curr);
            card_to_return.insert(*next);
        }
    }

    card_to_return.iter().map(|&card| card).collect::<Vec<Card>>()
}

// Poker Hand: Straight
// Five cards in consecutive order which are not all from the same suit. Aces can be counted high or low.
// Base scoring: 30 chips x 4 mult
pub fn is_straight(cards: &Vec<Card>) -> Vec<Card> {
    let mut card_to_return: Vec<Card> = Vec::new();



     for (curr, next) in cards.iter().tuple_windows() {

      // Check for consecutive cards with values from 2 - 9
      if next.rank.rank_value() - curr.rank.rank_value() == 1.0 {
          card_to_return.push(*curr);
      } 
      // Check for consecutive cards with values from 10 - K
      else if next.rank.rank_value() - curr.rank.rank_value() == 0.0 {
        if curr.rank.is_face() {

        } else {
          card_to_return.push(*curr);
        }
      }
    }

    card_to_return
}


pub fn determine_poker_hand(cards: Vec<Card>) -> (PokerHand, Vec<Card>) {
    let mut return_card;
    
    
    
    // Check if a three of a kind exists
    return_card = is_three_of_a_kind(&cards);
    if return_card.len() == 3 {
        return (PokerHand::ThreeOfAKind, return_card);
    }

    // Check if a pair exists
    return_card = is_two_pair(&cards);
    if return_card.len() == 4 {
        return (PokerHand::TwoPair, return_card);
    }
    
    // Check if a pair exists
    return_card = is_pair(&cards);
    if return_card.len() == 2 {
        return (PokerHand::Pair, return_card);
    }

    // Default/base case when no other poker hands exist
    (PokerHand::HighCard, is_high_card(&cards))
}