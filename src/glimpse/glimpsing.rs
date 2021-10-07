

/// The environment state of a `Glimpse`.
pub struct Glimpsing {
    /// Glimpse parameters.
    parameters: Glimpse,

    
    //! Store (nonterminal) state statistics assuming the white player is to play
    vision: HashMap<State, Analysis>,
    
    /// The parameters describing the world's rules.
    nature: Nature,

    /// The number of glimpsed paths.
    number_of_paths: u64,
    
    /// The number of unique states visited.
    unique_visits: u64,

}
