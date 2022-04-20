mod generate;

fn main() {
    let terms = generate::generate_planar_terms(5, 0);
    for t in terms {
        println!("{}", t);
    }
}
