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
                // same number of free variables as these are pure terms
                let rhs = generate_pure_terms(n - 1 - i, k);
                // Each possible lhs is paired up with each possible rhs
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

fn descending_list(x: usize, y: usize) -> Vec<usize> {
    (x..y).collect()
}

pub fn generate_planar_terms(n: usize, k: usize) -> Vec<Term> {
    fn generate_planar_terms_1(n: usize, ks: &mut Vec<usize>) -> Vec<Term> {
        match n {
            0 => vec![],
            1 => match ks[..] {
                [k] => vec![Term::Var(k)]
                // If there are multiple free variables, there are no linear terms
                // of size 1 as each variable must be used exactly once
                _ => vec![]
            },
            n => {
                let mut terms = vec![];
                // Under an abstraction, all the free variables are incremented by 1
                ks.iter_mut().for_each(|k| *k += 1);
                // Push the newest variable
                ks.push(1);
                let abs_terms = generate_planar_terms_1(n - 1, ks);
                for t in abs_terms {
                    terms.push(abs(t.clone()));
                }
                ks.pop();
                ks.iter_mut().for_each(|k| *k -= 1);
                // We split the subterms between each side of the application
                for i in 1..(n - 1) {
                    // We also split up the context into two halves, preserving their order
                    for j in 0..(ks.len() + 1) {
                        let lhs = generate_planar_terms_1(i, &mut ks[..j].to_vec());
                        let rhs = generate_planar_terms_1(n - 1 - i, &mut ks[j..].to_vec());
                        for t1 in &lhs {
                            for t2 in &rhs {
                                terms.push(app(t1.clone(), t2.clone()));
                            }
                        }
                    }
                }
                terms
            }
        }
    }
    let mut context = {
        match k {
            0 => vec![],
            k => descending_list(k - 1, 0),
        }
    };
    generate_planar_terms_1(n, &mut context)
}