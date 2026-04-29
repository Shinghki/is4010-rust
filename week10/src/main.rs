// Week 10: Mastering ownership and borrowing

fn main() {
    println!("Week 10: Mastering ownership and borrowing");
    problem_1();
    problem_2();
    problem_3();
    problem_4();
    problem_5();
    problem_6();
    problem_7();
}

fn problem_1() {
    println!("Problem 1: Value used after move");
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(&s1);
    println!("  The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: &String) -> (&String, usize) {
    let length = s.len();
    (s, length)
}

fn problem_2() {
    println!("Problem 2: Mutable and immutable borrow conflict");
    let mut s = String::from("hello");
    let r1 = &s;
    println!("  r1: {}", r1);
    let r2 = &mut s;
    println!("  r2: {}", r2);
}

fn problem_3() {
    println!("Problem 3: Mutating through an immutable reference");
    let mut s = String::from("hello");
    add_to_string(&mut s);
    println!("  Result: {}", s);
}

fn add_to_string(s: &mut String) {
    s.push_str(", world");
}

fn problem_4() {
    println!("Problem 4: Multiple mutable borrows");
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("  r1: {}", r1);
    }
    let r2 = &mut s;
    println!("  r2: {}", r2);
}

fn problem_5() {
    println!("Problem 5: Dangling reference");
    let r = create_string();
    println!("  Got: {}", r);
}

fn create_string() -> String {
    String::from("hello")
}

fn problem_6() {
    println!("Problem 6: Ownership in loops");
    let data = String::from("Rust");
    for i in 0..3 {
        print_with_number(&data, i);
    }
}

fn print_with_number(s: &String, n: i32) {
    println!("  {}: {}", n, s);
}

fn problem_7() {
    println!("Problem 7: Lifetime extension");
    let result;
    let s = String::from("inner scope");
    result = &s;
    println!("  Result: {}", result);
}

pub fn to_uppercase_owned(s: String) -> String {
    s.to_uppercase()
}

#[allow(clippy::ptr_arg)]
pub fn string_length(s: &String) -> usize {
    s.len()
}

pub fn append_suffix(s: &mut String, suffix: &str) {
    s.push_str(suffix);
}

pub fn concat_strings(s1: &str, s2: &str) -> String {
    format!("{}{}", s1, s2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_uppercase_owned() {
        let s = String::from("hello");
        assert_eq!(to_uppercase_owned(s), "HELLO");
    }
    #[test]
    fn test_to_uppercase_owned_already_upper() {
        let s = String::from("RUST");
        assert_eq!(to_uppercase_owned(s), "RUST");
    }
    #[test]
    fn test_string_length() {
        let s = String::from("testing");
        let len = string_length(&s);
        assert_eq!(len, 7);
        assert_eq!(s, "testing");
    }
    #[test]
    fn test_string_length_empty() {
        let s = String::from("");
        assert_eq!(string_length(&s), 0);
    }
    #[test]
    fn test_append_suffix() {
        let mut s = String::from("hello");
        append_suffix(&mut s, ", world");
        assert_eq!(s, "hello, world");
    }
    #[test]
    fn test_append_suffix_empty() {
        let mut s = String::from("");
        append_suffix(&mut s, "hi");
        assert_eq!(s, "hi");
    }
    #[test]
    fn test_concat_strings() {
        assert_eq!(concat_strings("hello", " world"), "hello world");
    }
    #[test]
    fn test_concat_strings_empty() {
        assert_eq!(concat_strings("", "abc"), "abc");
        assert_eq!(concat_strings("abc", ""), "abc");
    }
}
