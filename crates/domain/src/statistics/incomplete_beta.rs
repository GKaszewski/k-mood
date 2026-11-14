use super::log_gamma::log_gamma;

const ITERATION_CAP: usize = 1000;
const CONVERGENCE_EPSILON: f64 = 3.0e-14;
const SMALLEST_DIVISOR: f64 = 1.0e-300;

pub(super) fn regularised_incomplete_beta(a: f64, b: f64, x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }

    let front =
        (log_gamma(a + b) - log_gamma(a) - log_gamma(b) + a * x.ln() + b * (1.0 - x).ln()).exp();

    if x < (a + 1.0) / (a + b + 2.0) {
        front * continued_fraction(a, b, x) / a
    } else {
        1.0 - front * continued_fraction(b, a, 1.0 - x) / b
    }
}

fn continued_fraction(a: f64, b: f64, x: f64) -> f64 {
    let mut numerator_term = 1.0;
    let mut denominator_term = away_from_zero(1.0 - (a + b) * x / (a + 1.0)).recip();
    let mut fraction = denominator_term;

    for step in 1..=ITERATION_CAP {
        let step = step as f64;
        let twice_step = 2.0 * step;

        let even = step * (b - step) * x / ((a - 1.0 + twice_step) * (a + twice_step));
        denominator_term = away_from_zero(1.0 + even * denominator_term).recip();
        numerator_term = away_from_zero(1.0 + even / numerator_term);
        fraction *= denominator_term * numerator_term;

        let odd = -(a + step) * (a + b + step) * x / ((a + twice_step) * (a + 1.0 + twice_step));
        denominator_term = away_from_zero(1.0 + odd * denominator_term).recip();
        numerator_term = away_from_zero(1.0 + odd / numerator_term);
        let change = denominator_term * numerator_term;
        fraction *= change;

        if (change - 1.0).abs() < CONVERGENCE_EPSILON {
            break;
        }
    }

    fraction
}

fn away_from_zero(value: f64) -> f64 {
    if value.abs() < SMALLEST_DIVISOR {
        SMALLEST_DIVISOR
    } else {
        value
    }
}
