use rand::{Rng, distributions::Alphanumeric};

// fairly unique 6 digit case sensitive string generator
pub fn link_short_id() -> String {
    let string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();
    string
}
