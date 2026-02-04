fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    println!("Rust Project Template");
    println!("{}", greet("World"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("Alice"), "Hello, Alice!");
        assert_eq!(greet("Bob"), "Hello, Bob!");
    }

    #[test]
    fn test_greet_empty() {
        assert_eq!(greet(""), "Hello, !");
    }
}
