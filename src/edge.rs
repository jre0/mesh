use std::sync::Arc;
use super::*;

pub struct Edge {
    a: Arc<Vertex>,
    b: Arc<Vertex>,
}