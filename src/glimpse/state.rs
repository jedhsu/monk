//! A glimpse at a possible state.

pub struct StateGlimpse {
    actions: Iterator<GlimpsedAction>,
    value: f64,
    //! Value estimated.
}

impl StateGlimpse { 
    fn new(
        prior_probability: f64,
        energy: f64,
        prior_temperature: f64,
    ):
        let P = Util.apply_temperature(
            prior_probability,
            prior_temperature,
        );
        let glimpsed_actions = {GlimpsedAction(prior, 0, 0) for prior in P}
        GlimpsedState {glimpsed_actions, energy}
    
    /// Total number of visited states.
    fn total_visted(&self) -> i32 {
        sum(action.num_visited for action in &self.actions)
    }
}
