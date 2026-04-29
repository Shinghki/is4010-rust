#![allow(dead_code, unused_imports)]
use rand::Rng;

pub fn generate_random(length: usize, use_symbols: bool) -> String {
    let mut charset = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
    if use_symbols {
        charset.push_str("!@#$%^&*");
    }
    let chars: Vec<char> = charset.chars().collect();
    let mut rng = rand::thread_rng();
    (0..length).map(|_| chars[rng.gen_range(0..chars.len())]).collect()
}

pub fn generate_passphrase(word_count: usize, separator: char) -> String {
    let mut rng = rand::thread_rng();
    let words: Vec<&str> = (0..word_count)
        .map(|_| WORD_LIST[rng.gen_range(0..WORD_LIST.len())])
        .collect();
    words.join(&separator.to_string())
}

pub fn generate_pin(length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.gen_range(0..10).to_string()).collect()
}

pub const WORD_LIST: &[&str] = &[
    "apple", "river", "cloud", "stone", "flame", "ocean", "tiger", "maple", "storm", "frost",
    "eagle", "cedar", "brook", "ember", "coral", "prism", "solar", "lunar", "amber", "blaze",
    "cliff", "delta", "fable", "grove", "haven", "ivory", "jewel", "knoll", "lemon", "meadow",
    "north", "olive", "pearl", "quill", "ridge", "spark", "thorn", "umbra", "valor", "willow",
    "xenon", "yarrow", "zenith", "acorn", "birch", "crane", "drift", "elder", "flint", "glade",
    "hyena", "inlet", "junco", "kestrel",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_correct_length() { assert_eq!(generate_random(12, false).len(), 12); }
    #[test]
    fn test_random_no_symbols_only_alphanumeric() {
        assert!(generate_random(100, false).chars().all(|c| c.is_ascii_alphanumeric()));
    }
    #[test]
    fn test_random_with_symbols_contains_valid_chars() {
        let valid: std::collections::HashSet<char> =
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*".chars().collect();
        assert!(generate_random(100, true).chars().all(|c| valid.contains(&c)));
    }
    #[test]
    fn test_random_length_one() { assert_eq!(generate_random(1, false).len(), 1); }
    #[test]
    fn test_passphrase_word_count() { assert_eq!(generate_passphrase(4, '-').split('-').count(), 4); }
    #[test]
    fn test_passphrase_separator() {
        let phrase = generate_passphrase(3, '_');
        assert!(phrase.contains('_'));
        assert!(!phrase.contains('-'));
    }
    #[test]
    fn test_passphrase_words_from_list() {
        for word in generate_passphrase(5, '-').split('-') {
            assert!(WORD_LIST.contains(&word));
        }
    }
    #[test]
    fn test_passphrase_single_word() {
        let phrase = generate_passphrase(1, '-');
        assert!(!phrase.contains('-'));
        assert!(WORD_LIST.contains(&phrase.as_str()));
    }
    #[test]
    fn test_pin_correct_length() { assert_eq!(generate_pin(6).len(), 6); }
    #[test]
    fn test_pin_only_digits() { assert!(generate_pin(20).chars().all(|c| c.is_ascii_digit())); }
    #[test]
    fn test_pin_length_one() { assert_eq!(generate_pin(1).len(), 1); }
}