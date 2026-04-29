// Lab 09: Rust Basics

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn square(n: i32) -> i32 {
    n * n
}

fn main() {
    println!("5 + 3 = {}", add(5, 3));
    println!("4 * 7 = {}", multiply(4, 7));
    println!("Is 10 even? {}", is_even(10));
    println!("Max of 42 and 17: {}", max(42, 17));
    println!("Square of 6: {}", square(6));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
        assert_eq!(multiply(-2, 5), -10);
        assert_eq!(multiply(0, 100), 0);
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(4));
        assert!(is_even(0));
        assert!(!is_even(7));
        assert!(!is_even(-3));
    }

    #[test]
    fn test_max() {
        assert_eq!(max(5, 10), 10);
        assert_eq!(max(10, 5), 10);
        assert_eq!(max(-5, -10), -5);
        assert_eq!(max(0, 0), 0);
    }

    #[test]
    fn test_square() {
        assert_eq!(square(5), 25);
        assert_eq!(square(0), 0);
        assert_eq!(square(-3), 9);
    }
}
