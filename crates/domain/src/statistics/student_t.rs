use super::incomplete_beta::regularised_incomplete_beta;

const HALF: f64 = 0.5;

pub fn student_t_two_sided(statistic: f64, degrees_of_freedom: f64) -> f64 {
    let closeness_to_zero = degrees_of_freedom / (degrees_of_freedom + statistic * statistic);

    regularised_incomplete_beta(degrees_of_freedom * HALF, HALF, closeness_to_zero).clamp(0.0, 1.0)
}
