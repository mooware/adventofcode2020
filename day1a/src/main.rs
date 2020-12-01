// day 1a:
// in a list of numbers, find the pair that adds up to 2020 and multiply it

use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

const TARGET_SUM : u32 = 2020;

fn main() -> io::Result<()> {
    // reading into a single String allows using &str afterwards
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // drop the numbers into a hash
    let mut numbers = HashSet::new();
    for line in input.lines() {
        let num = u32::from_str_radix(&line, 10).unwrap();
        numbers.insert(num);
    }

    for num in numbers.iter() {
        let complement = TARGET_SUM - num;
        if numbers.contains(&complement) {
            let result = num * complement;
            println!("{}", result);
            break;
        }
    }

    Ok(())
}