use crate::entry::Mood;

const PRESENT: f64 = 1.0;

pub fn mean_difference(values: &[f64], moods: &[f64]) -> Option<f64> {
    let present: Vec<f64> = pick(values, moods, |value| value == PRESENT);
    let absent: Vec<f64> = pick(values, moods, |value| value != PRESENT);

    let difference = mean(&present)? - mean(&absent)?;

    Some(difference / mood_span())
}

fn pick(values: &[f64], moods: &[f64], wanted: impl Fn(f64) -> bool) -> Vec<f64> {
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

fn mood_span() -> f64 {
    f64::from(Mood::Rad.value() - Mood::Awful.value())
}
