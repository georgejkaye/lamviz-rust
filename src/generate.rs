use itertools::Itertools;
use lambda_calculus::Term;
use lambda_calculus::{abs, app};

fn descending_list(n: usize) -> Vec<usize> {
    let mut list = (1..n + 1).collect::<Vec<_>>();
    list.reverse();
    list
}

#[derive(Copy, Clone)]
pub enum Fragment {
    PURE,
    LINEAR,
    PLANAR,
}

fn generate_pure_contexts(ks: &Vec<usize>) -> Vec<(Vec<usize>, Vec<usize>)> {
    vec![(ks.clone(), ks.clone())]
}

fn generate_planar_contexts(ks: &Vec<usize>) -> Vec<(Vec<usize>, Vec<usize>)> {
    let mut contexts = vec![];
    for j in 0..(ks.len() + 1) {
        contexts.push((ks[..j].to_vec(), ks[j..].to_vec()));
    }
    contexts
}

fn generate_linear_contexts(ks: &Vec<usize>) -> Vec<(Vec<usize>, Vec<usize>)> {
    let mut contexts = vec![];
    let unique_perms = ks
        .iter()
        .permutations(ks.len())
        .unique()
        .collect::<Vec<_>>();
    // println!("{:?}", unique_perms);
    for j in 0..(ks.len() + 1) {
        for ks in &unique_perms {
            contexts.push((
                ks[..j].iter().map(|i| **i).collect(),
                ks[j..].iter().map(|i| **i).collect(),
            ));
        }
    }
    contexts
}

pub fn generate_terms(n: usize, k: usize, fragment: Fragment) -> Vec<Term> {
    fn generate_terms_helper(n: usize, ks: &mut Vec<usize>, fragment: Fragment) -> Vec<Term> {
        match n {
            0 => vec![],
            1 => match ks[..] {
                [k] => vec![Term::Var(k)],
                _ => match fragment {
                    Fragment::PURE => ks.iter().map(|x| Term::Var(*x)).collect(),
                    // If there are multiple free variables, there are no linear or planar
                    // terms of size 1 as each variable must be used exactly once
                    _ => vec![],
                },
            },
            n => {
                let mut terms = vec![];
                // Under an abstraction, , y: usizeall the free variables are incremented by 1
                ks.iter_mut().for_each(|k| *k += 1);
                // Push the newest variable
                ks.push(1);
                let abs_terms = generate_terms_helper(n - 1, ks, fragment);
                for t in abs_terms {
                    terms.push(abs(t.clone()));
                }
                ks.pop();
                ks.iter_mut().for_each(|k| *k -= 1);
                // We split the subterms between each side of the application
                for i in 1..(n - 1) {
                    // The two subterms may have different contexts
                    let contexts = match fragment {
                        Fragment::PURE => generate_pure_contexts(ks),
                        Fragment::PLANAR => generate_planar_contexts(ks),
                        Fragment::LINEAR => generate_linear_contexts(ks),
                    };

                    for (mut lhs_context, mut rhs_context) in contexts {
                        let lhs = generate_terms_helper(i, &mut lhs_context, fragment);
                        println!("{:?}", lhs);
                        let rhs = generate_terms_helper(n - 1 - i, &mut rhs_context, fragment);
                        println!("{:?}", rhs);
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
            k => descending_list(k),
        }
    };
    generate_terms_helper(n, &mut context, fragment)
}
