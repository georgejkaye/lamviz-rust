use lambda_calculus::Term;
use lambda_calculus::{abs, app};

pub fn generate_pure_terms(n: usize, k: usize) -> Vec<Term> {
    match n {
        0 => vec![],
        1 => (1..(k + 1)).map(|x| Term::Var(x)).collect(),
        n => {
            let mut terms = vec![];
            // The term inside an abstraction has one fewer subterm
            // but one more free variable (the abstracted variable)
            let abs_terms = generate_pure_terms(n - 1, k + 1);
            for t in abs_terms {
                terms.push(abs(t.clone()));
            }
            for i in 1..(n - 1) {
                // Pick a number of subterms and free variables
                let lhs = generate_pure_terms(i, k);
                // The rhs has the complementary number of subterms, but the
                // same number of free variables as these are free terms
                let rhs = generate_pure_terms(n - 1 - i, k);
                /*
                 * Each possible lhs is paired up with each possible rhs
                 */
                for t1 in &lhs {
                    for t2 in &rhs {
                        terms.push(app(t1.clone(), t2.clone()));
                    }
                }
            }
            terms
        }
    }
}
