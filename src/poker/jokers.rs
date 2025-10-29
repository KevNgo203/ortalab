use crate::poker::{apply_edition, compute_card_order, determine_current_suit, determine_total_colors};
use ordered_float::OrderedFloat;
use ortalib::{Card, Chips, Enhancement, Joker, JokerCard, Mult, PokerHand, Rank, Suit, SuitColor};

// ------------------------------- Easy Joker -------------------------------
fn apply_easy_jokers(
    joker: Joker,
    hand: PokerHand,
    joker_cards_len: usize,
    chip: f64,
    mul: f64,
) -> (Chips, Mult) {
    let mut res = (chip, mul);

    // Base Joker
    if joker == Joker::Joker {
        res.1 += 4.0;
        // dbg!(res);
    }

    // Jolly and Sly Joker
    let mut check = vec![
        PokerHand::Pair,
        PokerHand::TwoPair,
        PokerHand::FullHouse,
        PokerHand::ThreeOfAKind,
        PokerHand::FourOfAKind,
        PokerHand::FiveOfAKind,
        PokerHand::FlushHouse,
        PokerHand::FlushFive,
    ];
    if check.contains(&hand) {
        if joker == Joker::JollyJoker {
            res.1 += 8.0;
            // dbg!(res);
        } else if joker == Joker::SlyJoker {
            res.0 += 50.0;
        }
    }

    // Zany and Willy Joker
    check = vec![
        PokerHand::ThreeOfAKind,
        PokerHand::FullHouse,
        PokerHand::FlushHouse,
        PokerHand::FourOfAKind,
        PokerHand::FiveOfAKind,
        PokerHand::FlushFive,
    ];
    if check.contains(&hand) {
        if joker == Joker::ZanyJoker {
            res.1 += 12.0;
            // dbg!(res);
        } else if joker == Joker::WilyJoker {
            res.0 += 100.0;
        }
    }

    // Mad and Clever Joker
    check = vec![
        PokerHand::TwoPair,
        PokerHand::FullHouse,
        PokerHand::FlushHouse,
    ];
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
    check = vec![
        PokerHand::Flush,
        PokerHand::FlushFive,
        PokerHand::FlushHouse,
        PokerHand::FourOfAKind,
    ];
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
        // dbg!(res);
    }

    (res.0, res.1)
}

