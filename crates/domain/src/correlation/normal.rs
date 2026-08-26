const ERF_A1: f64 = 0.254_829_592;
const ERF_A2: f64 = -0.284_496_736;
const ERF_A3: f64 = 1.421_413_741;
const ERF_A4: f64 = -1.453_152_027;
const ERF_A5: f64 = 1.061_405_429;
const ERF_P: f64 = 0.327_591_1;

pub fn two_sided_tail(standard_score: f64) -> f64 {
    let beyond = 1.0 - error_function(standard_score.abs() / std::f64::consts::SQRT_2);

    beyond.clamp(0.0, 1.0)
}

fn error_function(x: f64) -> f64 {
    if x < 0.0 {
        return -error_function(-x);
    }

    let t = 1.0 / (1.0 + ERF_P * x);
    let series = t * (ERF_A1 + t * (ERF_A2 + t * (ERF_A3 + t * (ERF_A4 + t * ERF_A5))));

    1.0 - series * (-x * x).exp()
}
