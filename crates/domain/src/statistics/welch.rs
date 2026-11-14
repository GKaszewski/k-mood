const FEWEST_FOR_A_SPREAD: usize = 2;

pub fn welch_two_sample(first: &[f64], second: &[f64]) -> Option<f64> {
    let (error_first, error_second) = squared_standard_errors(first, second)?;

    Some((mean(second)? - mean(first)?) / (error_first + error_second).sqrt())
}

pub fn welch_satterthwaite_degrees_of_freedom(first: &[f64], second: &[f64]) -> Option<f64> {
    let (error_first, error_second) = squared_standard_errors(first, second)?;

    let spread_of_the_difference = (error_first + error_second).powi(2);
    let spread_of_the_spreads = error_first.powi(2) / degrees_of_freedom(first)
        + error_second.powi(2) / degrees_of_freedom(second);

    Some(spread_of_the_difference / spread_of_the_spreads)
}

fn squared_standard_errors(first: &[f64], second: &[f64]) -> Option<(f64, f64)> {
    if first.len() < FEWEST_FOR_A_SPREAD || second.len() < FEWEST_FOR_A_SPREAD {
        return None;
    }

    let error_first = variance(first)? / first.len() as f64;
    let error_second = variance(second)? / second.len() as f64;

    if error_first + error_second == 0.0 {
        return None;
    }

    Some((error_first, error_second))
}

fn mean(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }

    Some(values.iter().sum::<f64>() / values.len() as f64)
}

fn variance(values: &[f64]) -> Option<f64> {
    let average = mean(values)?;

    Some(
        values
            .iter()
            .map(|value| (value - average).powi(2))
            .sum::<f64>()
            / degrees_of_freedom(values),
    )
}

fn degrees_of_freedom(values: &[f64]) -> f64 {
    values.len() as f64 - 1.0
}
