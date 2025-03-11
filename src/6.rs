// This is a comment

fn main() {
    let x = 5; // Assign 5 to variable x
    let y = 10; // Assign 10 to variable y
    println!("The sum of {} and {} is {}", x, y, add(x, y));
}

fn add(a: i32, b: i32) -> i32 {
    a + b // Returns the sum of two numbers
}
