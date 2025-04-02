fn main() {
    // Example of adding two numbers using Rust's basic arithmetic operators.
    let num1 = 5;
    let num2 = 3;
    let sum = num1 + num2; // Sum is 8
}

// Function to demonstrate the use of the `or_else` method with a custom error type.
fn example_function(x: i32, y: i32) -> Result<(i32, String), &'static str> {
    if x > y {
        Ok((x, "Less than".to_string()))
    } else {
        Err("Greater than".into())
    }
}

// Example usage of the function.
let result = example_function(5, 3);
println!("{:?}", result); // Output: Some((5, "Less than".to_string()))
