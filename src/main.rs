use std::{
    error::Error,
    fs::File,
    io::{Read, stdin},
    path::{Path, PathBuf},
};
use std::collections::HashSet;

use clap::Parser;
use ortalib::{Card, Chips, Mult, PokerHand, Round};
use itertools::Itertools;
use ordered_float::OrderedFloat; 

#[derive(Parser)]
struct Opts {
    file: PathBuf,

    #[arg(long)]
    explain: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    let round = parse_round(&opts)?;

    let (chips, mult) = score(round);

    println!("{}", (chips * mult).floor());
    Ok(())
}

fn parse_round(opts: &Opts) -> Result<Round, Box<dyn Error>> {
    let mut input = String::new();
    if opts.file == Path::new("-") {
        stdin().read_to_string(&mut input)?;
    } else {
        File::open(&opts.file)?.read_to_string(&mut input)?;
    }

    let round = serde_yaml::from_str(&input)?;
    Ok(round)
}

fn score(round: Round) -> (Chips, Mult) {
    // println!("{:?}", round.cards_played);

    let result = determine_poker_hand(round.cards_played);

    let (base_chip, base_mul) = result.0.hand_value();
    let return_card = result.1;

    let compute_chip = return_card.iter().fold(base_chip, |acc, x| acc + x.rank.rank_value());

    println!("{} {}", base_chip, base_mul);
    println!("{:?}", return_card);

    (compute_chip, base_mul)
}

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
        if curr.rank.rank_value() == next.rank.rank_value() {
            card_to_return.push(*curr);
            card_to_return.push(*next);
            break;
        }
    }

    card_to_return
}

// Poker Hand: Three of a Kind
// Three cards with a matching rank. Suits may differ.
// Base scoring: 30 chips x 3 mult
fn is_three_of_a_kind(cards: &Vec<Card>) -> Vec<Card> {
    let mut card_to_return = HashSet::new();

     for (curr, next) in cards.iter().tuple_windows() {
        if curr.rank.rank_value() == next.rank.rank_value() {
            card_to_return.insert(*curr);
            card_to_return.insert(*next);
        }
    }

    card_to_return.iter().map(|&card| card).collect::<Vec<Card>>()
}


fn determine_poker_hand(cards: Vec<Card>) -> (PokerHand, Vec<Card>) {
    let mut return_card;
    
    
    
    // Check if a three of a kind exists
    return_card = is_three_of_a_kind(&cards);
    if return_card.len() == 3 {
        return (PokerHand::ThreeOfAKind, return_card);
    }
    
    // Check if a pair exists
    return_card = is_pair(&cards);
    if return_card.len() == 2 {
        return (PokerHand::Pair, return_card);
    }

    // Default/base case when no other poker hands exist
    (PokerHand::HighCard, is_high_card(&cards))
}