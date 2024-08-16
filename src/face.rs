use std::hash::Hash;
use super::*;

#[derive(Default, PartialEq, Eq, Hash)]
pub struct Face {
    a: ArcPlus<Vertex>,
    b: ArcPlus<Vertex>,
    c: ArcPlus<Vertex>,
}

impl Face {
    pub fn vertices(&self) -> [ArcPlus<Vertex>; 3] {
        [self.a.clone(), self.b.clone(), self.c.clone()]
    }
}

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