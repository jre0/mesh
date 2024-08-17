use super::*;
use std::{collections::HashSet, hash::Hash};

/// Face made of smart pointers to vertices
#[derive(Default, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Face {
    pub a: Pointer<Vertex>,
    pub b: Pointer<Vertex>,
    pub c: Pointer<Vertex>,
}

impl Face {
    /// F. Construct a new face from vertices
    pub fn new(vertices: [&Pointer<Vertex>; 3]) -> Pointer<Self> {
        let face = Pointer::new(Self {
            a: vertices[0].clone(),
            b: vertices[1].clone(),
            c: vertices[2].clone(),
        });
        let weak_face = Arc::downgrade(&face.0);
        // add weak back reference of face to vertices
        vertices[0].push_face_back_ref(&weak_face);
        vertices[1].push_face_back_ref(&weak_face);
        vertices[2].push_face_back_ref(&weak_face);
        face
    }

    /// Get short edge if there
    pub fn short_edge(&self, tol: f64) -> Option<Pointer<Edge>> {
        if (self.b.point() - self.a.point()).length() < tol {
            return Some(Edge::new(&self.a, &self.b));
        } else if (self.c.point() - self.a.point()).length() < tol {
            return Some(Edge::new(&self.a, &self.c));
        }
        None
    }

    /// Create a new face by replacing a vertex.
    /// If the vertex is already used, none is returned because the face 
    /// has collapsed
    pub fn replace_vertex(&self, old: &Pointer<Vertex>, new: &Pointer<Vertex>) -> Option<Pointer<Self>> {
        if old == new {
            return None;
        } 
        let mut face = self.clone();
        if face.a == *old {
            face.a = new.clone();
        } else if face.b == *old {
            face.b = new.clone();
        } else if face.c == *old {
            face.c = new.clone();
        }
        Some(Pointer::new(face))
    }

    /// Get vertices A, B, and C of this face
    pub fn vertices(&self) -> [&Pointer<Vertex>; 3] {
        [&self.a, &self.b, &self.c]
    }

    /// Normal vector of face. 
    pub fn normal(&self) -> Result<Vector3, Error> {
        let delta0 = self.b.point() - self.a.point();
        let delta1 = self.c.point() - self.a.point();
        delta0.cross(delta1).normalized()
    }
}

impl Pointer<Face> {
    /// B. Select adjacent faces excluding self
    pub fn adjacent_faces(&self) -> HashSet<Pointer<Face>> {
        let mut faces = HashSet::new();
        faces.extend(self.a.adjacent_faces());
        faces.extend(self.b.adjacent_faces());
        faces.extend(self.c.adjacent_faces());
        faces.remove(self);
        faces
    }

    /// G. Flip the sense of a face.
    pub fn flipped(&self) -> Self {
        // A and B are swapped so the normal will be flipped
        Face::new([&self.b, &self.a, &self.c])
    }

    /// 2. Write a function that returns whether all faces are consistently oriented.
    /// (See Mesh::consistent_orientation)
    /// This can be made simpler by checking for edges with same common_id and if they go the 
    /// same direction as well, it's not consistent orientation
    pub fn adjacent_is_flipped(&self) -> bool {
        for face in self.adjacent_faces() {
            // Imagine two triangle blocks on a table that can be turned and slid around.
            // They should not flip or overlap. If any of these vertex pairs are shared,
            // it means one triangle is flipped/overlapped and does not share the same direction/orientation.
            #[allow(clippy::nonminimal_bool)]
            if (self.a == face.a && self.b == face.b)
                || (self.a == face.a && self.c == face.c)
                || (self.b == face.b && self.c == face.c)
                // Same as previous check but face "rotated 120".
                || (self.a == face.b && self.b == face.c)
                || (self.a == face.b && self.c == face.a)
                || (self.b == face.c && self.c == face.a)
                // Same as previous check but face "rotated another 120".
                || (self.a == face.c && self.b == face.a)
                || (self.a == face.c && self.c == face.b)
                || (self.b == face.a && self.c == face.b)
            {
                return true;
            }
        }
        false
    }

    /// Make edges in winding order refering to face vertices
    pub fn edges(&self) -> [Pointer<Edge>; 3] {
        [Edge::new(&self.a, &self.b), Edge::new(&self.b, &self.c), Edge::new(&self.c, &self.a)]
    }

    /// 4. Write a function that returns all faces with minimum angle below a specified angle in degrees.
    /// I am not sure by "MINIMUM angle BELOW a specified angle".
    /// I think below means there is a max angle and we want to be below that. (if there is a min, we want to be above it)
    /// This method will grow selection from this face, stopping at sharp corners where the angle is too great.
    pub fn grow_selection_with_max_angle(&self, angle: f64) -> Result<Mesh, Error> {
        let mut mesh = Mesh::default();
        self.insert_adjacent_with_max_angle(&mut mesh, angle)?;
        Ok(mesh)
    }

    /// Recursively insert adjacent faces into given mesh and edges mesh.
    /// Stop if the angle is too big or face already present in mesh.
    pub fn insert_adjacent_with_max_angle<'a>(
        &self,
        mesh: &'a mut Mesh,
        angle: f64,
    ) -> Result<&'a mut Mesh, Error> {
        mesh.insert_face(self);
        let base_normal = self.normal()?;
        for face in self.adjacent_faces() {
            if mesh.faces.contains(&face) {
                continue;
            }
            let normal = face.normal()?;
            if base_normal.dot(&normal).acos().to_degrees() <= angle {
                face.insert_adjacent_with_max_angle(mesh, angle)?;
            }
        }
        Ok(mesh)
    }
}