// ------------------------------- Medium Joker -------------------------------
fn apply_medium_jokers(
    joker: Joker,
    on_held: &[Card],
    on_scored: &[Card],
    chip: f64,
    mul: f64,
    is_pareidolia_exists: bool,
    is_smeared_exists: bool
) -> (Chips, Mult) {
    let mut res = (chip, mul);
    let mut on_held_iter = on_held.iter();

    if joker == Joker::RaisedFist {
        let lowest_rank_card = on_held
            .iter()
            .min_by_key(|&card| OrderedFloat(card.rank.rank_value()))
            .unwrap();
        let mut vec_lowest_rank_card = on_held
            .iter()
            .filter(|&card| card.rank.rank_value() == lowest_rank_card.rank.rank_value());
        res.1 += vec_lowest_rank_card.next_back().unwrap().rank.rank_value() * 2.0;

        // TODO retriggers
    }

    // Blackboard Joker
    if joker == Joker::Blackboard {
        let vec_to_check = [Suit::Spades, Suit::Clubs];
        let check = on_held_iter.all(|c| {
            if let Some(enhance) = c.enhancement
                && enhance == Enhancement::Wild
            {
                return true;
            }
            vec_to_check.contains(&c.suit)
        });

        if check || on_held.is_empty() {
            res.1 *= 3.0;
        }
    }

    // Baron Joker
    if joker == Joker::Baron {
        on_held_iter.for_each(|&card| {
            if card.rank == Rank::King {
                res.1 *= 1.5;
            }
        });
        // dbg!(res);
    }

    // Greedy Joker
    if joker == Joker::GreedyJoker {
        on_scored.iter().for_each(|&card| {
            if card.suit == Suit::Diamonds || (card.suit == Suit::Hearts && is_smeared_exists) {
                res.1 += 3.0;
            } else if let Some(enhance) = card.enhancement
                && enhance == Enhancement::Wild
            {
                res.1 += 3.0;
            }
        })
    }

    // Lusty Joker
    if joker == Joker::LustyJoker {
        on_scored.iter().for_each(|&card| {
            if card.suit == Suit::Hearts || (card.suit == Suit::Diamonds && is_smeared_exists) {
                res.1 += 3.0;
            } else if let Some(enhance) = card.enhancement
                && enhance == Enhancement::Wild
            {
                res.1 += 3.0;
            }
        })
    }

    // Wrathful Joker
    if joker == Joker::WrathfulJoker {
        on_scored.iter().for_each(|&card| {
            if card.suit == Suit::Spades || (card.suit == Suit::Clubs && is_smeared_exists) {
                res.1 += 3.0;
            } else if let Some(enhance) = card.enhancement
                && enhance == Enhancement::Wild
            {
                res.1 += 3.0;
            }
        })
    }

    // Gluttonus Joker
    if joker == Joker::GluttonousJoker {
        on_scored.iter().for_each(|&card| {
            if card.suit == Suit::Clubs || (card.suit == Suit::Spades && is_smeared_exists){
                res.1 += 3.0;
            } else if let Some(enhance) = card.enhancement
                && enhance == Enhancement::Wild
            {
                res.1 += 3.0;
            }
        })
    }

    // Fibonacci Joker
    if joker == Joker::Fibonacci {
        on_scored.iter().for_each(|&card| {
            if card.rank == Rank::Ace
                || card.rank == Rank::Two
                || card.rank == Rank::Three
                || card.rank == Rank::Five
                || card.rank == Rank::Eight
            {
                res.1 += 8.0;
            }
        })
    }

    // Scary Face
    if joker == Joker::ScaryFace {
        on_scored.iter().for_each(|&card| {
            if card.rank.is_face() || is_pareidolia_exists {
                res.0 += 30.0;
            } 
        })
    }

    // Even Steven
    if joker == Joker::EvenSteven {
        on_scored.iter().for_each(|&card| {
            let value = compute_card_order(card);
            if value <= 10.0 && value % 2.0 == 0.0 {
                res.1 += 4.0;
            }
        })
    }

    // Odd Todd
    if joker == Joker::OddTodd {
        on_scored.iter().for_each(|&card| {
            let value = compute_card_order(card);
            if value == 14.0 || (value < 10.0 && value % 2.0 != 0.0) {
                res.0 += 31.0;
            }
        })
    }

    // Photograph
    if joker == Joker::Photograph {
        let mut firt_check = false;
        on_scored.iter().for_each(|&card| {
            if (card.rank.is_face() || is_pareidolia_exists) && !firt_check {
                res.1 *= 2.0;
                firt_check = true;
            }

            // TODO: Handle retriggers
        })
    }

    // Smiley Face
    if joker == Joker::SmileyFace {
        on_scored.iter().for_each(|&card| {
            if card.rank.is_face() || is_pareidolia_exists {
                res.1 += 5.0;
            }
        })
    }

    // Flower pot
    if joker == Joker::FlowerPot && on_scored.len() >= 4 {
        let having_diamonds = determine_current_suit(on_scored, Suit::Diamonds);
        let having_hearts = determine_current_suit(on_scored, Suit::Hearts);
        let having_spades = determine_current_suit(on_scored, Suit::Spades);
        let having_clubs = determine_current_suit(on_scored, Suit::Clubs);
        let having_red_cards = determine_total_colors(on_scored, SuitColor::Red);
        let having_black_cards = determine_total_colors(on_scored, SuitColor::Black);
        
        if (having_diamonds && having_hearts && having_spades && having_clubs) 
        || (is_smeared_exists && having_red_cards && having_black_cards) {
            res.1 *= 3.0
        } 
    }

    (res.0, res.1)
}

