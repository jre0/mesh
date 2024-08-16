use std::hash::Hash;
use super::*;

#[derive(Default, PartialEq, Eq, Hash)]
pub struct Face {
    a: Pointer<Vertex>,
    b: Pointer<Vertex>,
    c: Pointer<Vertex>,
}

impl Face {
    /// F. Construct a new face from vertices, and a new vertex from coordinates.
    /// (new face from vertices)
    pub fn new(vertices: [&Pointer<Vertex>; 3]) -> Pointer<Self> {
        let arc = Arc::new(Self {
            a: vertices[0].clone(),
            b: vertices[1].clone(),
            c: vertices[2].clone(),
        });
        let mut face = Pointer::from_arc(arc);
        let weak_face = Arc::downgrade(&face.0);
        // face.a = vertices[0].with_weak_face(&weak_face);
        face
    }

    /// G. Flip the sense of a face.
    pub fn flip(&mut self) {
        let original_a = self.a.clone();
        self.a = self.b.clone();
        self.b = original_a;
    }

    pub fn adjacent_is_flipped(&self) -> bool {
        let mut mesh = self.a.adjacent_faces();
        mesh.extend(self.b.adjacent_faces());
        mesh.extend(self.c.adjacent_faces());
        for face in mesh.face_list() {
            if self == face.0.as_ref() {
                continue;
            }
            // Imagine two triangle blocks on a table that can be turned and slid around.
            // They cannot flip or overlap. If any of these vertex pairs are shared, 
            // it means one triangle is flipped or overlapped and does not share the same direction/orientation. 
            if (self.a == face.a && self.b == face.b) 
                || (self.a == face.a && self.c == face.c) 
                || (self.b == face.b && self.c == face.c) 
                // same as previous check but face "rotated 120"
                || (self.a == face.b && self.b == face.c) 
                || (self.a == face.b && self.c == face.a) 
                || (self.b == face.c && self.c == face.a) 
                // same as previous check but face "rotated another 120"
                || (self.a == face.c && self.b == face.a) 
                || (self.a == face.c && self.c == face.b) 
                || (self.b == face.a && self.c == face.b) 
            {
                return true;
            }
        }
        false
    }

    pub fn vertices(&self) -> Mesh {
        let mut mesh = Mesh::default();
        mesh.vertices.extend([self.a.clone(), self.b.clone(), self.c.clone()]);
        mesh 
    }
}


// pub fn vertices(&self) -> [&Pointer<Vertex>; 3] {
//     [&self.a, &self.b, &self.c]
// }

// pub fn vertices(&self) -> [&Pointer<Vertex>; 3] {
//     [&self.a, &self.b, &self.c]
// }


// pub fn vertices(&self) -> Mesh {
//     Mesh::from_vertices(vec![self.a.clone(), self.b.clone(), self.c.clone()])
// }

// #[derive(Default, Eq)]
// pub struct ArcFace(Arc<Face>);

// impl PartialEq for ArcFace {
//     fn eq(&self, other: &Self) -> bool {
//         Arc::ptr_eq(&self.0, &other.0)
//     }
// }

// impl Hash for ArcFace {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.0.a.hash(state);
//         self.0.b.hash(state);
//         self.0.c.hash(state);
//     }
// }