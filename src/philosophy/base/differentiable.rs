/// Trait for policies with a representation that is differentiable wrt its
/// parameters.
pub trait DifferentiablePolicy<S>:
    Policy<S>
    + Differentiable<(S, <Self as Policy<S>>::Action), Jacobian = Array2<f64>>
    + for<'a> Differentiable<
        (S, &'a <Self as Policy<S>>::Action),
        Jacobian = Array2<f64>,
    >
{
}

impl<S, P> DifferentiablePolicy<S> for P where
    P: Policy<S>
        + Differentiable<(S, <Self as Policy<S>>::Action), Jacobian = Array2<f64>>
        + for<'a> Differentiable<
            (S, &'a <Self as Policy<S>>::Action),
            Jacobian = Array2<f64>,
        >
{
}
