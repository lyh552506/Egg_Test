use crate::Math;
use egg::*;

pub struct MathCostFunc<'a> {
    pub egraph: &'a EGraph<Math, ()>,
}

impl CostFunction<Math> for MathCostFunc<'_> {
    type Cost = f64;
    fn cost<C>(&mut self, enode: &Math, costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost,
    {
        match enode{
            Math::Add([a,b])=>0.1,
            Math::Num(i32)=>0.001,
            Math::Div([a,b])=>2.5,
            Math::Mul([a,b])=>0.5,
            Math::Sqrt([a])=>0.7,
            Math::VecAdd([a,b])=>0.1,
            Math::VecMul([a,b])=>0.5,
            Math::VecMAC([a,b,c])=>0.4,
            _=>1.0,
        }
    }
}
