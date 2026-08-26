const PRESENT: f64 = 1.0;
const FEWEST_FOR_A_SPREAD: usize = 2;

pub fn welch_standard_score(values: &[f64], moods: &[f64]) -> Option<f64> {
    let present = group(values, moods, |value| value == PRESENT);
    let absent = group(values, moods, |value| value != PRESENT);

    if present.len() < FEWEST_FOR_A_SPREAD || absent.len() < FEWEST_FOR_A_SPREAD {
        return None;
    }

    let spread = (variance(&present)? / present.len() as f64
        + variance(&absent)? / absent.len() as f64)
        .sqrt();

    if spread == 0.0 {
        return None;
    }

    Some((mean(&present)? - mean(&absent)?) / spread)
}

fn group(values: &[f64], moods: &[f64], wanted: impl Fn(f64) -> bool) -> Vec<f64> {
    values
        .iter()
        .zip(moods)
        .filter(|(value, _)| wanted(**value))
        .map(|(_, mood)| *mood)
        .collect()
}

fn mean(moods: &[f64]) -> Option<f64> {
    if moods.is_empty() {
        return None;
    }

    Some(moods.iter().sum::<f64>() / moods.len() as f64)
}

fn variance(moods: &[f64]) -> Option<f64> {
    let average = mean(moods)?;
    let degrees_of_freedom = moods.len() as f64 - 1.0;

    Some(
        moods
            .iter()
            .map(|mood| (mood - average).powi(2))
            .sum::<f64>()
            / degrees_of_freedom,
    )
}
