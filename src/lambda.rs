pub enum Term {
    Var(u32),
    Abs(Box<Term>, String),
    App(Box<Term>, Box<Term>),
}

enum Pos {
    Root,
    Left,
    Right,
}

impl Term {
    pub fn make_var(x: u32) -> Term {
        Term::Var(x)
    }
    pub fn make_abs(t: Term, x: &str) -> Term {
        Term::Abs(Box::new(t), String::from(x))
    }
    pub fn make_app(t1: Term, t2: Term) -> Term {
        Term::App(Box::new(t1), Box::new(t2))
    }

    pub fn pretty_print(&self) -> String {
        fn pretty_print_helper(term: &Term, pos: Pos) -> String {
            match term {
                Term::Var(x) => format!("{}", x),
                Term::Abs(t, _) => {
                    let string = format!("λ {}", pretty_print_helper(t, Pos::Root));
                    match pos {
                        Pos::Left => format!("({})", string),
                        Pos::Right => format!("({})", string),
                        Pos::Root => string,
                    }
                }
                Term::App(t1, t2) => {
                    format!(
                        "{} {}",
                        pretty_print_helper(t1, Pos::Left),
                        pretty_print_helper(t2, Pos::Right)
                    )
                }
            }
        }
        pretty_print_helper(self, Pos::Root)
    }
}
