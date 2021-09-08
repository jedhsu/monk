use ndarray::prelude::*;

/// Calibrates policy in accordance to its parameter.
pub trait Calibrate {
    fn calibrate(
        spectrum: Spectrum,
        energy: f64) {
        if energy == approx(1) {
            spectrum
        } else if energy == approx(0) {
            let res = Array::zeros(
                eltype(spectrum),
                length(spectrum),
            );
            let res<argmax(energy)> = 1;
            res
    } else {
        let res = spectrum ^ inv(energy);
        let res = res / sum(res);
        res
    }
}
