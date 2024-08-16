use std::{hash::Hash, sync::Arc};
use super::*;

#[derive(Default, Eq)]
pub struct ArcFace(Arc<Face>);

impl PartialEq for ArcFace {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Hash for ArcFace {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.a.hash(state);
        self.0.b.hash(state);
        self.0.c.hash(state);
    }
}

#[derive(Default, PartialEq, Eq)]
pub struct Face {
    a: ArcVertex,
    b: ArcVertex,
    c: ArcVertex,
}