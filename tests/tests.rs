extern crate optimization_methods_development;

use optimization_methods_development::methods::{segment_division, golden_section};
use optimization_methods_development::functions::{sample_func};

const A: f64 = 1.0;
const B: f64 = 5.0;
const EPS: f64 = 0.00001;
const EXPECTED_RESULT: f64 = 3.0;

fn check_result(result: f64) -> bool {
    result + EPS >= EXPECTED_RESULT || result - EPS <= EXPECTED_RESULT
}

#[test]
fn test_segment_division() {
    let result = segment_division::segment_divide(A, B, EPS, &sample_func);
    assert!(check_result(result));
}

#[test]
fn test_golden_section() {
    let result = golden_section::golden_section(A, B, EPS, &sample_func);
    assert!(check_result(result))
}