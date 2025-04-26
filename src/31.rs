fn main() {
    // Example Rust code snippet
    let mut x = 0;
    while true {
        println!("Enter a number: ");
        match std::io::stdin().read_line(&mut x) {
            Ok(_) => break,
            Err(e) => eprintln!("{}", e),
        };
    }
}
