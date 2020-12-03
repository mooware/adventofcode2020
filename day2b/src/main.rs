// day 2b:
// in a list of "passwords", check that a letter appears exactly once
// in one of two positions

use std::io;
use std::io::prelude::*;

const SPLIT_CHARS : &str = "- :";

fn main() -> io::Result<()> {
    // reading into a single String allows using &str afterwards
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut validcount = 0;

    for line in input.lines() {
        // string format: "1-5 a: foobar"
        let mut parts = line.splitn(5, |c| SPLIT_CHARS.contains(c));
        let index1 = parts.next().unwrap().parse::<usize>().unwrap();
        let index2 = parts.next().unwrap().parse::<usize>().unwrap();
        let checkchar = parts.next().unwrap().chars().next().unwrap();
        parts.next();
        let password = parts.next().unwrap();

        let mut count = 0;
        for (i, c) in password.char_indices() {
            if c == checkchar && (i + 1 == index1 || i + 1 == index2) {
                count += 1;
            }
        }

        if count == 1 {
            validcount += 1;
        }
    }

    println!("{}", validcount);

    Ok(())
}