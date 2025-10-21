use std::ptr;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use ortalib::{Card, PokerHand, Rank};

// Poker Hand: High Card
// When no other poker hand is possible, the one highest card in your played hand. 
// Base scoring: 5 chips x 1 mult
fn is_high_card(cards: &Vec<Card>) -> Vec<Card> {
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
fn is_pair(cards: &Vec<Card>) -> Vec<Card> {
    let mut card_to_return: Vec<Card> = Vec::new();

     for (curr, next) in cards.iter().tuple_windows() {
        if compute_card_order(*curr)  == compute_card_order(*next) {
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
fn is_two_pair(cards: &Vec<Card>) -> Vec<Card> {
    let mut card_to_return: Vec<Card> = Vec::new();
    let mut prev_rank = 0.0;

     for (curr, next) in cards.iter().tuple_windows() {
        let curr_order = compute_card_order(*curr);
        let next_order = compute_card_order(*next);
        if prev_rank != 0.0 {
            if curr_order  == next_order
            &&  curr_order != prev_rank { 
                card_to_return.push(*curr);
                card_to_return.push(*next);
            }
        } else {
            if curr_order == next_order {
                 card_to_return.push(*curr);
                 card_to_return.push(*next);
                 prev_rank = curr_order;
            }  
        }

    }

    card_to_return
}


// Poker Hand: Three of a Kind
// Three cards with a matching rank. Suits may differ.
// Base scoring: 30 chips x 3 mult
fn is_three_of_a_kind(cards: &Vec<Card>) -> Vec<Card> {
    let mut card_to_return: Vec<Card> = Vec::new();
    let mut prev_rank = 0.0;

     for (curr, next) in cards.iter().tuple_windows() {
        let curr_order = compute_card_order(*curr);
        let next_order = compute_card_order(*next);
        if curr_order == next_order {
            prev_rank = curr_order;
            card_to_return.push(*curr);
            if let Some(last) = cards.last() {
                if ptr::eq(next, last) {
                    card_to_return.push(*next);
                }
            }
        } else {
            if curr_order == prev_rank {
                card_to_return.push(*curr);
            }
        }
    }
    card_to_return
}

// Poker Hand: Straight
// Five cards in consecutive order which are not all from the same suit. Aces can be counted high or low.
// Base scoring: 30 chips x 4 mult
fn is_straight(cards: &Vec<Card>) -> Vec<Card> {
    let mut card_to_return: Vec<Card> = Vec::new();

    // Check for consecutive cards with values from 2 - A
    for (curr, next) in cards.iter().tuple_windows() {
        let curr_order = compute_card_order(*curr);
        let next_order = compute_card_order(*next);
        if next_order - curr_order == 1.0 {
          card_to_return.push(*curr);

          if let Some(last) = cards.last() {
            if ptr::eq(next, last) {
                card_to_return.push(*next);
            }
          }
        } 
        // Handle case where Ace is the lowest value card (below 2)
        else if next_order - curr_order == 9.0 {
            card_to_return.push(*curr);
            card_to_return.push(*next);
        }
    }
    card_to_return
}

// Poker Hand: Flush
// Five cards of any rank, all from a single suit.
// Base scoring: 35 chips x 4 mult
fn is_flush(cards: &Vec<Card>) -> Vec<Card> {
    let mut card_to_return: Vec<Card> = Vec::new();
    let base_suit = cards.first().unwrap().suit;

    cards.iter().for_each(|card| {
        if card.suit == base_suit {
            card_to_return.push(*card);
        }
    });
    card_to_return
}

// Poker Hand: Full House
// Three cards with a matching rank, and two cards with any other matching rank, 
// with cards from two or more suits.
// Base scoring: 40 chips x 4 mult
fn is_full_house(cards: &Vec<Card>) -> Vec<Card> {
    let mut card_to_return: Vec<Card> = Vec::new();
    let mut prev_rank = 0.0;

     for (curr, next) in cards.iter().tuple_windows() {
        let curr_order = compute_card_order(*curr);
        let next_order = compute_card_order(*next);

        if prev_rank != 0.0 {
            if curr_order == next_order && curr_order != prev_rank {
                card_to_return.push(*curr);
                if let Some(last) = cards.last() {
                    if ptr::eq(next, last) {
                        card_to_return.push(*next);
                    }
                }
                continue;
            }
        } 
        if curr_order == next_order {
            prev_rank = curr_order;
            card_to_return.push(*curr);
            if let Some(last) = cards.last() {
                if ptr::eq(next, last) {
                    card_to_return.push(*next);
                }
            }
        } else {
            if curr_order == prev_rank {
                card_to_return.push(*curr);
            }
        }
        
    }
    card_to_return
}

// Poker Hand: Four of a Kind
// Four cards with a matching rank. Suits may differ.
// Base scoring: 60 chips x 4 mult
fn is_four_of_a_kind(cards: &Vec<Card>) -> Vec<Card> {
    let mut card_to_return: Vec<Card> = Vec::new();
    let mut prev_rank = 0.0;

     for (curr, next) in cards.iter().tuple_windows() {
        let curr_order = compute_card_order(*curr);
        let next_order = compute_card_order(*next);

        if prev_rank != 0.0 {
            if curr_order == next_order && curr_order == prev_rank {
                card_to_return.push(*curr);
                if let Some(last) = cards.last() {
                    if ptr::eq(next, last) {
                        card_to_return.push(*next);
                    }
                }
            } else if curr_order != next_order {
                if curr_order == prev_rank {
                    card_to_return.push(*curr);
                }
            }
        } else {
            if curr_order == next_order {
                prev_rank = curr_order;
                card_to_return.push(*curr);
            } 
        }
    }
    card_to_return
}

// Poker Hand: Straight Flush
// Five cards in consecutive order, all from the same suit. Aces can be counted high or low.
// Base scoring: 100 chips x 8 mult
fn is_straight_flush(cards: &Vec<Card>) -> Vec<Card> {
    let card_to_return = cards.clone();
    if is_flush(cards).len() == 5 && is_straight(cards).len() == 5 {
        return card_to_return; 
    } 

    vec![]
}

// Poker Hand: Five of a Kind
// Five cards with the same rank which are not all the same suit.
// (an "illegal" hand)
// Base scoring: 120 chips x 12 mult
fn is_five_of_a_kind(cards: &Vec<Card>) -> Vec<Card> {
    let mut card_to_return: Vec<Card> = Vec::new();
     for (curr, next) in cards.iter().tuple_windows() {
        let curr_order = compute_card_order(*curr);
        let next_order = compute_card_order(*next);
        if curr_order == next_order {
            card_to_return.push(*curr);
            if let Some(last) = cards.last() {
                if ptr::eq(next, last) {
                    card_to_return.push(*next);
                }
            }
        } 
    }
    card_to_return
}

// Poker Hand: Flush House
// Three cards with a matching rank, and two cards with any other matching rank, all from a single suit.
// (an "illegal" hand)
// Base scoring: 140 chips x 14 mult
fn is_flush_house(cards: &Vec<Card>) -> Vec<Card> {
    let mut card_to_return: Vec<Card> = Vec::new();
    let base_suit = cards.first().unwrap().suit;
    let mut prev_rank = 0.0;

     for (curr, next) in cards.iter().tuple_windows() {
        let curr_order = compute_card_order(*curr);
        let next_order = compute_card_order(*next);

        if prev_rank != 0.0 {
            if curr_order == next_order && curr.suit == base_suit && curr_order != prev_rank {
                card_to_return.push(*curr);
                if let Some(last) = cards.last() {
                    if ptr::eq(next, last) {
                        card_to_return.push(*next);
                    }
                }
                continue;
            }
        } 
        if curr_order == next_order && curr.suit == base_suit {
            prev_rank = curr_order;
            card_to_return.push(*curr);
            if let Some(last) = cards.last() {
                if ptr::eq(next, last) {
                    card_to_return.push(*next);
                }
            }
        } else {
            if curr_order == prev_rank && curr.suit == base_suit {
                card_to_return.push(*curr);
            }
        }
        
    }
    card_to_return
}

// Poker Hand: Flush Five
// Five cards with the same rank and same suit.
// (an "illegal" hand)
// Base scoring: 160 chips x 16 mult
fn is_flush_five(cards: &Vec<Card>) -> Vec<Card> {
    let mut card_to_return: Vec<Card> = Vec::new();
    let base_suit = cards.first().unwrap().suit;

     for (curr, next) in cards.iter().tuple_windows() {
        let curr_order = compute_card_order(*curr);
        let next_order = compute_card_order(*next);
        if curr_order == next_order && curr.suit == base_suit {
            card_to_return.push(*curr);
            if let Some(last) = cards.last() {
                if ptr::eq(next, last) {
                    card_to_return.push(*next);
                }
            }
        } 
    }
    card_to_return
}


pub fn compute_card_order(card: Card) -> f64 {
    let return_value = match card.rank {
        Rank::Two => 2.0,
        Rank::Three => 3.0,
        Rank::Four => 4.0,
        Rank::Five => 5.0,
        Rank::Six => 6.0,
        Rank::Seven => 7.0,
        Rank::Eight => 8.0,
        Rank::Nine => 9.0,
        Rank::Ten => 10.0,
        Rank::Jack => 11.0,
        Rank::Queen => 12.0,
        Rank::King => 13.0,
        Rank::Ace => 14.0,
    };

    return_value   
}


pub fn determine_poker_hand(cards: Vec<Card>) -> (PokerHand, Vec<Card>) {
    let mut return_card;

    let sorted_cards_played: Vec<_> = cards
    .iter()
    .sorted_by_key(|&card| OrderedFloat(compute_card_order(*card)))
    .map(|&card| card)
    .collect();

    // Check if a flush five  exists
    return_card = is_flush_five(&sorted_cards_played);
    if return_card.len() == 5 {
        // println!("IS FLUSH FIVE");
        return (PokerHand::FlushFive, cards);
    }

    // Check if a flush house  exists
    return_card = is_flush_house(&sorted_cards_played);
    if return_card.len() == 5 {
        // println!("IS FLUSH HOUSE");
        return (PokerHand::FlushHouse, cards);
    }

    // Check if a five of a kind  exists
    return_card = is_five_of_a_kind(&sorted_cards_played);
    if return_card.len() == 5 {
        // println!("IS FIVE OF A KIND");
        return (PokerHand::FiveOfAKind, cards);
    }

    // Check if a straight flush exists
    return_card = is_straight_flush(&sorted_cards_played);
    if return_card.len() == 5 {
        // println!("IS STRAIGHT FLUSH");
        return (PokerHand::StraightFlush, cards);
    }

    // Check if a four of a kind  exists
    return_card = is_four_of_a_kind(&sorted_cards_played);
    if return_card.len() == 4 {
        // println!("IS FOUR OF A KIND");
        return (PokerHand::FourOfAKind, return_card);
    }

    // Check if a full house exists
    return_card = is_full_house(&sorted_cards_played);
    if return_card.len() == 5 {
        // println!("IS FULL HOUSE");
        return (PokerHand::FullHouse, cards);
    }

    // Check if a flush exists
    return_card = is_flush(&sorted_cards_played);
    if return_card.len() == 5 {
        // println!("IS FLUSH");
        return (PokerHand::Flush, cards);
    }
    
    // Check if a straight exists
    return_card = is_straight(&sorted_cards_played);
    if return_card.len() == 5 {
        // println!("IS STRAIGHT");
        return (PokerHand::Straight, cards);
    }
    
    // Check if a three of a kind exists
    return_card = is_three_of_a_kind(&sorted_cards_played);
    if return_card.len() == 3 {
        // println!("IS THREE OF A KIND");
        return (PokerHand::ThreeOfAKind, return_card);
    }

    // Check if a pair exists
    return_card = is_two_pair(&sorted_cards_played);
    if return_card.len() == 4 {
        // println!("IS TWO PAIR");
        return (PokerHand::TwoPair, return_card);
    }
    
    // Check if a pair exists
    return_card = is_pair(&sorted_cards_played);
    if return_card.len() == 2 {
        // println!("IS PAIR");
        return (PokerHand::Pair, return_card);
    }

    // Default/base case when no other poker hands exist
    // println!("IS HIGH CARD");
    (PokerHand::HighCard, is_high_card(&sorted_cards_played))
}