//! A glimpse at a possible action.

pub type Probability<T> = T;

pub struct ActionGlimpse {
    prior_probability: Probability<f64>,
    //! Prior probability computed.
    
    reward: f64,
    //! Cumulative Q-Value for the action.

    number_of_visits: i32,
    //! Number of times the action has been visited.
}
