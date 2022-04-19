mod generate;

fn main() {
    let terms = generate::generate_pure_terms(4, 0);
    for t in terms {
        println!("{0}", t);
    }
}
