use super::*;

/// Edge that points to two vertices.
/// The order of A and B matters.
#[derive(Default, PartialEq, Eq, Hash, Debug)]
pub struct Edge {
    // The sum of vertex indices to uniquely identify the edge independent of direction
    pub id_sum: u64,
    pub a: Pointer<Vertex>,
    pub b: Pointer<Vertex>,
}

impl Edge {
    /// Make edge from two vertices
    pub fn new(a: &Pointer<Vertex>, b: &Pointer<Vertex>) -> Pointer<Self> {
        let edge = Pointer::new(Self {
            id_sum: a.id as u64 + b.id as u64,
            a: a.clone(),
            b: b.clone(),
        });
        let weak_edge = Arc::downgrade(&edge.0);
        // add weak back reference of edge to vertices
        a.push_edge_back_ref(&weak_edge);
        b.push_edge_back_ref(&weak_edge);
        edge
    }
}

impl Pointer<Edge> {
    /// Get the next edge connected to B of this edge
    pub fn next_edge(&self) -> Option<Self> {
        let edges = self.b.adjacent_edges();
        if edges.len() > 1 {
            if edges[0] != *self {
                return Some(edges[0].clone());
            } else if edges[1] != *self {
                return Some(edges[1].clone());
            }
        }
        None
    }
}
