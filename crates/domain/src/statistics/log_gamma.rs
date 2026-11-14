const LANCZOS_COEFFICIENTS: [f64; 6] = [
    76.180_091_729_471_46,
    -86.505_320_329_416_77,
    24.014_098_240_830_91,
    -1.231_739_572_450_155,
    0.001_208_650_973_866_179,
    -0.000_005_395_239_384_953,
];
const LANCZOS_SERIES_START: f64 = 1.000_000_000_190_015;
const LANCZOS_SHIFT: f64 = 5.5;
const SQUARE_ROOT_OF_TWO_PI: f64 = 2.506_628_274_631_000_5;
const HALF: f64 = 0.5;

pub(super) fn log_gamma(x: f64) -> f64 {
    let shifted = x + LANCZOS_SHIFT;
    let correction = shifted - (x + HALF) * shifted.ln();

    let series = LANCZOS_COEFFICIENTS
        .iter()
        .enumerate()
        .fold(LANCZOS_SERIES_START, |sum, (index, coefficient)| {
            sum + coefficient / (x + 1.0 + index as f64)
        });

    -correction + (SQUARE_ROOT_OF_TWO_PI * series / x).ln()
}
