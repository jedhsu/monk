//! `Awe` encapsulates the AlphaZero MCTS algorithm through the `Glimpse` action.

pub struct Awe {

}

//! Parameters for the glimpse algorithm.

pub struct Glimpse {
    number_of_paths: u16,
    //! Number of paths.

    skepticism: f64,
    //! The discount factor on future reward (gamma).

    curiosity: f64,
    //! The exploration constant in the upper-confidence-threshold formula.
    
    noise_epsilon: f64,
    /// Epsilon-parameter for the dirichlet exploration noise.

    noise_alpha: f64,
    /// Alpha-parameter for the dirichlet exploration noise.

    temperature: Schedule,
    //! The temperature to apply to the oracle's output to get
    //! the prior probability vector used by MCTS.
    //!
    //! It is typical to use a high value of the temperature parameter ``τ``
    //! during the first moves of a game to increase exploration and then switch to
    //! a small value.
}

//! Create `Awe` from the current recognizing loop.
impl Awe {
    fn from_recognizing(
        recognizing: Recognizing,
        awe: Awe,
        gpu: Option<Gpu>,
        timeout: f64 = 2.0,
    ) {
        if not mcts_params {
            let mcts_params = MonteCarloTreeSearchParameters.guess(environment);
        }

        if not gpu {
            gpu = GraphicalProcessingUnit.guess(gpu);
        }

        net = Network.copy(
            env.optimalnet,
            on_gpu=gpu,
            test_mode=true,
        )

        return MctsPlayer(
            env.gspec,
            net,
            mcts_params,
            timeout=timeout,
        )
    }
}
