use std::io::BufRead;
use std::{env, io};

fn main() {
    let mut args = env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        let stdin = io::stdin();
        stdin
            .lock()
            .lines()
            .for_each(|line| args.push(line.unwrap()));
    }
    let mut quotes = Vec::new();
    args.iter()
        .skip(1)
        .for_each(|quote| quotes.push(quote.split_whitespace().collect()));
    println!(
        "{}",
        markov::gen_chain_from_many(&quotes)
            .iter()
            .map(|s| String::from(**s))
            .collect::<Vec<String>>()
            .join(" ")
    );
}
