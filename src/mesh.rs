use std::{collections::HashSet, fs};
use regex::{Captures, Regex};
use super::*;

mod read;
mod write;

/// Mesh can be the primary data or selection of a subset of the data.
#[derive(Default, Clone)]
pub struct Mesh {
    pub vertices: HashSet<Pointer<Vertex>>,
    pub faces: HashSet<Pointer<Face>>,
    pub edges: HashSet<Pointer<Edge>>,
}

impl Mesh {

    /// C. Return all the vertices or faces 
    /// (Vertices)
    pub fn vertex_list(self) -> Vec<Pointer<Vertex>> {
        self.vertices.into_iter().collect()
    }

    /// C. Return all the vertices or faces 
    /// (Faces)
    pub fn face_list(self) -> Vec<Pointer<Face>> {
        self.faces.into_iter().collect()
    }

    /// E. Delete a vertex or face, with optional flag to delete all connected faces (if a vertex).
    /// (Vertex)
    pub fn delete_vertex(&mut self, vertex: &Pointer<Vertex>, delete_faces: bool) {
        if delete_faces {
            for face in vertex.adjacent_faces().face_list() {
                self.faces.remove(&face);
            }
        }
        self.vertices.remove(vertex);
    }

    /// E. Delete a vertex or face, with optional flag to delete all connected faces (if a vertex).
    /// (Face)
    pub fn delete_face(&mut self, face: &Pointer<Face>) {
        self.faces.remove(face);
    }

    /// 2. Write a function that returns whether all faces are consistently oriented.
    pub fn consistent_orientation(&self) -> bool {
        for face in &self.faces {
            if face.adjacent_is_flipped() {
                return false;
            }
        }
        true 
    }

    /// Mesh from weak face pointers 
    pub fn from_weak_faces(faces: &Vec<Weak<Face>>) -> Self {
        let mut mesh = Self::default();
        for weak_face in faces {
            if let Some(arc_face) = weak_face.upgrade() {
                mesh.faces.insert(Pointer::from_arc(arc_face));
            }
        }
        mesh
    }

    /// Select all vertices of faces of this mesh
    pub fn face_vertices(&self) -> Self {
        let mut mesh = Self::default();
        for face in &self.faces {
            mesh.extend(face.vertices());
        }
        mesh
    }

    /// Include face vertices directly in new mesh
    pub fn with_face_vertices(&self) -> Self {
        let mut mesh = self.clone();
        mesh.extend(self.face_vertices());
        mesh
    }

    /// Extend vertices, faces, and edges from other
    pub fn extend(&mut self, other: Self) {
        self.vertices.extend(other.vertices);
        self.faces.extend(other.faces);
        self.edges.extend(other.edges);
    }
}



// mesh.vertices.extend(face.vertices().map(|x| x.clone()))

// /// Extend vertices, faces, and edges from other
// pub fn extend(&mut self, other: Self) {
//     self.vertices.extend(other.vertices);
//     self.faces.extend(other.faces);
//     self.edges.extend(other.edges);
// }



// /// Mesh from join of self and other
// pub fn join(&self, other: Self) -> Self {
//     let mut mesh = self.clone();
//     mesh.vertices.extend(other.vertices);
//     mesh.faces.extend(other.faces);
//     mesh.edges.extend(other.edges);
//     mesh
// }




// pub fn extend_vertices(&mut self, vertices: Vec<ArcPlus<Vertex>>) - {
//     self.vertices.extend(vertices);
// }

// pub fn from_vertices(vertices: Vec<ArcPlus<Vertex>>) -> Self {
//     let mut mesh = Mesh::default();
//     mesh.vertices.extend(vertices);
//     mesh
// }


// /// Mesh
// #[derive(Default)]
// pub struct Mesh {
//     pub vertices: Vec<f64>,
//     pub faces: Vec<usize>,
// }

// impl Mesh {
//     pub fn new() -> Self {
//         Self::default()
//     }

//     /// A. Read/Write located in src/mesh

//     // TODO: do the same by face index
//     /// B. Given a vertex/face, return the adjacent faces/vertices
//     /// Only selecting the faces that use the given vertex index by design
//     pub fn select_adjacent_by_vertex_index(&self, target_index: usize) -> Selection {
//         let mut vertices = HashSet::new();
//         let mut faces = HashSet::new();
//         for (face_index, face) in self.faces.windows(3).step_by(3).enumerate() {
//             if face.iter().find(|vertex_index| **vertex_index == target_index).is_some() {
//                 faces.insert(face_index);
//                 vertices.extend(face);
//             }
//         }
//         Selection {vertices, faces}
//     }

//     /// C. Return all the vertices or faces.
//     pub fn select_all(&self) -> Selection {
//         let vertices = (0..self.vertices.len()/3).collect();
//         let faces = (0..self.faces.len()/3).collect();
//         Selection {vertices, faces}
//     }

//     /// D. Return the coordinates of a given vertex.
//     pub fn vertex_coordinates(&self, index: usize) -> Vector3 {
//         let i = index * 3;
//         Vector3::new(&self.vertices[i..i+3])
//     }

