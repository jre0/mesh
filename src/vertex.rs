use std::{collections::HashSet, hash::Hash, sync::Weak};
use super::*;

#[derive(Default)]
pub struct Vertex {
    id: u64,
    point: Vector3,
    edges: HashSet<Weak<Edge>>,
    faces: HashSet<Weak<Face>>,
}

impl Vertex {

    pub fn new(point: Vector3) -> Self {
        Self { 
            id: rand::random::<u64>(),
            point,
            ..Default::default()
        }
    }

    /// B. Given a vertex/face, return the adjacent faces/vertices
    /// Select faces of vertex. The faces are adjacent because they have Arc pointers this vertex.
    pub fn adjacent_faces(&self) -> Mesh {
        Mesh::from_weak_faces(&self.faces)
    }

    /// B. Given a vertex/face, return the adjacent faces/vertices
    /// Select adjacent vertices of vertex including this vertex
    pub fn adjacent_vertices(&self) -> Mesh {
        self.adjacent_faces().vertices_only()
    }

    /// B. Given a vertex/face, return the adjacent faces/vertices
    /// Select adjacent vertices AND faces
    pub fn adjacent_vertices_and_faces(&self) -> Mesh {
        self.adjacent_faces().with_face_vertices()
    }
}

impl Eq for Vertex {}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Vertex {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}