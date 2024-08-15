// use std::usize;

use super::*;

const HEAD: &str = "Test Export\n\n";

impl Mesh {
    pub fn write(&self, path: &str) -> Result<(), Error> {
        let mut out = HEAD.to_owned();
        out += "# Vertices\n";
        for data in self.vertices.windows(3).step_by(3) {
            self.push_vertices(&mut out, data);
        }
        out += "# Faces\n";
        for data in self.faces.windows(3).step_by(3) {
            out += &self.face(data);
        }
        fs::write(path, out)?;
        Ok(())
    }

    /// Push triangle vertices
    fn push_vertices(&self, out: &mut String, d: &[f64]) {
        *out += &format!("v {:.6} {:.6} {:.6}\n", d[0], d[1], d[2]);
    }

    /// Make face from one vertex index.
    fn face(&self, i: &[usize]) -> String {
        format!("f {} {} {}\n", i[0], i[1], i[2])
    }
}