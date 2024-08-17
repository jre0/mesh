use super::*;

pub trait IntoMesh {
    fn into_mesh(self) -> Mesh;
}

impl IntoMesh for Vec<Pointer<Vertex>> {
    /// Convert vertices to `Mesh`
    fn into_mesh(self) -> Mesh {
        let mut mesh = Mesh::default();
        mesh.vertices.extend(self.iter().cloned());
        mesh
    }
}

impl IntoMesh for HashSet<Pointer<Face>> {
    /// Convert faces to `Mesh`
    fn into_mesh(self) -> Mesh {
        let mut mesh = Mesh::default();
        mesh.faces.extend(self.iter().cloned());
        mesh
    }
}
