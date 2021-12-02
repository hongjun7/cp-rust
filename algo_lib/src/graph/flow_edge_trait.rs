use crate::graph::edge_trait::EdgeTrait;
use crate::graph::graph::Graph;
use crate::numbers::integer::Integer;

pub trait FlowEdgeTrait<C: Integer>: EdgeTrait {
    fn capacity(&self) -> C;
    fn capacity_mut(&mut self) -> &mut C;
    fn flow(&self, graph: &Graph<Self>) -> C;
    fn push_flow(&self, flow: C) -> (usize, usize, C) {
        (self.to(), self.reverse_id(), flow)
    }
}