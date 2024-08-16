use super::*;
use std::{
    hash::Hash,
    sync::{RwLock, Weak},
};

/// Mesh vertex. 
/// Primary mesh data with back references to faces
/// which allow easy traversal of a surface.
#[derive(Default, Clone, Debug)]
pub struct Vertex {
    pub id: u32,
    point: Vector3,
    faces: Arc<RwLock<Vec<Weak<Face>>>>,
    edges: Arc<RwLock<Vec<Weak<Edge>>>>,
}

impl Vertex {
    /// F. Construct a new face from vertices, and a new vertex from coordinates.
    /// (new vertex from coordinates)
    pub fn new(point: Vector3) -> Pointer<Self> {
        Pointer::new(Self {
            id: rand::random::<u32>(),
            point,
            ..Default::default()
        })
    }

    pub fn adjacent_edges(&self) -> Vec<Pointer<Edge>> {
        let mut edges = vec![];
        for weak_edge in self.edges.read().expect("no poison").iter() {
            if let Some(arc_edge) = weak_edge.upgrade() {
                edges.push(Pointer::from_arc(arc_edge));
            }
        }
        edges
    }

    /// B. Given a vertex/face, return the adjacent faces/vertices
    /// Select faces of vertex. The faces are adjacent because they have Arc pointers this vertex.
    pub fn adjacent_faces(&self) -> Mesh {
        Mesh::from_weak_faces(&self.faces.read().expect("no poison").clone())
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

    /// Mutate the interior to include the weak face pointer. 
    pub fn push_face_back_ref(&self, face: &Weak<Face>) {
        self.faces.write().expect("no poison").push(face.clone());
    }

    /// Mutate the interior to include the weak edge pointer. 
    pub fn push_edge_back_ref(&self, edge: &Weak<Edge>) {
        self.edges.write().expect("no poison").push(edge.clone());
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