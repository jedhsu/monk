use crate::{
    domains::Transition, fa::StateActionUpdate, Enumerable, Function, Handler,
    Parameterised,
};
use std::ops::Index;

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub struct Response<R> {
    pub q_res: R,
    pub error: f64,
}

/// Watkins' Q-learning.
///
///   - Watkins, C. J. C. H. (1989). Learning from Delayed Rewards. Ph.D. thesis,
///   Cambridge University.
///
///   - Watkins, C. J. C. H., Dayan, P. (1992). Q-learning. Machine Learning,
///   8:279–292.
///
#[derive(Clone, Debug, Parameterised)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub struct QLearning<Q> {
    #[weights]
    pub q_func: Q,

    pub gamma: f64,
}

impl<'m, S, Q> Handler<&'m Transition<S, usize>> for QLearning<Q>
where
    Q: Enumerable<(&'m S,)> + Handler<StateActionUpdate<&'m S, usize, f64>>,
    <Q as Function<(&'m S,)>>::Output:
        Index<usize, Output = f64> + IntoIterator<Item = f64>,
    <<Q as Function<(&'m S,)>>::Output as IntoIterator>::IntoIter:
        ExactSizeIterator,
{
    type Response = Response<Q::Response>;
    type Error = Q::Error;

    fn handle(
        &mut self,
        t: &'m Transition<S, usize>,
    ) -> Result<Self::Response, Self::Error> {
        let state = t.from.state();
        let qsa = self.q_func.evaluate_index((state,), t.action);

        let error = if t.terminated() {
            t.reward - qsa
        } else {
            let ns = t.to.state();
            let (_, nqsna) = self.q_func.find_max((ns,));

            t.reward + self.gamma * nqsna - qsa
        };

        self.q_func
            .handle(StateActionUpdate {
                state,
                action: t.action,
                error,
            })
            .map(|q_res| Response { q_res, error })
    }
}
