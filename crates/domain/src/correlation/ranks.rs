pub fn average_ranks(values: &[f64]) -> Vec<f64> {
    let mut order: Vec<usize> = (0..values.len()).collect();
    order.sort_by(|left, right| values[*left].total_cmp(&values[*right]));

    let mut ranks = vec![0.0; values.len()];
    let mut start = 0;

    while start < order.len() {
        let mut end = start;
        while end + 1 < order.len() && values[order[end + 1]] == values[order[start]] {
            end += 1;
        }

        let shared = shared_rank(start, end);
        for position in &order[start..=end] {
            ranks[*position] = shared;
        }

        start = end + 1;
    }

    ranks
}

fn shared_rank(start: usize, end: usize) -> f64 {
    let first = start as f64 + 1.0;
    let last = end as f64 + 1.0;

    (first + last) / 2.0
}
