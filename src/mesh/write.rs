use std::collections::HashMap;

use super::*;

const HEAD: &str = "Test Export\n\n";

impl Mesh {
    /// Write mesh to OBJ file at path
    pub fn write(&self, path: &str) -> Result<(), Error> {
        let mut out = HEAD.to_owned();
        out += "# Vertices\n";
        let mut faces_out = "# Faces\n".to_owned();
        let mut vertices = HashMap::new();
        let mut count: usize = 0;
        // Collect vertices from faces and pre-write faces
        // as indices are assigned to vertices
        for face in &self.faces {
            let mut indices = vec![];
            for vertex in face.vertices() {
                if let Some(index) = vertices.get(vertex) {
                    indices.push(*index);
                } else {
                    out += &self.vertex_entry(vertex.point());
                    // Start index at 1 for OBJ format
                    count += 1;
                    vertices.insert(vertex.clone(), count);
                    indices.push(count);
                }
            }
            faces_out += &self.face_entry(indices);
        }
        // Output leftover vertices that no faces are referencing
        for vertex in self.vertices.iter() {
            if !vertices.contains_key(vertex) {
                out += &self.vertex_entry(vertex.point());
            }
        }
        // Append the faces to the existing doc of vertices
        out += &("\n\n".to_owned() + &faces_out);
        fs::write(path, out)?;
        Ok(())
    }

    /// Vertex entry for OBJ format from point
    fn vertex_entry(&self, point: &Vector3) -> String {
        format!("v {:.6} {:.6} {:.6}\n", point.x, point.y, point.z)
    }

    /// Face entry for OBJ format from vertex indices
    fn face_entry(&self, indices: Vec<usize>) -> String {
        format!("f {} {} {}\n", indices[0], indices[1], indices[2])
    }
}
