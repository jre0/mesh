use super::*;

#[derive(Default, PartialEq, Eq, Debug)]
pub struct Edge {
    id: u64,
    a: Pointer<Vertex>,
    b: Pointer<Vertex>,
}

impl Edge {
    /// Make edge that is common to two faces, if there is such an edge
    pub fn from_faces(face0: &Pointer<Face>, face1: &Pointer<Face>) -> Result<Pointer<Self>, Error> {
        let mesh0 = face0.vertices();
        let mesh1 = face1.vertices();
        let vertices: Vec<&Pointer<Vertex>> = mesh0.vertices.intersection(&mesh1.vertices).collect();
        let a = {*vertices.first().ok_or("no vertices")?}.clone();
        let b = {*vertices.last().ok_or("not enough vertices")?}.clone();
        let edge = Pointer::new(Self {
            id: a.id as u64 + b.id as u64,
            a: a.clone(),
            b: b.clone(),
        });
        let weak_edge = Arc::downgrade(&edge.0);
        // add weak back reference of edge to vertices
        a.push_edge_back_ref(&weak_edge);
        b.push_edge_back_ref(&weak_edge);
        Ok(edge)
    }

    /// Select vertices as new mesh
    pub fn vertices(&self) -> Mesh {
        let mut mesh = Mesh::default();
        mesh.vertices
            .extend([self.a.clone(), self.b.clone()]);
        mesh
    }
}

impl Hash for Edge {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        // (self.a.id as u64 + self.b.id as u64).hash(state);
    }
}
