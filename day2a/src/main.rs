// day 2a:
// in a list of "passwords", check that a letter only appears a certain number of times

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
        let mincount = parts.next().unwrap().parse::<usize>().unwrap();
        let maxcount = parts.next().unwrap().parse::<usize>().unwrap();
        let checkchar = parts.next().unwrap().chars().next().unwrap();
        parts.next();
        let password = parts.next().unwrap();

        // I'm sure there's a smart way to put this into one expression,
        // but the language server isn't giving me any completions...
        let mut count = 0;
        for c in password.chars() {
            if c == checkchar {
                count += 1;
            }
        }

        if count >= mincount && count <= maxcount {
            validcount += 1;
        }
    }

    println!("{}", validcount);

    Ok(())
}