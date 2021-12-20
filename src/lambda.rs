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
    /**
     * Generic lambda term traversal function
     */
    fn traverse<T, U>(
        &self,
        var: fn(&u32, T, U) -> (T, U),
        pre_abs: fn(&Term, &str, T, U) -> (T, U),
        post_abs: fn(&Term, &str, T, U) -> (T, U),
        app: fn(&Term, &Term, T, U) -> (T, U),
        init: T,
        store: U,
    ) -> T {
        fn traverse_2<T, U>(
            t: &Term,
            acc: T,
            store: U,
            var: fn(&u32, T, U) -> (T, U),
            pre_abs: fn(&Term, &str, T, U) -> (T, U),
            post_abs: fn(&Term, &str, T, U) -> (T, U),
            app: fn(&Term, &Term, T, U) -> (T, U),
        ) -> (T, U) {
            match t {
                Term::Var(x) => var(x, acc, store),
                Term::Abs(t, x) => {
                    let (acc, store) = pre_abs(t, x, acc, store);
                    let (acc, store) = traverse_2(t, acc, store, var, pre_abs, post_abs, app);
                    post_abs(t, x, acc, store)
                }
                Term::App(t1, t2) => {
                    let (acc, store) = app(t1, t2, acc, store);
                    let (acc, store) = traverse_2(t1, acc, store, var, pre_abs, post_abs, app);
                    traverse_2(t2, acc, store, var, pre_abs, post_abs, app)
                }
            }
        }
        traverse_2(&self, init, store, var, pre_abs, post_abs, app).0
    }

    pub fn subterms(&self) -> u32 {
        self.traverse(
            |_, i, _| (i + 1, 0),
            |_, _, i, _| (i + 1, 0),
            |_, _, i, _| (i, 0),
            |_, _, i, _| (i + 1, 0),
            0,
            0,
        )
    }
    pub fn variables(&self) -> u32 {
        self.traverse(
            |_, i, _| (i + 1, 0),
            |_, _, i, _| (i, 0),
            |_, _, i, _| (i, 0),
            |_, _, i, _| (i, 0),
            0,
            0,
        )
    }
    pub fn abstractions(&self) -> u32 {
        self.traverse(
            |_, i, _| (i, 0),
            |_, _, i, _| (i + 1, 0),
            |_, _, i, _| (i, 0),
            |_, _, i, _| (i, 0),
            0,
            0,
        )
    }
    pub fn applications(&self) -> u32 {
        self.traverse(
            |_, i, _| (i, 0),
            |_, _, i, _| (i, 0),
            |_, _, i, _| (i, 0),
            |_, _, i, _| (i + 1, 0),
            0,
            0,
        )
    }
    pub fn unique_variables(&self) -> u32 {
        self.traverse(
            |x, i, mut s| {
                if s.contains(&x) {
                    (i, s)
                } else {
                    s.push(*x);
                    (i + 1, s)
                }
            },
            |_, _, i, s| (i, s.iter().map(|&x| x + 1).collect()),
            |_, _, i, s| (i, s.iter().filter(|&x| *x != 0).map(|&x| x - 1).collect()),
            |_, _, i, s| (i, s),
            0,
            Vec::<u32>::new(),
        )
    }
    fn free_variable_indices(&self) -> Vec<u32> {
        self.traverse(
            |x, mut i, _| {
                i.push(*x);
                (i, 0)
            },
            |_, _, i, _| (i, 0),
            |_, _, i, _| (i.iter().filter(|&x| *x != 0).map(|&x| x - 1).collect(), 0),
            |_, _, i, _| (i, 0),
            Vec::<u32>::new(),
            0,
        )
    }
    pub fn crossings(&self) -> u32 {
        fn inter_crossings(v1: Vec<u32>, v2: Vec<u32>) -> u32 {
            let mut counter = 0;
            for i in v1.iter() {
                for j in v2.iter() {
                    if i < j {
                        counter += 1;
                    }
                }
            }
            counter
        }
        match &self {
            Term::Var(_) => 0,
            Term::Abs(t, _) => t.crossings(),
            Term::App(t1, t2) => {
                let c1 = t1.crossings();
                let c2 = t2.crossings();
                let v1 = t1.free_variable_indices();
                let v2 = t2.free_variable_indices();
                c1 + c2 + inter_crossings(v1, v2)
            }
        }
    }
}
