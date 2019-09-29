use std::rc::Rc;

/// Wrapper around a `Vec` if `Rc`,
/// that ensures pointer uniqueness of its elements
#[derive(Debug, Clone, Default)]
pub struct RcUniqueVec<T> {
    data: Vec<Rc<T>>,
}

impl<T> RcUniqueVec<T> {
    /// Creates an empty `RcUniqueVec`
    pub fn new() -> RcUniqueVec<T> {
        RcUniqueVec { data: Vec::new() }
    }

    /// Checks for pointer collision while inserting.
    /// Returns the index for the position of the inserted element.
    pub fn insert(&mut self, element: Rc<T>) -> usize {
        match self.data.iter().position(|t| Rc::ptr_eq(t, &element)) {
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
    pub fn remove(&mut self, element: &Rc<T>) -> bool {
        match self.data.iter().position(|t| Rc::ptr_eq(t, element)) {
            Some(index) => {
                self.data.remove(index);
                true
            }
            None => false,
        }
    }

    /// Checks if that element is in this `RcUniqueVec`
    /// and returns its index if possible
    pub fn get_index(&self, element: &Rc<T>) -> Option<usize> {
        match self.data.iter().position(|t| Rc::ptr_eq(t, element)) {
            Some(index) => Some(index),
            None => None,
        }
    }

    /// Clears all elements from this vector
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Returns the reference to the internal vector
    pub fn as_vec(&self) -> &Vec<Rc<T>> {
        &self.data
    }
}
