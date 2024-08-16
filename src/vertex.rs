use std::{collections::HashSet, hash::Hash, sync::Weak};
use super::*;

#[derive(Default, Clone)]
pub struct Vertex {
    id: u64,
    point: Vector3,
    edges: HashSet<Weak<Edge>>,
    faces: HashSet<Weak<Face>>,
}

impl Vertex {
    /// F. Construct a new face from vertices, and a new vertex from coordinates.
    /// (new vertex from coordinates)
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
        self.adjacent_faces().face_vertices()
    }

    /// B. Given a vertex/face, return the adjacent faces/vertices
    /// Select adjacent faces AND vertices
    pub fn adjacent_vertices_and_faces(&self) -> Mesh {
        self.adjacent_faces().with_face_vertices()
    }

    /// D. Return the coordinates of a given vertex.
    pub fn point(&self) -> &Vector3 {
        &self.point
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