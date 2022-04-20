mod generate;

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n = input.trim().parse().unwrap();

    let terms = generate::generate_terms(n, 3, generate::Fragment::LINEAR);
    println!("There are {} terms", terms.len());
    for t in terms {
        println!("{}", t);
    }
}
