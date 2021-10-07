

pub struct Processed {
    /// The average number of samples processed per second.
    sampling_frequency: f64,

    /// Average number of explored steps ahead during the Glimpsing phase.
    average_exploration_depth: f64,
    
    //! Estimation of the total memory load from the Glimpsing phase.
    mcts_memory_footpri32: i32,

    //! Number of samples in the memory buffer at the end of the Glimpsing phase.
    memory_size: i32,

    //! number of unique board representations stored.
    number_of_unique_representations: i32,
}
