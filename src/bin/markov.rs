use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let mut sentences = Vec::new();

    args.iter().skip(1).for_each(|sentence| sentences.push(sentence.split(' ').collect()));

    println!("{}", markov::gen_chain_from_many(&sentences).iter().map(|s| String::from(**s)).collect::<Vec<String>>().join(" "));
}
