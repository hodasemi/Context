use std::sync::Arc;

/// Wrapper around a `Vec` if `Rc`,
/// that ensures pointer uniqueness of its elements
#[derive(Debug, Clone, Default)]
pub struct ArcUniqueVec<T> {
    data: Vec<Arc<T>>,
}

impl<T> ArcUniqueVec<T> {
    /// Creates an empty `RcUniqueVec`
    pub fn new() -> ArcUniqueVec<T> {
        ArcUniqueVec { data: Vec::new() }
    }

    /// Checks for pointer collision while inserting.
    /// Returns the index for the position of the inserted element.
    pub fn insert(&mut self, element: Arc<T>) -> usize {
        match self.data.iter().position(|t| Arc::ptr_eq(t, &element)) {
            Some(index) => index,
            None => {
                let index = self.data.len();
                self.data.push(element);
                index
            }
        }
    }

    /// Checks if the given element is in the vector, then returns true,
    /// otherwise false.
    pub fn remove(&mut self, element: &Arc<T>) -> bool {
        match self.data.iter().position(|t| Arc::ptr_eq(t, element)) {
            Some(index) => {
                self.data.remove(index);
                true
            }
            None => false,
        }
    }

    /// Checks if that element is in this `RcUniqueVec`
    /// and returns its index if possible
    pub fn get_index(&self, element: &Arc<T>) -> Option<usize> {
        match self.data.iter().position(|t| Arc::ptr_eq(t, element)) {
            Some(index) => Some(index),
            None => None,
        }
    }

    /// Clears all elements from this vector
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Returns the reference to the internal vector
    pub fn as_vec(&self) -> &Vec<Arc<T>> {
        &self.data
    }
}
