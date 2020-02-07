use core::slice::{Iter, IterMut};
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct VkMappedMemory<'a, T>
where
    T: Clone,
{
    data: &'a mut [T],
}

impl<'a, T: Clone> VkMappedMemory<'a, T> {
    pub(crate) fn new(data: &'a mut [T]) -> VkMappedMemory<'a, T> {
        VkMappedMemory { data }
    }

    pub fn copy(&mut self, data: &[T]) {
        self.data.clone_from_slice(data);
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.data.iter_mut()
    }
}

impl<'a, T: Clone> Index<usize> for VkMappedMemory<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.data[index]
    }
}

impl<'a, T: Clone> IndexMut<usize> for VkMappedMemory<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.data[index]
    }
}
