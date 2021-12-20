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
    let t13 = lambda::Term::make_var(2);
    let t14 = lambda::Term::make_var(1);
    let t15 = lambda::Term::make_app(t12, t13);
    let t16 = lambda::Term::make_app(t15, t14);
    println!("{}", t16.pretty_print());
    println!("{}", t16.subterms());
    println!("{}", t16.variables());
    println!("{}", t16.abstractions());
    println!("{}", t16.applications());
    println!("{}", t16.unique_variables());
    println!("{}", t16.crossings());

    let t17 = lambda::Term::make_var(0);
    let t18 = lambda::Term::make_var(1);
    let t19 = lambda::Term::make_var(2);

    let t20 = lambda::Term::make_app(lambda::Term::make_app(t19, t18), t17);

    println!();
    println!("{}", t20.pretty_print());
    println!("{}", t20.crossings());
}
