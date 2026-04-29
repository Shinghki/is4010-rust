#![allow(dead_code)]
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum PasswordStrength { Weak, Medium, Strong, VeryStrong }

impl fmt::Display for PasswordStrength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            PasswordStrength::Weak => "Weak",
            PasswordStrength::Medium => "Medium",
            PasswordStrength::Strong => "Strong",
            PasswordStrength::VeryStrong => "Very strong",
        };
        write!(f, "{}", label)
    }
}

pub fn validate_strength(password: &str) -> PasswordStrength {
    let mut score = 0;
    if password.len() >= 8 { score += 1; }
    if password.len() >= 12 { score += 1; }
    if password.len() >= 16 { score += 1; }
    if password.chars().any(|c| c.is_ascii_lowercase()) { score += 1; }
    if password.chars().any(|c| c.is_ascii_uppercase()) { score += 1; }
    if password.chars().any(|c| c.is_ascii_digit()) { score += 1; }
    if password.chars().any(|c| !c.is_ascii_alphanumeric()) { score += 1; }
    match score {
        0..=2 => PasswordStrength::Weak,
        3..=4 => PasswordStrength::Medium,
        5..=6 => PasswordStrength::Strong,
        _ => PasswordStrength::VeryStrong,
    }
}

pub fn check_common_patterns(password: &str) -> bool {
    let lower = password.to_lowercase();
    if COMMON_PASSWORDS.iter().any(|&p| p == lower) { return true; }
    let chars: Vec<char> = password.chars().collect();
    !chars.is_empty() && chars.iter().all(|&c| c == chars[0])
}

pub fn calculate_entropy(password: &str) -> f64 {
    if password.is_empty() { return 0.0; }
    let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
    let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_symbol = password.chars().any(|c| !c.is_ascii_alphanumeric());
    let charset_size = match (has_lower, has_upper, has_digit, has_symbol) {
        (true, false, false, false) => 26,
        (_, true, false, false) => 52,
        (_, _, true, false) => 62,
        _ => 94,
    };
    f64::log2(charset_size as f64) * password.len() as f64
}

pub const COMMON_PASSWORDS: &[&str] = &[
    "password", "123456", "password123", "qwerty", "letmein",
    "iloveyou", "admin", "welcome", "monkey", "dragon",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strength_weak_short() { assert_eq!(validate_strength("hi"), PasswordStrength::Weak); }
    #[test]
    fn test_strength_medium() { assert_eq!(validate_strength("Password"), PasswordStrength::Medium); }
    #[test]
    fn test_strength_strong() { assert_eq!(validate_strength("Password1!"), PasswordStrength::Strong); }
    #[test]
    fn test_strength_very_strong() { assert_eq!(validate_strength("MyStr0ng!Pass2024"), PasswordStrength::VeryStrong); }
    #[test]
    fn test_strength_display() {
        assert_eq!(format!("{}", PasswordStrength::Weak), "Weak");
        assert_eq!(format!("{}", PasswordStrength::Medium), "Medium");
        assert_eq!(format!("{}", PasswordStrength::Strong), "Strong");
        assert_eq!(format!("{}", PasswordStrength::VeryStrong), "Very strong");
    }
    #[test]
    fn test_common_password_detected() {
        assert!(check_common_patterns("password"));
        assert!(check_common_patterns("123456"));
        assert!(check_common_patterns("PASSWORD"));
    }
    #[test]
    fn test_all_same_char_detected() {
        assert!(check_common_patterns("aaaa"));
        assert!(check_common_patterns("1111"));
        assert!(check_common_patterns("ZZZZ"));
    }
    #[test]
    fn test_unique_password_not_flagged() { assert!(!check_common_patterns("X7#kP2@mQ9")); }
    #[test]
    fn test_entropy_lowercase_only() {
        let e = calculate_entropy("abcd");
        assert!((e - 4.0 * f64::log2(26.0)).abs() < 1e-9);
    }
    #[test]
    fn test_entropy_mixed_case() {
        let e = calculate_entropy("abCD");
        assert!((e - 4.0 * f64::log2(52.0)).abs() < 1e-9);
    }
    #[test]
    fn test_entropy_alphanumeric() {
        let e = calculate_entropy("aB3d");
        assert!((e - 4.0 * f64::log2(62.0)).abs() < 1e-9);
    }
    #[test]
    fn test_entropy_with_symbols() {
        let e = calculate_entropy("aB3!");
        assert!((e - 4.0 * f64::log2(94.0)).abs() < 1e-9);
    }
    #[test]
    fn test_entropy_empty() { assert_eq!(calculate_entropy(""), 0.0); }
}