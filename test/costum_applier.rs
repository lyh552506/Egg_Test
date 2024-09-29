use crate::Math;
use egg::*;
use std::{str::FromStr, vec};

pub struct VecApplier;

impl Applier<Math, ()> for VecApplier {
    fn apply_one(
        &self,
        egraph: &mut EGraph<Math, ()>,
        eclass: Id,
        subst: &Subst,
        searcher_ast: Option<&PatternAst<Math>>,
        rule_name: Symbol,
    ) -> Vec<Id> {
        println!("apply rule: {}", rule_name);
        let a_var = Var::from_str("a").unwrap();
        let b_var = Var::from_str("b").unwrap();
        let c_var = Var::from_str("c").unwrap();
        
        let a_id = subst[a_var];
        let b_id = subst[b_var];
        let c_id = subst[c_var];
        let new_node = Math::VecMAC([a_id, b_id, c_id]);
        let new_eclass_id = egraph.add(new_node);

        vec![new_eclass_id]        
    }
}
