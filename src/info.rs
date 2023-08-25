
use std::fmt::Display;

fn info<T: Display>(text: &T) {
    println!("{}", text);
}