//! A MCTS is Glimpse.
//! 
//! Parameters for the glimpse.

//! An MCTS player picks an action as follows.
//!
//! Given a game state, it launches `num_iters_per_turn` MCTS iterations, with UCT exploration constant `cpuct`.
//! Rewards are discounted using the `gamma` factor.
//!
//! Then, an action is picked according to the distribution ``π`` where ``π_i ∝ n_i^{1/τ}`` with ``n_i`` the number
//! of times that the ``i^{\\text{th}}`` action was visited and ``τ`` the `temperature` parameter.
//!
//! Therefore, `temperature` is am <`AbstractSchedule`>(@ref).
//!
//! For information on parameters `cpuct`, `dirichlet_noise_ϵ`
//!
//! `dirichlet_noise_α` and `prior_temperature`, see <`MCTS.Env`>(@ref).
//!
//!
//! + The temperature is set to 1 for the 30 first moves and then to an
//!   infinitesimal value.
//!

//! A training sample.
//!
//! As revealed by the last field `n`, several samples that correspond to the
//! same state can be merged, in which case the `π`, `z` and `t`
//! fields are averaged together.


pub struct Flashed {
    position: Position,
    //! `s::State` is the state

    policy: Policy,
    //! `π::Vector{Float64}` is the recorded MCTS policy for this position
    
    energy: Energy,
    //! Energy is the discounted reward cumulated from state `s`.

    expected_time_left: f64,
    //! The estimated number of days remaining before the world ends.

    heat: i32,
    //! `n::Int` is the number of times the state `s` was recorded
}

