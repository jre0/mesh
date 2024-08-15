use super::*;

impl Mesh {
    pub fn read(path: &str) -> Result<Self, Error> {
        let data = fs::read_to_string(path)?;
        let mut mesh = Self::new();
        mesh.read_vertices(&data)?;
        mesh.read_tris(&data)?;
        mesh.read_quads(&data)?;
        Ok(mesh)
    }

    pub fn read_vertices(&mut self, data: &str) -> Result<(), Error> {
        let regex = Regex::new(r"v (-?[0-9]\d*\.\d+) (-?[0-9]\d*\.\d+) (-?[0-9]\d*\.\d+)")?;
        for caps in regex.captures_iter(&data) {
            self.vertices.extend([
                self.parse_vertex_component(&caps, 1)?, 
                self.parse_vertex_component(&caps, 2)?, 
                self.parse_vertex_component(&caps, 3)?
            ]);
        }
        Ok(())
    }

    pub fn read_tris(&mut self, data: &str) -> Result<(), Error> {
        let regex = Regex::new(r"f ([0-9]*) ([0-9]*) ([0-9]*)\n")?;
        for caps in regex.captures_iter(&data) {
            self.faces.extend([
                self.parse_vertex_index(&caps, 1)?,
                self.parse_vertex_index(&caps, 2)?,
                self.parse_vertex_index(&caps, 3)?,
            ]);
        }
        Ok(())
    }

    pub fn read_quads(&mut self, data: &str) -> Result<(), Error> {
        let regex = Regex::new(r"f ([0-9]*) ([0-9]*) ([0-9]*) ([0-9]*)")?;
        for caps in regex.captures_iter(&data) {
            let i1 = self.parse_vertex_index(&caps, 1)?;
            let i2 = self.parse_vertex_index(&caps, 2)?;
            let i3 = self.parse_vertex_index(&caps, 3)?;
            let i4 = self.parse_vertex_index(&caps, 4)?;
            // make two triangles from the quad
            self.faces.extend([i1, i2, i3]);
            self.faces.extend([i1, i3, i4]);
        }
        Ok(())
    }

    /// X, Y, or Z component from capture groups
    fn parse_vertex_component(&self, caps: &Captures, group_index: usize) -> Result<f64, Error> {
        Ok(caps
            .get(group_index)
            .ok_or("parse_vertex_component failed")?
            .as_str()
            .parse::<f64>()?)
    }

    /// face vertex index with 1 subtracted so index starts at 0
    fn parse_vertex_index(&self, caps: &Captures, group_index: usize) -> Result<usize, Error> {
        Ok(caps
            .get(group_index)
            .ok_or("parse_vertex_index failed")?
            .as_str()
            .parse::<usize>()? - 1)
    }
}