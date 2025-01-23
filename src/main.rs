
// src/lib.rs

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5); // Test case 1
        assert_eq!(add(-1, 1), 0); // Test case 2
        assert_eq!(add(0, 0), 0); // Test case 3
        assert_eq!(add(1, 1), 3); // Test case 3
    }
}



fn main() {
    println!("Hello, world!");
}
