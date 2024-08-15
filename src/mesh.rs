use std::fs;
use regex::{Captures, Regex};
use super::*;

mod read;
mod write;

/// Mesh
#[derive(Default)]
pub struct Mesh {
    pub vertices: Vec<f64>,
    pub faces: Vec<usize>,
}

impl Mesh {
    pub fn new() -> Self {
        Self::default()
    }

    /// A. Read/Write located in src/mesh

    // TODO: do the same by face index
    /// B. Given a vertex/face, return the adjacent faces/vertices
    pub fn select_adjacent_by_vertex_index(&self, target_index: usize) -> Selection {
        let mut vertices = vec![];
        let mut faces = vec![];
        for (face_index, face) in self.faces.windows(3).step_by(3).enumerate() {
            if face.iter().find(|vertex_index| **vertex_index == target_index).is_some() {
                faces.push(face_index);
                vertices.extend(face);
            }
        }
        Selection {vertices, faces}
    }

    /// C. Return all the vertices or faces.
    pub fn select_all(&self) -> Selection {
        let vertices = (0..self.vertices.len()/3).collect();
        let faces = (0..self.faces.len()/3).collect();
        Selection {vertices, faces}
    }

    /// D. Return the coordinates of a given vertex.
    pub fn vertex_coordinates(&self, index: usize) -> Vector3 {
        let i = index * 3;
        Vector3::new(&self.vertices[i..i+3])
    }

    /// E. Delete a vertex or face, with optional flag to delete all connected faces (if a vertex)
    pub fn delete_vertex(&mut self, target_index: usize, delete_faces: bool) {
        let adjacent = self.select_adjacent_by_vertex_index(target_index);
        let i = target_index * 3;
        let slice_0 = &self.vertices[..i];
        let slice_1 = &self.vertices.clone()[i+3..];
        self.vertices = slice_0.to_vec();
        self.vertices.extend(slice_1);
        // Reduce vertex indices in faces after the deleted vertex
        for vertex_index in &mut self.faces {
            if *vertex_index > target_index {
                *vertex_index -= 1;
            }
        }
        if delete_faces {
            for index in adjacent.faces {
                self.delete_face(index);
            }
        }
    }

    /// Delete a face
    pub fn delete_face(&mut self, target_index: usize) {
        let i = target_index * 3;
        let slice_0 = &self.faces[..i];
        let slice_1 = &self.faces.clone()[i+3..];
        self.faces = slice_0.to_vec();
        self.faces.extend(slice_1);
    }

    /// F. Construct a new face from vertices, ...
    pub fn add_face(&mut self, vertice_indices: [usize; 3]) {
        self.faces.extend(vertice_indices);
    }
    /// F. ... and a new vertex from coordinates.
    pub fn add_vertex(&mut self, coordinates: Vector3) {
        self.vertices.extend(coordinates.as_vec());
    }

    /// G. Flip the sense of a face
    pub fn flip_face(&mut self, index: usize) {
        let i = index * 3;
        let first_vertex_index = self.faces[i];
        self.faces[i] = self.faces[i + 1];
        self.faces[i + 1] = first_vertex_index;
    }

    /// 2. Write a function that returns whether all faces are consistently oriented.
    pub fn consistent_orientation(&self) -> bool {
        for l in self.faces.windows(3).step_by(3) {
            for r in self.faces.windows(3).step_by(3) {
                if l != r {
                    if (l[0] == r[0] && l[1] == r[1]) 
                        || (l[0] == r[0] && l[2] == r[2]) 
                        // or other bad shared vertex pair/situation
                    {
                        return false;
                    }
                }
            }
        }
        true
    } 
    
    /// 5. Write a function that collapses all edges with length below a specified threshold.
    /// Not finished!
    pub fn collapse_short_edges(&mut self, threshold: f64) {
        let mut verts_to_delete: Vec<usize> = vec![];
        for vi in self.faces.windows(3).step_by(3) {
            let a = self.vertex_coordinates(vi[0]);
            let b = self.vertex_coordinates(vi[1]);
            let c = self.vertex_coordinates(vi[2]);
            if (&a - &b).length() < threshold {
                let effected_faces = self.select_adjacent_by_vertex_index(vi[0]);
                verts_to_delete.extend(&vi[0..1])
            }
            // TODO: check other edges
        }
        // self.delete_vertex(index, true);
    }
}

/// List of selected vertex and face indices in a mesh.
pub struct Selection {
    pub vertices: Vec<usize>,
    pub faces: Vec<usize>,
}