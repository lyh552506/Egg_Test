use crate::Math;
use egg::*;
use std::{str::FromStr, vec};

pub struct VecSearcher;

impl Searcher<Math, ()> for VecSearcher {
    fn search_eclass_with_limit(
        &self,
        egraph: &EGraph<Math, ()>,
        eclass_id: Id,
        limit: usize,
    ) -> Option<SearchMatches<'_, Math>> {
        let eclass = &egraph[eclass_id];
        for node in &eclass.nodes {
            if let Math::Vec(elements) = node {
                if elements.len() == 4 {
                    //iterate all the nodes in vec
                    for &ele in elements.iter() {
                        let inside_ele = &egraph[ele];
                        let valid = inside_ele.nodes.iter().any(|nodes| match nodes {
                            Math::Mul([_a, _b]) => true,
                            Math::Add([a, b]) => {
                                match (&egraph[*a].nodes[0], &egraph[*b].nodes[0]) {
                                    (Math::Mul([_b_, _c_]), _a_) => {
                                        return true;
                                    }
                                    _ => false,
                                }
                            }
                            Math::Num(0) => true,
                            _ => false,
                        });
                        if valid {
                            return Some(SearchMatches {
                                eclass: eclass_id,
                                substs: vec![],
                                ast: None,
                            });
                        }
                    }
                }
            }
        }
        None
    }

    fn vars(&self) -> Vec<Var> {
        vec![
            Var::from_str("?a").unwrap(),
            Var::from_str("?b").unwrap(),
            Var::from_str("?c").unwrap(),
        ]
    }
}
