use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let mut numbers = Vec::new();

    for i in 0..10 {
        numbers.push(i);
    }

    println!("{:?}", numbers);
}
