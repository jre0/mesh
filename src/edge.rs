use super::*;

#[derive(Default, PartialEq, Eq, Hash, Debug)]
pub struct Edge {
    // the sume of vertex indices to uniquely identify the edge independent of direction
    pub id_sum: u64,
    a: Pointer<Vertex>,
    b: Pointer<Vertex>,
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

    /// Select vertices as new mesh
    pub fn vertices(&self) -> Mesh {
        let mut mesh = Mesh::default();
        mesh.vertices
            .extend([self.a.clone(), self.b.clone()]);
        mesh
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

// id: a.id as u64 + b.id as u64,

// impl Hash for Edge {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.id.hash(state);
//         // (self.a.id as u64 + self.b.id as u64).hash(state);
//     }
// }

// let mesh0 = face0.vertices();
//         let mesh1 = face1.vertices();
//         let vertices: Vec<&Pointer<Vertex>> = mesh0.vertices.intersection(&mesh1.vertices).collect();
//         let a = {*vertices.first().ok_or("no vertices")?}.clone();
//         let b = {*vertices.last().ok_or("not enough vertices")?}.clone();
//         let edge = Pointer::new(Self {
//             id: a.id as u64 + b.id as u64,
//             a: a.clone(),
//             b: b.clone(),
//         });
//         let weak_edge = Arc::downgrade(&edge.0);
//         // add weak back reference of edge to vertices
//         a.push_edge_back_ref(&weak_edge);
//         b.push_edge_back_ref(&weak_edge);
//         Ok(edge)
