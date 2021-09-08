use ndarray::prelude::*;

pub trait Synthesize {
    fn synthesize(self, laws: Laws, observed: Observed<T>) -> Observed<T>;
    // Applies a symmetry.
}

impl Synthesize for Processing {
    fn synthesize(
        &self,
        laws: Laws,
        observed: Observed<T>,
        // (symstate, aperm),
    ) {
        interpreting = Interpreting.initialize(laws, observed);

        let mask = GlobalInterpreter.actions_mask(interpreting);

        let symmask = GlobalInterpreter.actions_mask(
            GlobalInterpreter.init(gspec, symstate)
        );

        let spectrum = Array::zeros(eltype(sample.π), length(mask));
        let spectrum<mask> = observed.spectrum;
        let spectrum= spectrum<aperm>;
        let assert spectrum<.~symmask> == 0;
        let spectrum = spectrum<symmask>;
        
        Observed<T>::new(symstate, π, sample.z, sample.t, sample.n,);
    }
}
