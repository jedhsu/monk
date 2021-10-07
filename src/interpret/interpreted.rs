//! A single training sample.
//! z
//! 
//! As revealed by the last field `n`, several samples that correspond to the
//! same state can be merged, in which case the `π`, `z` and `t`
//! fields are averaged together.
//!

pub trait Interpret {}

pub struct Interpreted {
    /// The fractum state.
    state: &State,

    /// The cumulative discounted reward accumulated from a fractum.
    cumulative_reward: f64,

    /// The average number of moves remaining before the end of the game.
    expected_episodes_remaining: i32,

    //! The number of times the state `s` was recorded.
    temperature: f64,
}

