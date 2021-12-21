use crate::lambda::Term;

const MAX_OPS: usize = 10000;

impl Term {
    fn shift(&self, d: i32, c: usize) -> Term {
        match self {
            Term::Var(x) => {
                if x < &c {
                    Term::make_var(*x)
                } else {
                    let x1 = *x as i32 + d;
                    Term::make_var(x1 as usize)
                }
            }
            Term::Abs(t, x) => Term::make_abs(t.shift(d, c + 1), x),
            Term::App(t1, t2) => Term::make_app(t1.shift(d, c), t2.shift(d, c)),
        }
    }

    fn substitute(&self, j: usize, s: &Term) -> Term {
        match self {
            Term::Var(x) => {
                if *x == j {
                    s.clone()
                } else {
                    self.clone()
                }
            }
            Term::Abs(t, x) => Term::make_abs(t.substitute(j + 1, &s.shift(1, 0)), x),
            Term::App(t1, t2) => Term::make_app(t1.substitute(j, s), t2.substitute(j, s)),
        }
    }
    fn perform_beta_reduction(&self) -> Option<Term> {
        fn perform_beta(abs: &Term, arg: &Term) -> Option<Term> {
            if let Term::Abs(t, _) = abs {
                println!("before shift subs {}", t.pretty_print(None));
                println!("subbing {}", &arg.shift(1, 0).pretty_print(None));
                let x = t.substitute(0, &arg.shift(1, 0));
                println!("before shift {}", x.pretty_print(None));
                Some(x.shift(-1, 0))
            } else {
                None
            }
        }
        if let Term::App(abs, arg) = self {
            perform_beta(abs, arg)
        } else {
            None
        }
    }
    pub fn normalise(&self) -> Option<Term> {
        fn normalise_1(t: &Term, op: usize) -> (Option<Term>, usize) {
            let op = op + 1;
            if op > MAX_OPS {
                (None, op)
            } else {
                match t {
                    Term::Var(_) => (Some(t.clone()), op),
                    Term::Abs(t, x) => {
                        let (t, op) = normalise_1(t, op);
                        let t = match t {
                            None => None,
                            Some(t) => Some(Term::make_abs(t, x)),
                        };
                        (t, op)
                    }
                    Term::App(t1, t2) => match t.perform_beta_reduction() {
                        Some(t) => normalise_1(&t, op),
                        None => {
                            let (t1n, op) = normalise_1(t1, op);
                            match t1n {
                                None => (None, op),
                                Some(t1n) => {
                                    let (t2n, op) = normalise_1(t2, op);
                                    match t2n {
                                        None => (None, op),
                                        Some(t2n) => (Some(Term::make_app(t1n, t2n)), op),
                                    }
                                }
                            }
                        }
                    },
                }
            }
        }
        normalise_1(self, 0).0
    }
}
