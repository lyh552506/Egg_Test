use crate::Math;
use egg::*;

pub struct MathCostFunc<'a> {
    pub egraph: &'a EGraph<Math, ()>,
}

impl CostFunction<Math> for MathCostFunc<'_> {
    type Cost = f64;
    fn cost<C>(&mut self, enode: &Math, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost,
    {
        let op_cost =match enode{
            Math::Add([_a,_b])=>0.1,
            Math::Num(_)=>0.001,
            Math::Div([_a,_b])=>2.5,
            Math::Mul([_a,_b])=>0.5,
            Math::Sqrt([_a])=>0.7,
            Math::VecAdd([_a,_b])=>0.01,
            Math::VecMul([_a,_b])=>0.05,
            Math::VecMAC([_a,_b,_c])=>0.04,
            _=>0.001,
        };
        enode.fold(op_cost, |sum, id| sum + costs(id))
    }
}
