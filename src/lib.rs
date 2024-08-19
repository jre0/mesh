pub use config::*;
pub use edge::*;
pub use face::*;
pub use mesh::*;
pub use vector3::*;
pub use vertex::*;

use std::{error, hash::Hash, ops::Deref, sync::Arc};

mod config;
mod edge;
mod face;
mod mesh;
#[cfg(test)]
mod tests;
mod vector3;
mod vertex;

/// Mesh crate error
pub type Error = Box<dyn error::Error>;

/// Atomic reference counter smart pointer.
/// Equality and Hash implementations allow insertion in HashSet.
#[derive(Debug)]
pub struct Pointer<T>(Arc<T>);

impl<T> Deref for Pointer<T> {
    type Target = T;
    /// Auto deref of Pointer to contents
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Default> Default for Pointer<T> {
    fn default() -> Self {
        Self(Arc::new(T::default()))
    }
}

impl<T> Clone for Pointer<T> {
    /// Clone the pointer. The data is not cloned.
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Eq for Pointer<T> where T: Eq {}

impl<T> PartialEq for Pointer<T> {
    /// Pointer equality
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl<T: Hash> Hash for Pointer<T> {
    /// Hash Pointer by hashing contents
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> Pointer<T> {
    pub fn new(item: T) -> Self {
        Self(Arc::new(item))
    }
    /// New pointer directly from atomic reference counter
    pub fn from_arc(arc: Arc<T>) -> Self {
        Self(arc)
    }
}
