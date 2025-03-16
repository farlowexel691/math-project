use std::{thread, time};
fn main() {
    let x = 5; // Change this to any number you want!
    thread::spawn(move || {
        let mut i = 0;
        while i < x {
            println!("Hello, world!");
            i += 1;
            time::sleep(time::Duration::from_secs(1));
        }
    });
}
