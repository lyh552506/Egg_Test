use std::str::FromStr;

use egg::{rewrite as rw, *};

fn main() {
    // let my_expression: RecExpr<SymbolLang> = "(foo a b)".parse().unwrap();
    // println!("this is my expression {}", my_expression);
    // let my_node = SymbolLang::new("Fuck", vec![]);

    // // we can do the same thing with an EGraph
    // let mut egraph: EGraph<SymbolLang, ()> = Default::default();
    // let a = egraph.add(SymbolLang::leaf("a"));
    // let b = egraph.add(SymbolLang::leaf("b"));
    // let foo = egraph.add(SymbolLang::new("foo", vec![a, b]));

    // egraph.rebuild();
    let rules: &[Rewrite<SymbolLang, ()>] = &[
        rw!("commute-add"; "(+ ?x ?y)" => "(+ ?y ?x)"),
        rw!("commute-mul"; "(* ?x ?y)" => "(* ?y ?x)"),
        rw!("add-0"; "(+ ?x 0)" => "?x"),
        rw!("mul-0"; "(* ?x 0)" => "0"),
        rw!("mul-1"; "(* ?x 1)" => "?x"),
        rw!("div-one"; "?x" => "(/ ?x 1)"),
        rw!("unsafe-invert-division"; "(/ ?a ?b)" => "(/ 1 (/ ?b ?a))"),
        rw!("simplify-frac"; "(/ ?a (/ ?b ?c))" => "(/ (* ?a ?c) (* (/ ?b ?c) ?c))"),
        rw!("cancel-denominator"; "(* (/ ?a ?b) ?b)" => "?a"),
    ];
    let start = "0".parse().unwrap();
    let end = "1".parse().unwrap();
    let mut runner = Runner::default()
        .with_explanations_enabled()
        .with_expr(&start)
        .run(rules);
    let extractor = Extractor::new(&runner.egraph, AstSize);
    let (best_cost, best_exp) = extractor.find_best(runner.roots[0]);
    println!("Best cost is {best_cost},{best_exp}");
    println!(
        "{}",
        runner.explain_equivalence(&start, &end).get_flat_string()
    );
}
