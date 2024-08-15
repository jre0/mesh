use std::fs;
use regex::{Captures, Regex};
use super::*;

mod import;
mod export;

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
        let front_range = &self.vertices[0..i];
        let back_range = &self.vertices.clone()[i+1..];
        self.vertices = front_range.to_vec();
        self.vertices.extend(back_range);
        // Reduce vertex indices in faces after the deleted vertex
        for vertex_index in &mut self.faces {
            if *vertex_index > target_index {
                *vertex_index -= 1;
            }
        }
        if delete_faces {
            for face in adjacent.faces {

            }
        }
    }

    /// F. Construct a new face from vertices, ...
    pub fn add_face(&mut self, face: [usize; 3]) {
        self.faces.extend(face);
    }
    /// F. ... and a new vertex from coordinates.
    pub fn add_vertex(&mut self, coordinates: Vector3) {
        self.vertices.extend(coordinates.as_vec());
    }

    /// G. Flip the sense of a face
    pub fn flip_face(&mut self, face_index: usize) {
        
    }
}

/// List of selected vertex and face indices in a mesh.
pub struct Selection {
    pub vertices: Vec<usize>,
    pub faces: Vec<usize>,
}