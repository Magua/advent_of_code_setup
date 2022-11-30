
#[path="utils/mod.rs"]
mod utils;

use advent_of_code_setup::d2101::solution::*;

#[test]
fn verify_example_input() {
    match aw!(count_rows_increased("./src/d2101/example_input")) {
        Ok(result) => assert_eq!(result, 7),
        Err(e) => panic!("{}", e)
    }
}

#[test]
fn verify_input() {
    match aw!(count_rows_increased("./src/d2101/input")) {
        Ok(result) => assert_eq!(result, 1387),
        Err(e) => panic!("{}", e)
    }
}

#[test]
fn verify_sliding_window_example() {
    match aw!(count_windows_increased("./src/d2101/example_input")) {
        Ok(result) => assert_eq!(result, 5),
        Err(e) => panic!("{}", e)
    }
}

#[test]
fn verify_sliding_window() {
    match aw!(count_windows_increased("./src/d2101/input")) {
        Ok(result) => assert_eq!(result, 1362),
        Err(e) => panic!("{}", e)
    }
}

