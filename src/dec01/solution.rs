use std::error::Error;

// Prepared for december first problem
pub fn solve(s: &str) -> &str {
    println!("Solution is: '{}'", s);
    s
}

pub async fn solve_async_errorhandling(s: &str)-> Result<&str, Box<dyn Error>> {
    println!("Solution is: '{}'", s);
    Ok(s)
}