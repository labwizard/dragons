#![feature(array_windows)]

mod curve;

use ::clap::Parser;
use curve::Curve;

#[derive(Debug, Parser)]
struct Arguments {
    pub iterations: usize,
    #[clap(default_value_t = 2)]
    pub expand_factor: i32
}

fn main() {
    let arguments = Arguments::parse();

    let dragon = curve::dragon(arguments.iterations)
        .expand(arguments.expand_factor);
    let bottom_right = dragon.bottom_right();
    for y in 0..=(bottom_right.1) {
        for x in 0..=(bottom_right.0) {
            match dragon.contains(&(x, y)) {
                true  => print!("\u{00b7}"),
                false => print!(" ")
            }
        }
        println!();
    }
}
