//! # Poker Module
//!
//! This module contains all the logic for evaluating and scoring poker rounds.
//! It is organized into several submodules:
//!
//! - [`hands`] — functions for determining poker hands and card order.
//! - [`helpers`] — utility functions for suits, colors, and related calculations.
//! - [`jokers`] — logic for applying joker effects to hands and scores.
//! - [`modifiers`] — functions for applying scoring modifiers and enhancements.
//! - [`scoring`] — the main scoring pipeline, including [`score`].
//!
//! ## Example
//! ```
//! use ortalib::poker::{determine_poker_hand, score};
//! use ortalib::Round;
//!
//! // Construct a dummy round (fill with real cards in practice)
//! let round = Round {
//!     cards_played: vec![A♥,K♠,8♦,6♣,4♥],
//!     cards_held_in_hand: vec![],
//!     jokers: vec![],
//! };
//!
//! let (chips, mult) = score(round);
//! assert!(chips >= 0.0);
//! assert!(mult >= 1.0);
//! ```

pub mod hands;
pub mod helpers;
pub mod jokers;
pub mod modifiers;
pub mod scoring;

pub use hands::{compute_card_order, determine_poker_hand};
pub use helpers::{compute_most_appear_suit, determine_current_suit, determine_total_colors};
pub use modifiers::apply_edition;
pub use scoring::score;
