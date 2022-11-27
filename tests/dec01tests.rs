
#[path="utils/mod.rs"]
mod utils;

use advent_of_code_setup::dec01::solution::*;

#[test]
fn verify_echo_simple() {
    let result = solve("a string");
    assert_eq!(result, "a string");
}