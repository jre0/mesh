pub use convert::IntoMesh;

use super::*;
use regex::{Captures, Regex};
use std::{
    collections::{HashMap, HashSet},
    fs,
};

mod convert;
mod read;
mod write;

/// Mesh made of smart pointers to vertices, faces, and edges
/// Can represent primary loaded data or subset/selection
#[derive(Default, Clone, Debug)]
pub struct Mesh {
    pub vertices: HashSet<Pointer<Vertex>>,
    pub faces: HashSet<Pointer<Face>>,
    pub edges: HashSet<Pointer<Edge>>,
}

impl Mesh {
    /// C. Return all the vertices
    pub fn vertex_list(&self) -> Vec<&Pointer<Vertex>> {
        self.vertices.iter().collect()
    }

    /// C. Return all the faces
    pub fn face_list(&self) -> Vec<&Pointer<Face>> {
        self.faces.iter().collect()
    }

    /// E. Delete a vertex, with optional flag to delete all connected faces.
    pub fn remove_vertex(&mut self, vertex: &Pointer<Vertex>, delete_faces: bool) {
        if delete_faces {
            for face in vertex.adjacent_faces() {
                self.faces.remove(&face);
            }
        }
        self.vertices.remove(vertex);
    }

    /// E. Delete a face
    pub fn remove_face(&mut self, face: &Pointer<Face>) {
        self.faces.remove(face);
    }

    /// G. Flip the sense of a face.
    pub fn flip_face(&mut self, face: &Pointer<Face>) {
        self.remove_face(face);
        self.insert_face(&face.flipped());
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

    /// 3. Write a function that returns the number of loops bounding a surface mesh.
    /// The idea is to collect all edges then we can traverse the edges in order
    /// by using the forward and backward refs with vertices.
    pub fn surface_bounding_loop_count(&self) -> Result<(usize, Mesh), Error> {
        // mesh of edges to examine
        let mut edges: HashMap<u64, Pointer<Edge>> = HashMap::new();
        // collect open edges
        for face in &self.faces {
            for edge in face.edges() {
                if edges.contains_key(&edge.id_sum) {
                    // If this edge (independent of direction) has been collected alread, remove it
                    // because we are looking for edges only touched by one face.
                    edges.remove(&edge.id_sum);
                } else {
                    edges.insert(edge.id_sum, edge);
                }
            }
        }
        let mut remaining: HashSet<Pointer<Edge>> = HashSet::from_iter(edges.values().cloned());
        let mut count = 0;
        let mut current = remaining.iter().next().ok_or("no edges")?.clone();
        // Count loops
        while !remaining.is_empty() {
            remaining.remove(&current);
            if let Some(edge) = current.next_edge() {
                if remaining.contains(&edge) {
                    current = edge;
                    continue;
                }
            }
            count += 1;
            if let Some(edge) = remaining.iter().next() {
                current = edge.clone();
            }
        }
        // make a mesh to visualize the loop vertices
        let mut mesh = Mesh::default();
        mesh.vertices
            .extend(edges.values().flat_map(|e| [e.a.clone(), e.b.clone()]));
        Ok((count, mesh))
    }

    /// 5. Write a function that collapses all edges with length below a specified threshold.
    pub fn collapse_edges(&self, tol: f64) -> Mesh {
        let mut mesh = self.clone();
        for base_face in &self.faces {
            if let Some(edge) = base_face.short_edge(tol) {
                for old_face in edge.a.adjacent_faces() {
                    if let Some(new_face) = old_face.replace_vertex(&edge.a, &edge.b) {
                        mesh.insert_face(&new_face);
                    }
                    mesh.remove_face(&old_face);
                }
            }
        }
        mesh
    }

    /// Insert face pointer
    pub fn insert_face(&mut self, face: &Pointer<Face>) {
        self.faces.insert(face.clone());
    }

    /// Insert edge pointer
    pub fn insert_edge(&mut self, edge: &Pointer<Edge>) {
        self.edges.insert(edge.clone());
    }
}
