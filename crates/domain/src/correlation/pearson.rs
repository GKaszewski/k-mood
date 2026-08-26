pub fn pearson(left: &[f64], right: &[f64]) -> Option<f64> {
    let count = left.len() as f64;
    let left_mean = left.iter().sum::<f64>() / count;
    let right_mean = right.iter().sum::<f64>() / count;

    let covariance: f64 = left
        .iter()
        .zip(right)
        .map(|(l, r)| (l - left_mean) * (r - right_mean))
        .sum();

    let left_variance: f64 = left.iter().map(|l| (l - left_mean).powi(2)).sum();
    let right_variance: f64 = right.iter().map(|r| (r - right_mean).powi(2)).sum();

    if left_variance == 0.0 || right_variance == 0.0 {
        return None;
    }

    Some(covariance / (left_variance * right_variance).sqrt())
}
