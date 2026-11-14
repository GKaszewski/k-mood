use crate::statistics::welch_two_sample;

const PRESENT: f64 = 1.0;

pub fn welch_standard_score(values: &[f64], moods: &[f64]) -> Option<f64> {
    let present = group(values, moods, |value| value == PRESENT);
    let absent = group(values, moods, |value| value != PRESENT);

    welch_two_sample(&absent, &present)
}

fn group(values: &[f64], moods: &[f64], wanted: impl Fn(f64) -> bool) -> Vec<f64> {
    values
        .iter()
        .zip(moods)
        .filter(|(value, _)| wanted(**value))
        .map(|(_, mood)| *mood)
        .collect()
}
