use core::fmt;
use cost::MathCostFunc;
use egg::{rewrite as rw, *};
use lexpr::*;
mod cost;
mod costum_applier;
mod costum_searcher;

//define my own language
define_language! {
    enum Math {
        Num(i32),
        "+"=Add([Id; 2]),
        "-"=Sub([Id; 2]),
        "*"=Mul([Id; 2]),
        "-"=Neg([Id;1]),
        "/"=Div([Id; 2]),
        "or"=Or([Id;2]),
        "&&"=And([Id;2]),
        "<"=Lt([Id;2]),
        ">"=Gt([Id;2]),
        "sqrt"=Sqrt([Id;1]),
        //vec
        "List"=List(Box<[Id]>),
        "Vec"=Vec(Box<[Id]>),
        "VecAdd"=VecAdd([Id;2]),
        "VecSub"=VecSub([Id;2]),
        "VecMul"=VecMul([Id;2]),
        "VecDiv"=VecDiv([Id;2]),
        "VecMac"=VecMAC([Id;3]),
        "ConCat"=ConCat([Id;2]),
        "Get"=Get([Id;2]),

        Symbol(Symbol),
    }
}

// This returns a function that implements Condition
fn is_not_zero(var: &'static str) -> impl Fn(&mut EGraph<Math, ()>, Id, &Subst) -> bool {
    let var = var.parse().unwrap();
    let zero = Math::Num(0);
    move |egraph, _, subst| !egraph[subst[var]].nodes.contains(&zero)
}

fn main() {
    //define my own rules
    let rule: &[Rewrite<Math, ()>] = &[
        rw!("add-0";"(+ ?x 0)"=>"?x"),
        rw!("mul-0";"(* ?x 0)"=>"0"),
        rw!("add-sub-same";"(-(+ ?x ?y)?y)"=>"?x"),
        rw!("mul-1";"(* ?x 1)"=>"?x"),
        rw!("commute-addx";"(+ ?a ?b)"=>"(+ ?b ?a)"),
        rw!("reassociate";"(*(+ ?x ?y) ?z)" => "(+ (* ?x ?z) (* ?y ?z))"),
        rw!("div-reverse";"(/ ?x ?y)"=>"(/ 1 (/ ?y ?x))" if is_not_zero("?x")),
        rw!("div-mul";"(* (/ ?z (/ ?x ?y)) (/ ?x ?y))"=>"?z"),
        // rw!("get-trans";"(Get ?a Num(i))"),
        rw!("vec-add";"(Vec (+ ?a ?b) (+ ?c ?d))"=>"(VecAdd (Vec ?a ?c) (Vec ?b ?d))"),
        rw!("to-vecmac";"(VecAdd ?a (VecMul ?b ?c))"=>"(VecMac ?a ?b ?c)"),
        // rw!("0-add-0"; "(Vec (Num 0))" => "(VecAdd (Num 0) (Num 0))"),
        rw!("solve-AC-matching";{costum_searcher::VecSearcher}=>{costum_applier::VecApplier}),
    ];

    // let test_exp = "(*(/ 3 2) (/ 2 3))".parse().unwrap();
    let to_vecadd = "(Concat(Vec (+ (Get a 0) (Get b 0))
                                                  (+ (Get a 1) (Get b 1)))
                                  (Vec (+ (Get a 2) (Get b 2))
                                                  (+ (Get a 3) (Get b 3))))";
    let to_vecadd = "(Vec (+ a b) (+ c d))";
    let to_vecmac = "(VecAdd a (VecMul b c))";
    let costom_macth = "(Vec (+ a0 (* b0 c0))
                              (+ a1 (* b1 c1))
                              (+ a2 (* b2 c2))
                              (+ (* b3 c3) a3))";
    let target_exp: RecExpr<Math> = costom_macth.parse().unwrap();
    let mut runner = Runner::default()
        .with_explanations_enabled()
        .with_expr(&target_exp)
        .with_iter_limit(10)
        .run(rule);
    let mut cost_fn = cost::MathCostFunc {
        egraph: &runner.egraph,
    };
    //print each iteration and cost
    for (i, iteration) in runner.iterations.iter().enumerate() {
        let extractor = Extractor::new(
            &runner.egraph,
            MathCostFunc {
                egraph: &runner.egraph,
            },
        );
        let (best_cost, best_expr) = extractor.find_best(runner.roots[0]);
        println!(
            "Iteration {}: Cost: {}, Expression: {}",
            i, best_cost, best_expr
        );
    }
    let extract = Extractor::new(&runner.egraph, cost_fn);
    let res = extract.find_best(runner.roots[0]);
    #[cfg(debug_assertions)]
    println!(
        "Stopped after {} iterations, reason: {:?}",
        runner.iterations.len(),
        runner.stop_reason
    );
    println!("Egg Graph:{}", res.1);
}
