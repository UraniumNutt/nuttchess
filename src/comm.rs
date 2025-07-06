use std::io;

// Returns a string from input
pub fn engine_in() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}
