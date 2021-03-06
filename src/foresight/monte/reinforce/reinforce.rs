use crate::{domains::Batch, fa::StateActionUpdate, policies::Policy, Handler};

#[derive(Clone, Debug, Parameterised)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub struct Reinforce<P> {
    #[weights]
    pub policy: P,

    pub alpha: f64,
    pub gamma: f64,
}

impl<P> Reinforce<P> {
    pub fn new(
        policy: P,
        alpha: f64,
        gamma: f64,
    ) -> Self {
        Reinforce {
            policy,

            alpha,
            gamma,
        }
    }
}

impl<'m, S, P> Handler<&'m Batch<S, P::Action>> for Reinforce<P>
where
    P: Policy<S>
        + Handler<StateActionUpdate<&'m S, &'m <P as Policy<S>>::Action>>,
{
    type Response = Vec<P::Response>;
    type Error = P::Error;

    fn handle(
        &mut self,
        batch: &'m Batch<S, P::Action>,
    ) -> Result<Self::Response, Self::Error> {
        let mut ret = 0.0;

        batch
            .iter()
            .map(|t| {
                ret = t.reward + self.gamma * ret;

                self.policy.handle(StateActionUpdate {
                    state: t.from.state(),
                    action: &t.action,
                    error: self.alpha * ret,
                })
            })
            .collect()
    }
}
