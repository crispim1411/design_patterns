use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn generate_id(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}