//     /// E. Delete a vertex or face, with optional flag to delete all connected faces (if a vertex)
//     pub fn delete_vertex(&mut self, target_index: usize, delete_faces: bool) {
//         let adjacent = self.select_adjacent_by_vertex_index(target_index);
//         let i = target_index * 3;
//         let slice_0 = &self.vertices[..i];
//         let slice_1 = &self.vertices.clone()[i+3..];
//         self.vertices = slice_0.to_vec();
//         self.vertices.extend(slice_1);
//         // Reduce vertex indices in faces after the deleted vertex
//         for vertex_index in &mut self.faces {
//             if *vertex_index > target_index {
//                 *vertex_index -= 1;
//             }
//         }
//         if delete_faces {
//             for index in adjacent.faces {
//                 self.delete_face(index);
//             }
//         }
//     }

//     /// Delete a face
//     pub fn delete_face(&mut self, target_index: usize) {
//         let i = target_index * 3;
//         let slice_0 = &self.faces[..i];
//         let slice_1 = &self.faces.clone()[i+3..];
//         self.faces = slice_0.to_vec();
//         self.faces.extend(slice_1);
//     }

//     /// F. Construct a new face from vertices, ...
//     pub fn add_face(&mut self, vertice_indices: [usize; 3]) {
//         self.faces.extend(vertice_indices);
//     }
//     /// F. ... and a new vertex from coordinates.
//     pub fn add_vertex(&mut self, coordinates: Vector3) {
//         self.vertices.extend(coordinates.as_vec());
//     }

//     /// G. Flip the sense of a face
//     pub fn flip_face(&mut self, index: usize) {
//         let i = index * 3;
//         let first_vertex_index = self.faces[i];
//         self.faces[i] = self.faces[i + 1];
//         self.faces[i + 1] = first_vertex_index;
//     }

//     /// 2. Write a function that returns whether all faces are consistently oriented.
//     /// Making the assumption that all faces share vertices so they are connected together
//     pub fn consistent_orientation(&self) -> bool {
//         for (index_a, a) in self.faces.windows(3).step_by(3).enumerate() {
//             for b in self.faces.windows(3).step_by(3).skip(index_a + 1) {
//                 // imagine two paper cutout triangles on a table. These vertex pairs cannot be 
//                 // shared without flipping or overlapping the triangles
//                 if (a[0] == b[0] && a[1] == b[1]) 
//                     || (a[0] == b[0] && a[2] == b[2]) 
//                     || (a[1] == b[1] && a[2] == b[2]) 
//                     // same as previous check but r triangle "rotated" 120
//                     || (a[0] == b[1] && a[1] == b[2]) 
//                     || (a[0] == b[1] && a[2] == b[0]) 
//                     || (a[1] == b[2] && a[2] == b[0]) 
//                     // same as previous check but r triangle "rotated" another 120
//                     || (a[0] == b[2] && a[1] == b[0]) 
//                     || (a[0] == b[2] && a[2] == b[1]) 
//                     || (a[1] == b[0] && a[2] == b[1]) 
//                     // or other bad shared vertex pair/situation
//                 {
//                     return false;
//                 }
//             }
//         }
//         true
//     } 

//     // 3. Write a function that returns the number of loops bounding a surface mesh.
//     // I would need to track the data in some other way or use the min angle function 
//     // to find all the triangles of a face and go from there

//     /// 4. Write a function that returns all faces with minimum angle below a specified angle in degrees.
//     /// Collecting any triangle that connects with another triangle with given angle or less.
//     pub fn faces_with_minimum_angle(&self, angle: f64) -> Selection {
//         let mut faces = HashSet::new();
//         for (index_a, a) in self.faces.windows(3).step_by(3).enumerate() {
//             for (index_b, b) in self.faces.windows(3).step_by(3).enumerate().skip(index_a + 1) {
//                 let norm_a = self.face_normal(a);
//                 let norm_b = self.face_normal(b);
//                 if norm_a.dot(&norm_b).acos().to_degrees() <= angle {
//                     faces.extend([index_a, index_b]);
//                 };
//             }
//         }
//         Selection {vertices: HashSet::new(), faces}
//     }

//     /// Get normal of face using cross product of two edges 
//     pub fn face_normal(&self, face: &[usize]) -> Vector3 {
//         let a = self.vertex_coordinates(face[0]);
//         let b = self.vertex_coordinates(face[1]);
//         let c = self.vertex_coordinates(face[2]);
//         // Unwrapping result for simplicity right now (could be NaN!).
//         // Normally we should handle the error if vector is too short to normalized
//         (&b - &a).cross(&c - &a).normalized().unwrap()
//     }
    
//     /// 5. Write a function that collapses all edges with length below a specified threshold.
//     /// Not finished!
//     pub fn collapse_short_edges(&mut self, threshold: f64) {
//         let mut verts_to_delete: Vec<usize> = vec![];
//         for vi in self.faces.windows(3).step_by(3) {
//             let a = self.vertex_coordinates(vi[0]);
//             let b = self.vertex_coordinates(vi[1]);
//             let c = self.vertex_coordinates(vi[2]);
//             if (&a - &b).length() < threshold {
//                 let effected_faces = self.select_adjacent_by_vertex_index(vi[0]);
//                 verts_to_delete.extend(&vi[0..1])
//             }
//             // TODO: check other edges
//         }
//         // self.delete_vertex(index, true);
//     }
// }

// /// List of selected vertex and face indices in a mesh.
// pub struct Selection {
//     pub vertices: HashSet<usize>,
//     pub faces: HashSet<usize>,
// }