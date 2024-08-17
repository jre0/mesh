use super::*;

impl Mesh {
    /// Read mesh from OBJ file at path
    pub fn read(path: &str) -> Result<Self, Error> {
        let data = fs::read_to_string(path)?;
        let mut mesh = Self::default();
        let vertices = mesh.read_vertices(&data)?;
        mesh.read_tris(&data, &vertices)?;
        mesh.read_quads(&data, &vertices)?;
        Ok(mesh)
    }

    /// Put vertices directly into Mesh and keep a list for read_tris and read_quads to find by index
    pub fn read_vertices(&mut self, data: &str) -> Result<Vec<Pointer<Vertex>>, Error> {
        let mut vertices = vec![];
        let regex = Regex::new(r"v (-?[0-9]\d*\.\d+) (-?[0-9]\d*\.\d+) (-?[0-9]\d*\.\d+)")?;
        for caps in regex.captures_iter(data) {
            let vector = Vector3::new(
                self.parse_vertex_component(&caps, 1)?,
                self.parse_vertex_component(&caps, 2)?,
                self.parse_vertex_component(&caps, 3)?,
            );
            let vertex = Vertex::new(vector);
            self.vertices.insert(vertex.clone());
            vertices.push(vertex);
        }
        Ok(vertices)
    }

    /// Read all triangles in obj file
    pub fn read_tris(&mut self, data: &str, vertices: &[Pointer<Vertex>]) -> Result<(), Error> {
        let regex = Regex::new(r"f ([0-9]*) ([0-9]*) ([0-9]*)\n")?;
        for caps in regex.captures_iter(data) {
            let face = Face::new([
                &vertices[self.parse_vertex_index(&caps, 1)?],
                &vertices[self.parse_vertex_index(&caps, 2)?],
                &vertices[self.parse_vertex_index(&caps, 3)?],
            ]);
            self.faces.insert(face);
        }
        Ok(())
    }

    /// Read all quads in obj file
    pub fn read_quads(&mut self, data: &str, vertices: &[Pointer<Vertex>]) -> Result<(), Error> {
        let regex = Regex::new(r"f ([0-9]*) ([0-9]*) ([0-9]*) ([0-9]*)")?;
        for caps in regex.captures_iter(data) {
            let i1 = &vertices[self.parse_vertex_index(&caps, 1)?];
            let i2 = &vertices[self.parse_vertex_index(&caps, 2)?];
            let i3 = &vertices[self.parse_vertex_index(&caps, 3)?];
            let i4 = &vertices[self.parse_vertex_index(&caps, 4)?];
            let first_face = Face::new([i1, i2, i3]);
            self.faces.insert(first_face);
            let second_face = Face::new([i1, i3, i4]);
            self.faces.insert(second_face);
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
            .parse::<usize>()?
            - 1)
    }
}
