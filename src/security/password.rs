/// Module for dealing with hashed passwords

use orion::pwhash;

/// Make a hash from a password.
/// Use this when creating or updating a password.
pub fn make_hash(password: &str) -> String {
    let pw = pwhash::Password::from_slice(password.as_bytes()).unwrap();
    let hash = pwhash::hash_password(&pw, 3, 1<<16).unwrap();
    let hash_str = hash.unprotected_as_encoded();
    hash_str.to_string()
}

/// Check a password against a hash.
/// Use this when logging in.
pub fn check_hash(password: &str, expected_hash: &str) -> bool {
    let hash = pwhash::PasswordHash::from_encoded(expected_hash).unwrap();
    let pw = pwhash::Password::from_slice(password.as_bytes()).unwrap();
    pwhash::hash_password_verify(&hash, &pw).is_ok()
}