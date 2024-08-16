use super::*;

#[derive(Default, PartialEq, Eq, Hash)]
pub struct Edge {
    a: Pointer<Vertex>,
    b: Pointer<Vertex>,
}