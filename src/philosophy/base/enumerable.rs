/// Trait for policies that are defined on an enumerable action space.
pub trait EnumerablePolicy<S>:
    Policy<S, Action = usize> + Enumerable<(S,)>
where
    OutputOf<Self, (S,)>:
        std::ops::Index<usize, Output = f64> + IntoIterator<Item = f64>,
    <OutputOf<Self, (S,)> as IntoIterator>::IntoIter: ExactSizeIterator,
{
}

impl<S, P> EnumerablePolicy<S> for P
where
    P: Policy<S, Action = usize> + Enumerable<(S,)>,

    OutputOf<Self, (S,)>:
        std::ops::Index<usize, Output = f64> + IntoIterator<Item = f64>,
    <OutputOf<Self, (S,)> as IntoIterator>::IntoIter: ExactSizeIterator,
{
}
