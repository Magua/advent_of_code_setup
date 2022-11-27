
#[path="utils/mod.rs"]
mod utils;

use advent_of_code_setup::dec00::solution::*;

#[test]
fn verify_echo_simple() {
    let result = echo_simple("a string");
    assert_eq!(result, "a string");
}

#[test]
fn verify_error_handling_ok() {
    match echo_error_handling("a string") {
        Ok(result) => assert_eq!(result, "a string"),
        _ => panic!("Expected a string"),
    }
}

#[test]
fn verify_error_handling_not_ok() {
    match echo_error_handling("profanity") {
        Err(result) => assert_eq!(result, "oh dear"),
        _ => panic!("Should have failed"),
    }
}

#[test]
fn verify_async_ok() {
    match aw!(asynch_echo("a string")) {
        Ok(result) => assert_eq!(result, "a string"),
        _ => panic!("Expected a string"),
    }
}

#[test]
fn verify_count_rows_containing() {
    match aw!(count_rows_containing("./src/dec00/dec00.txt", "l")) {
        Ok(result) => assert_eq!(result, 2),
        Err(r) => panic!("Expected 2, got {}", r),
    }
}
