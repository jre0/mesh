use std::{hash::Hash, sync::{RwLock, Weak}};
use super::*;

#[derive(Default, Clone, Debug)]
pub struct Vertex {
    id: u64,
    point: Vector3,
    // edges: Arc<RwLock<Vec<Weak<Face>>>>,
    faces: Arc<RwLock<Vec<Weak<Face>>>>,
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

    /// Mutate the intorior to include the weak face pointer
    pub fn push_face_back_ref(&self, face: &Weak<Face>) {
        self.faces.write().expect("no poison").push(face.clone());
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



// fn upgraded_faces(&self) -> Vec<Pointer<Face>> {
//     let mut faces = vec![];
//     for weak_face in self.faces.read().expect("no poison").iter(){
//         if let Some(arc_face) = weak_face.upgrade() {
//             let face = Pointer::from_arc(arc_face);
//             if arc_face.vertices().vertices.contains(&face) {
                
//             }
//             faces.push(Pointer::from_arc(arc_face));
//         }
//     }
//     faces
// }


// pub trait Backed<T> {
//     fn backed(&self, weak: &Weak<T>) -> Self;
// }

// impl Backed<Face> for Pointer<Vertex> {
//     fn backed(&self, weak: &Weak<Face>) -> Self {
//         let vertex = Pointer Vertex {
//             id: self.id,
//             point: self.point.clone(),
//             faces,
//         }
//         //let weak_face = face
//         vertex.0.a.faces.push(weak.clone());
//         vertex
//     }
// }