// ------------------------------- Hard Joker -------------------------------
// fn apply_hard_jokers(joker: Joker, chip: f64, mul: f64) -> (Chips, Mult) {
//     let mut res = (chip, mul);





//     (res.0, res.1)
// }

pub fn joker_application(
    joker_cards: &[JokerCard],
    on_held_cards: &[Card],
    on_scored_cards: &[Card],
    hand: PokerHand,
    chip: f64,
    mul: f64,
) -> (Chips, Mult) {
    let mut new_result = (chip, mul);
    let independent_jokers = [
        Joker::Joker,
        Joker::JollyJoker,
        Joker::ZanyJoker,
        Joker::MadJoker,
        Joker::CrazyJoker,
        Joker::DrollJoker,
        Joker::SlyJoker,
        Joker::WilyJoker,
        Joker::CleverJoker,
        Joker::DeviousJoker,
        Joker::CraftyJoker,
        Joker::AbstractJoker,
        Joker::Blackboard,
        Joker::FlowerPot,
    ];
    let on_scored_jokers = [
        Joker::GreedyJoker,
        Joker::LustyJoker,
        Joker::WrathfulJoker,
        Joker::GluttonousJoker,
        Joker::Fibonacci,
        Joker::ScaryFace,
        Joker::EvenSteven,
        Joker::OddTodd,
        Joker::Photograph,
        Joker::SmileyFace,
        Joker::SockAndBuskin
    ];
    let on_held_jokers = [Joker::RaisedFist, Joker::Baron, Joker::Mime];
    // let special_jokers = [Joker::FourFingers, Joker::Shortcut, Joker::Pareidolia, Joker::Splash, Joker::SmearedJoker, Joker::Blueprint];
    let is_pareidolia_exists = joker_cards.iter().any(|card| card.joker == Joker::Pareidolia);
    let is_smeared_exists= joker_cards.iter().any(|card| card.joker == Joker::SmearedJoker);

    // joker_cards
    //     .iter()
    //     .filter(|card| special_jokers.contains(&card.joker))
    //     .for_each(|card| {
    //         new_result = apply_hard_jokers(card.joker, new_result.0, new_result.1);
    //     });

    joker_cards
        .iter()
        .filter(|card| on_scored_jokers.contains(&card.joker))
        .for_each(|card| {
            new_result = apply_medium_jokers(
                card.joker,
                on_held_cards,
                on_scored_cards,
                new_result.0,
                new_result.1,
                is_pareidolia_exists,
                is_smeared_exists
            );
            // new_result = apply_hard_jokers(card.joker, new_result.0, new_result.1);
        });

    joker_cards
        .iter()
        .filter(|card| on_held_jokers.contains(&card.joker))
        .for_each(|card| {
            new_result = apply_medium_jokers(
                card.joker,
                on_held_cards,
                on_scored_cards,
                new_result.0,
                new_result.1,
                is_pareidolia_exists,
                is_smeared_exists
            );
            // new_result = apply_hard_jokers(card.joker, new_result.0, new_result.1);
        });

    joker_cards
        .iter()
        .filter(|card| independent_jokers.contains(&card.joker))
        .for_each(|card| {
            new_result = apply_easy_jokers(
                card.joker,
                hand,
                joker_cards.len(),
                new_result.0,
                new_result.1,
            );
            new_result = apply_medium_jokers(
                card.joker,
                on_held_cards,
                on_scored_cards,
                new_result.0,
                new_result.1,
                is_pareidolia_exists,
                is_smeared_exists
            );
        });

    // Apply edition
    joker_cards.iter().for_each(|card| {
        new_result = if let Some(edition) = card.edition {
            apply_edition(edition, new_result.0, new_result.1, false)
        } else {
            new_result
        };
    });

    // cards.iter().for_each(|card| {
    //   new_result = apply_medium_jokers(card.joker, on_held_cards, on_scored_cards, new_result.0, new_result.1);
    //   new_result = if let Some(edition) = card.edition {
    //     apply_edition(edition, new_result.0, new_result.1, false)
    //   } else {
    //     new_result
    //   };
    // });

    (new_result.0, new_result.1)
}
