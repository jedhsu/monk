pub struct Action<S> {
    state: S,
    action: <Self as Policy<S>>::Action,
}

/// Policy trait for functions that define a probability distribution over
/// actions.
pub trait Philosophy<S>:
    Function<Action<S>, Output = f64> + for<'a> Function<Action<S>, Output = f64>
{
    type Action: Sized;

    /// Sample the (possibly stochastic) policy distribution for a given
    /// `state`.
    fn sample<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        state: S,
    ) -> Self::Action;

    /// Return the most probable action according to the policy distribution,
    /// if well-defined.
    fn mode(
        &self,
        state: S,
    ) -> Self::Action;
}

impl<S, T: Policy<S>> Policy<S> for Shared<T> {
    type Action = T::Action;

    fn sample<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        state: S,
    ) -> Self::Action {
        self.borrow().sample(rng, state)
    }

    fn mode(
        &self,
        state: S,
    ) -> Self::Action {
        self.borrow().mode(state)
    }
}
