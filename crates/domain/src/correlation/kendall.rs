pub fn kendall_tau_b(left: &[f64], right: &[f64]) -> Option<f64> {
    let mut concordant = 0i64;
    let mut discordant = 0i64;

    for first in 0..left.len() {
        for second in (first + 1)..left.len() {
            let agreement =
                direction(left[first] - left[second]) * direction(right[first] - right[second]);

            match agreement {
                1 => concordant += 1,
                -1 => discordant += 1,
                _ => continue,
            }
        }
    }

    let pairs = pair_count(left.len());
    let denominator = ((pairs - tied_pairs(left)) * (pairs - tied_pairs(right))).sqrt();

    if denominator == 0.0 {
        return None;
    }

    Some((concordant - discordant) as f64 / denominator)
}

fn direction(difference: f64) -> i64 {
    if difference > 0.0 {
        return 1;
    }
    if difference < 0.0 {
        return -1;
    }

    0
}

fn pair_count(len: usize) -> f64 {
    let len = len as f64;

    len * (len - 1.0) / 2.0
}

fn tied_pairs(values: &[f64]) -> f64 {
    let mut sorted = values.to_vec();
    sorted.sort_by(|left, right| left.total_cmp(right));

    let mut tied = 0.0;
    let mut start = 0;

    while start < sorted.len() {
        let mut end = start;
        while end + 1 < sorted.len() && sorted[end + 1] == sorted[start] {
            end += 1;
        }

        tied += pair_count(end - start + 1);
        start = end + 1;
    }

    tied
}
