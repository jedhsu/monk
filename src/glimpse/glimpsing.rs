

/// The environment state of a `Glimpse`.
pub struct Glimpsing {
    parameters: Glimpse,
    //! Glimpse parameters.

    interpreter: Demon,
    //! External monk to evaluate positions
    
    vision: HashMap<State, Analysis>,
    //! Store (nonterminal) state statistics assuming the white player is to play
    
    nature: Nature,
    //! The parameters describing the world's rules.

    number_of_paths: u64,
    //! The number of glimpsed paths.
    
    unique_visits: u64,
    //! The number of unique states visited.

}



impl Glimpsing {
    /// Return the upper confidence threshold.
    fn upper_confidence_threshold(&self) -> f32 {
        assert!((self.parameters.noise_epsilon == 0) || (&self.estimated_time_remaining() == &self.state.actions.len()));

        let sqrt_num_visited = glimpsed_state.total_visted().sqrt();

        for i, action in enumerate(glimpsed_state.actions) {
            let energy = action.energy / action.num_visited.max(1);

            if self.parameters.noise_epsilon == 0 {
                let probability = action.prior_probability;
            } else if {
                // [TODO] clean me functionally
                let probability = (1 - epsilon) * action.prior_probability + epsilon * eta[
                    i
                ]
            }

            energy + cpuct * probability * sqrt_num_visited / (action.number_of_visits + 1)
        }
    }

    /// Return the average number of nodes that are traversed
    /// during a glimpse, not counting the root.
    fn depth_of_analysis(&self) -> f32 {
        if self.total_simulations == 0 {
            0
        } else  {
            self.total_nodes_traversed / self.total_simulations
        }
    }

    fn update(&self, state: GlimpsedState, action_id, q) {
    }
}
