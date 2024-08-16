use super::*;

#[derive(Default, PartialEq, Eq, Hash, Debug)]
pub struct Edge {
    a: Pointer<Vertex>,
    b: Pointer<Vertex>,
}

impl Edge {
    /// Make edge that is common to two faces 
    pub fn from_faces(face0: &Pointer<Face>, face1: &Pointer<Face>) -> Pointer<Self> {
        let mesh0 = face0.vertices();
        let mesh1 = face1.vertices();
        let vertices: Vec<&Pointer<Vertex>> = mesh0.vertices.intersection(&mesh1.vertices).collect();
        let edge = Pointer::new(Self {
            a: vertices[0].clone(),
            b: vertices[1].clone(),
        });
        let weak_edge = Arc::downgrade(&edge.0);
        // add weak back reference of edge to vertices
        vertices[0].push_edge_back_ref(&weak_edge);
        vertices[1].push_edge_back_ref(&weak_edge);
        edge
    }
}
