use ortalib::{Card, Chips, Edition, Enhancement, Mult };

fn apply_enhancement(enhancement: Enhancement, input_chip: f64, input_mul: f64, in_hand: bool) -> (Chips, Mult) {
  let (mut chip, mut mul) = (input_chip, input_mul);

  if !in_hand {
    match enhancement {
      Enhancement::Bonus => chip += 30.0,
      Enhancement::Mult => mul += 4.0,
      Enhancement::Wild => {},
      Enhancement::Glass => mul *= 2.0,
      Enhancement::Steel => {},
    }
  } else {
    if enhancement == Enhancement::Steel {
      mul *= 1.5;
    }
  }

  (chip, mul)
}

fn apply_edition(edition: Edition, input_chip: f64, input_mul: f64, in_hand: bool) -> (Chips, Mult) {
 let (mut chip, mut mul) = (input_chip, input_mul);

  if !in_hand {
    match edition {
     Edition::Foil => chip += 50.0,
     Edition::Holographic => mul += 10.0,
     Edition::Polychrome => mul *= 1.5,
    }
  }
  (chip, mul)
}

pub fn compute_enhancement(cards: Vec<Card>, chip: f64, mul: f64, in_hand: bool) -> (Chips, Mult) {
  let (mut new_chip, mut new_mul) = (chip, mul);
  cards.iter().for_each(|card| {
    if let Some(enhancement) = card.enhancement {
      let result = apply_enhancement(enhancement, new_chip, new_mul, in_hand);
      new_chip = result.0;
      new_mul = result.1;
    } 

    if let Some(edition) = card.edition {
      let result = apply_edition(edition, new_chip, new_mul, in_hand);
      new_chip = result.0;
      new_mul = result.1;
    }
    // dbg!(new_mul);
  });

  (new_chip, new_mul)
}