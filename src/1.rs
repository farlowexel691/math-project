fn main() {
    let mut x = 0;
    let y = 0;

    if 5 == 3 {
        println!("Hello world!");
        x = 1 + 2;
    } else {
        println!("Goodbye world!");
    }

    if true {
        println!("This is always printed.");
    }

    while y < 5 {
        y += 1;
    }

    for i in 0..=4 {
        println!("{}", i);
    }
}