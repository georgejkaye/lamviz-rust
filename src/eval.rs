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
                Some(t.substitute(0, &arg.shift(1, 0)).shift(-1, 0))
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
                                        Some(t2n) => {
                                            let cont = t1n.is_abs();
                                            let t = Term::make_app(t1n, t2n);
                                            if cont {
                                                normalise_1(&t, op)
                                            } else {
                                                (Some(t), op)
                                            }
                                        }
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
    pub fn outermost_reduction(&self) -> Option<Term> {
        match self {
            Term::Var(_) => None,
            Term::Abs(t, x) => match t.outermost_reduction() {
                None => None,
                Some(t) => Some(Term::make_abs(t, x)),
            },
            Term::App(t1, t2) => match self.perform_beta_reduction() {
                Some(t) => Some(t),
                None => match t1.outermost_reduction() {
                    Some(t1) => Some(Term::remake_app_lhs(t1, t2)),
                    None => match t2.outermost_reduction() {
                        Some(t2) => Some(Term::remake_app_rhs(t1, t2)),
                        None => None,
                    },
                },
            },
        }
    }
    pub fn innermost_reduction(&self) -> Option<Term> {
        match self {
            Term::Var(_) => None,
            Term::Abs(t, x) => match t.innermost_reduction() {
                Some(t) => Some(Term::make_abs(t, x)),
                None => None,
            },
            Term::App(t1, t2) => match t1.innermost_reduction() {
                Some(t1) => Some(Term::remake_app_lhs(t1, t2)),
                None => match t2.innermost_reduction() {
                    Some(t2) => Some(Term::remake_app_rhs(t1, t2)),
                    None => self.perform_beta_reduction(),
                },
            },
        }
    }
    pub fn specific_reduction(&self, i: usize) -> Option<Term> {
        fn specific_reduction_1(t: &Term, i: usize) -> (Option<Term>, usize) {
            match t {
                Term::Var(_) => (None, i),
                Term::Abs(t, x) => match specific_reduction_1(t, i) {
                    (Some(t), i) => (Some(Term::make_abs(t, x)), i),
                    (None, i) => (None, i),
                },
                Term::App(t1, t2) => {
                    if t.is_beta_redex() && i == 0 {
                        (t.perform_beta_reduction(), 0)
                    } else {
                        let i = if t.is_beta_redex() {
                            (i as i32 - 1) as usize
                        } else {
                            i
                        };
                        match specific_reduction_1(t1, i) {
                            (Some(t1), i) => (Some(Term::remake_app_lhs(t1, t2)), i),
                            (None, i) => match specific_reduction_1(t2, i) {
                                (Some(t2), i) => (Some(Term::remake_app_rhs(t1, t2)), i),
                                (None, i) => (None, i),
                            },
                        }
                    }
                }
            }
        }
        specific_reduction_1(self, i).0
    }
}
