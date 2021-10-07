//! The results of an episode of `Glimpse`.

pub struct Glimpsed {
    /// Average number of samples generated per second.
    /// TODO is this infer_frequency?
    glimpsing_frequency: f64,

    average_steps_ahead: f64,
    //!

    estimated_dream_scale: i32,
    //! estimation of the maximal memory footpri32 of the
    //! MCTS tree during &self-play, as computed by
    //! <`MCTS.approximate_memory_footpri32`>(@ref)
    //! &self-play phase
    
    /// number of samples in the memory buffer at the end of the
    num_glimpsed: i32.

    /// number of unique board positions in the
    /// memory buffer at the end of the &self-play phase
    num_realized: i32,
}
