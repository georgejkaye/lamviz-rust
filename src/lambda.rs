#[derive(Clone)]
pub enum Term {
    Var(usize),
    Abs(Box<Term>, String),
    App(Box<Term>, Box<Term>),
}

enum Pos {
    Root,
    Left,
    Right,
}
impl Term {
    pub fn make_var(x: usize) -> Term {
        Term::Var(x)
    }
    pub fn make_abs(t: Term, lab: &str) -> Term {
        Term::Abs(Box::new(t), lab.to_string())
    }
    pub fn make_app(t1: Term, t2: Term) -> Term {
        Term::App(Box::new(t1), Box::new(t2))
    }
    pub fn remake_app_lhs(t1: Term, t2: &Box<Term>) -> Term {
        Term::App(Box::new(t1), t2.clone())
    }
    pub fn remake_app_rhs(t1: &Box<Term>, t2: Term) -> Term {
        Term::App(t1.clone(), Box::new(t2))
    }
    pub fn is_abs(&self) -> bool {
        if let Term::Abs(_, _) = self {
            true
        } else {
            false
        }
    }
    pub fn pretty_print(&self, ctx: Option<&Vec<String>>) -> String {
        fn pretty_print_helper(
            term: &Term,
            ctx: Option<&Vec<String>>,
            frees: &mut Vec<String>,
            pos: Pos,
        ) -> String {
            match term {
                Term::Var(x) => match ctx {
                    None => format!("{}", x),
                    Some(ctx) => {
                        // Check if the variable is in the local stack
                        let i = if x < &frees.len() {
                            &frees[frees.len() - 1 - *x]
                        // Otherwise it's a global variable
                        } else {
                            &ctx[ctx.len() - 1 - (*x - frees.len())]
                        };
                        format!("{}", i)
                    }
                },
                Term::Abs(t, x) => {
                    // If we're printing labels, print it
                    // Otherwise just print a lambda
                    let lambda = match ctx {
                        None => "λ ".to_string(),
                        Some(_) => format!("λ{}. ", x),
                    };
                    // If we care about context, push the current identifier
                    match ctx {
                        None => (),
                        Some(_) => frees.push(x.to_string()),
                    }
                    // Put everything together
                    let string = format!(
                        "{}{}",
                        lambda,
                        pretty_print_helper(t, ctx, frees, Pos::Root)
                    );
                    // If we care about context, pop the last identifier
                    match ctx {
                        None => (),
                        Some(_) => {
                            frees.pop();
                        }
                    }
                    // If this isn't the root, add brackets
                    match pos {
                        Pos::Left => format!("({})", string),
                        Pos::Right => format!("({})", string),
                        Pos::Root => string,
                    }
                }
                Term::App(t1, t2) => {
                    format!(
                        "{} {}",
                        pretty_print_helper(t1, ctx, frees, Pos::Left),
                        pretty_print_helper(t2, ctx, frees, Pos::Right)
                    )
                }
            }
        }
        let mut frees = Vec::<String>::new();
        pretty_print_helper(self, ctx, &mut frees, Pos::Root)
    }
    /**
     * Generic lambda term traversal function
     */
    fn traverse<T, U>(
        &self,
        var: fn(&usize, T, U) -> (T, U),
        pre_abs: fn(&Term, T, U) -> (T, U),
        post_abs: fn(&Term, T, U) -> (T, U),
        app: fn(&Term, &Term, T, U) -> (T, U),
        init: T,
        store: U,
    ) -> T {
        fn traverse_2<T, U>(
            t: &Term,
            acc: T,
            store: U,
            var: fn(&usize, T, U) -> (T, U),
            pre_abs: fn(&Term, T, U) -> (T, U),
            post_abs: fn(&Term, T, U) -> (T, U),
            app: fn(&Term, &Term, T, U) -> (T, U),
        ) -> (T, U) {
            match t {
                Term::Var(x) => var(x, acc, store),
                Term::Abs(t, _) => {
                    let (acc, store) = pre_abs(t, acc, store);
                    let (acc, store) = traverse_2(t, acc, store, var, pre_abs, post_abs, app);
                    post_abs(t, acc, store)
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
            |_, i, _| (i + 1, 0),
            |_, i, _| (i, 0),
            |_, _, i, _| (i + 1, 0),
            0,
            0,
        )
    }
    pub fn variables(&self) -> u32 {
        self.traverse(
            |_, i, _| (i + 1, 0),
            |_, i, _| (i, 0),
            |_, i, _| (i, 0),
            |_, _, i, _| (i, 0),
            0,
            0,
        )
    }
    pub fn abstractions(&self) -> u32 {
        self.traverse(
            |_, i, _| (i, 0),
            |_, i, _| (i + 1, 0),
            |_, i, _| (i, 0),
            |_, _, i, _| (i, 0),
            0,
            0,
        )
    }
    pub fn applications(&self) -> u32 {
        self.traverse(
            |_, i, _| (i, 0),
            |_, i, _| (i, 0),
            |_, i, _| (i, 0),
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
            |_, i, s| (i, s.iter().map(|&x| x + 1).collect()),
            |_, i, s| (i, s.iter().filter(|&x| *x != 0).map(|&x| x - 1).collect()),
            |_, _, i, s| (i, s),
            0,
            Vec::<usize>::new(),
        )
    }
    fn free_variable_indices(&self) -> Vec<usize> {
        self.traverse(
            |x, mut i, _| {
                i.push(*x);
                (i, 0)
            },
            |_, i, _| (i, 0),
            |_, i, _| (i.iter().filter(|&x| *x != 0).map(|&x| x - 1).collect(), 0),
            |_, _, i, _| (i, 0),
            Vec::<usize>::new(),
            0,
        )
    }
    pub fn crossings(&self) -> u32 {
        fn inter_crossings(v1: Vec<usize>, v2: Vec<usize>) -> u32 {
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
    pub fn beta_redexes(&self) -> u32 {
        self.traverse(
            |_, i, _| (i, 0),
            |_, i, _| (i, 0),
            |_, i, _| (i, 0),
            |t1, _, i, _| {
                if let Term::Abs(_, _) = t1 {
                    (i + 1, 0)
                } else {
                    (i, 0)
                }
            },
            0,
            0,
        )
    }
    pub fn is_beta_redex(&self) -> bool {
        if let Term::App(t1, t2) = self {
            t1.is_abs()
        } else {
            false
        }
    }
    pub fn free_variables(&self) -> usize {
        self.free_variable_indices().len()
    }
    pub fn closed(&self) -> bool {
        self.free_variables() == 0
    }
    pub fn bridges(&self) -> u32 {
        self.traverse(
            |_, i, _| (i, 0),
            |_, i, _| (i, 0),
            |_, i, _| (i, 0),
            |t1, t2, i, _| {
                let b1 = if t1.closed() { 1 } else { 0 };
                let b2 = if t2.closed() { 1 } else { 0 };
                (i + b1 + b2, 0)
            },
            0,
            0,
        )
    }
}
