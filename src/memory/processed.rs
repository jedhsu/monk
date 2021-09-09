

pub struct Processed {
    /// The average number of samples processed per second.
    sampling_frequency: f64,

    /// Average number of explored steps ahead during the glimpse.
    average_exploration_depth: f64,
    
    //! estimation of the maximal memory footpri32 of the glimpse
    mcts_memory_footpri32: i32,

    //! number of samples in the memory buffer at the end of the &self-play phase
    memory_size: i32,

    //! number of unique board representations stored.
    number_of_unique_representations: i32,
}
