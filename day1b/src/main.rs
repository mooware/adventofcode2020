// day 1b:
// in a list of numbers, find the triple that adds up to 2020 and multiply it

use std::io;
use std::io::prelude::*;

const TARGET_SUM : u32 = 2020;

fn main() -> io::Result<()> {
    // reading into a single String allows using &str afterwards
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // drop the numbers into a vector and sort it
    let mut numbers = Vec::new();
    for line in input.lines() {
        let num = u32::from_str_radix(&line, 10).unwrap();
        numbers.push(num);
    }

    numbers.sort();

    for num1 in numbers.iter() {
        let complement = TARGET_SUM - num1;
        for num2 in numbers.iter() {
            if num2 > &complement {
                break;
            }
            let num3 = complement - num2;
            if numbers.binary_search(&num3).is_ok() {
                println!("{}", num1 * num2 * num3);
                return Ok(())
            }
        }
    }

    Ok(())
}