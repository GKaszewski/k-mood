pub fn two_proportion(
    hits_first: usize,
    total_first: usize,
    hits_second: usize,
    total_second: usize,
) -> Option<f64> {
    if total_first == 0 || total_second == 0 {
        return None;
    }

    let pooled = (hits_first + hits_second) as f64 / (total_first + total_second) as f64;
    let spread =
        (pooled * (1.0 - pooled) * (1.0 / total_first as f64 + 1.0 / total_second as f64)).sqrt();

    if spread == 0.0 {
        return None;
    }

    let proportion_first = hits_first as f64 / total_first as f64;
    let proportion_second = hits_second as f64 / total_second as f64;

    Some((proportion_second - proportion_first) / spread)
}
