//! A generic, standalone implementation of Monte Carlo Tree Search.
//!
//! An oracle can be any function or callable object.
//!
//!     oracle(state)
//!
//! evaluates a single state from the current player's perspective and returns
//! a pair `(P, V)` where:
//!

// impl<'m, S, Q, R> Handler<&'m Transition<S, usize>>
//     for QLambda<Q, Tr<&'m S, usize, Q, R>>
// where
//     Q: Enumerable<(&'m S,)>
//         + Differentiable<(&'m S, usize)>
//         + for<'j> Handler<ScaledGradientUpdate<&'j Tr<&'m S, usize, Q, R>>>,
//     R: traces::UpdateRule<<Q as Differentiable<(&'m S, usize)>>::Jacobian>,

//     <Q as Function<(&'m S,)>>::Output:
//         Index<usize, Output = f64> + IntoIterator<Item = f64>,
//     <<Q as Function<(&'m S,)>>::Output as IntoIterator>::IntoIter:
//         ExactSizeIterator,
// {
//     type Response = Response;
//     type Error = ();

//     fn handle(
//         &mut self,
//         t: &'m Transition<S, usize>,
//     ) -> Result<Self::Response, Self::Error> {
//         let s = t.from.state();

//         let qs = self.fa_theta.evaluate((s,));
//         let qsa = qs[t.action];
//         let grad_s = self.fa_theta.grad((s, t.action));

//         // Update trace:
//         if t.action != argmax_first(qs).0 {
//             self.trace.reset();
//         }

//         self.trace.update(&grad_s);

//         let td_error = if t.terminated() {
//             let residual = t.reward - qsa;

//             self.fa_theta
//                 .handle(ScaledGradientUpdate {
//                     alpha: self.alpha * residual,
//                     jacobian: &self.trace,
//                 })
//                 .map_err(|_| ())?;

//             self.trace.reset();

//             residual
//         } else {
//             let ns = t.to.state();
//             let (_, nqs_max) = self.fa_theta.find_max((ns,));

//             let residual = t.reward + self.gamma * nqs_max - qsa;

//             self.fa_theta
//                 .handle(ScaledGradientUpdate {
//                     alpha: self.alpha * residual,
//                     jacobian: &self.trace,
//                 })
//                 .map_err(|_| ())?;

//             residual
//         };

//         Ok(Response { td_error })
//     }
// }
