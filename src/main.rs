mod lambda;

fn main() {
    let t1 = lambda::Term::make_abs(lambda::Term::make_var(0), "x");
    let t2 = lambda::Term::make_var(0);
    let t3 = lambda::Term::make_app(t1, t2);
    let t4 = lambda::Term::make_var(0);
    let t5 = lambda::Term::make_app(t3, t4);
    println!("{}", t5.pretty_print());

    let t6 = lambda::Term::make_var(0);
    let t7 = lambda::Term::make_var(1);
    let t8 = lambda::Term::make_app(t6, t7);
    let t9 = lambda::Term::make_abs(t8, "y");
    let t10 = lambda::Term::make_abs(t9, "z");
    let t11 = lambda::Term::make_var(0);
    let t12 = lambda::Term::make_app(t10, t11);
    println!("{}", t12.pretty_print());
}
