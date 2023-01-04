/// Functions for generating tokens
use rand::{distributions::Alphanumeric, Rng};

/// Make a random string of a given length.
pub fn generate_token(length: u16) -> String {
    let mut token = String::new();
    for char in rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length as usize)
    {
        token.push(char as char);
    }
    token
}

/// Compare two strings in constant time.
///
/// If the two strings are not the same length, this function will take the time
/// it would have taken to compare the strings if they were the shorter length.
/// This leaks information about the length of the strings, but not the contents.
pub fn compare_token(a: &str, b: &str) -> bool {
    let mut result = a.len() == b.len();
    for (a, b) in a.chars().zip(b.chars()) {
        result &= a == b;
    }
    result
}
