use std::{collections::HashSet, hash::Hash, sync::Weak};
use super::*;

#[derive(Default)]
pub struct ArcVertex(Vertex);

impl Hash for ArcVertex {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.point.hash(state);
    }
}

#[derive(Default)]
pub struct Vertex {
    point: Vector3,
    edges: HashSet<Weak<Edge>>,
    faces: HashSet<Weak<Face>>,
}

impl Vertex {
    pub fn faces(&self) -> Pick {
        let mut pick = Pick::default();
        for weak_face in &self.faces {
            if let Some(strong_face) = weak_face.upgrade() {
                pick.faces.insert()
            }
        }
        pick
    }
}