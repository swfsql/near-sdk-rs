use borsh::{BorshDeserialize, BorshSerialize};
use core::{iter::FusedIterator, ops::Range};

use super::{Vector, ERR_INDEX_OUT_OF_BOUNDS};
use crate::env;

/// An iterator over references to each element in the stored vector.
#[derive(Debug)]
pub struct Iter<'a, T>
where
    T: BorshSerialize + BorshDeserialize,
{
    /// Underlying vector to iterate through
    vec: &'a Vector<T>,
    /// Range of indices to iterate.
    range: Range<u32>,
}

impl<'a, T> Iter<'a, T>
where
    T: BorshSerialize + BorshDeserialize,
{
    pub(super) fn new(vec: &'a Vector<T>) -> Self {
        Self { vec, range: Range { start: 0, end: vec.len() } }
    }

    /// Returns number of elements left to iterate.
    fn remaining(&self) -> usize {
        self.range.len()
    }
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: BorshSerialize + BorshDeserialize,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        <Self as Iterator>::nth(self, 0)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining();
        (remaining, Some(remaining))
    }

    fn count(self) -> usize {
        self.remaining()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let idx = self.range.nth(n)?;
        Some(self.vec.get(idx).unwrap_or_else(|| env::panic_str(ERR_INDEX_OUT_OF_BOUNDS)))
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> where T: BorshSerialize + BorshDeserialize {}
impl<'a, T> FusedIterator for Iter<'a, T> where T: BorshSerialize + BorshDeserialize {}

impl<'a, T> DoubleEndedIterator for Iter<'a, T>
where
    T: BorshSerialize + BorshDeserialize,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        <Self as DoubleEndedIterator>::nth_back(self, 0)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let idx = self.range.nth_back(n)?;
        Some(self.vec.get(idx).unwrap_or_else(|| env::panic_str(ERR_INDEX_OUT_OF_BOUNDS)))
    }
}

/// An iterator over exclusive references to each element of a stored vector.
#[derive(Debug)]
pub struct IterMut<'a, T>
where
    T: BorshSerialize + BorshDeserialize,
{
    /// Mutable reference to vector used to iterate through.
    vec: &'a mut Vector<T>,
    /// Range of indices to iterate.
    range: Range<u32>,
}

impl<'a, T> IterMut<'a, T>
where
    T: BorshSerialize + BorshDeserialize,
{
    /// Creates a new iterator for the given storage vector.
    pub(crate) fn new(vec: &'a mut Vector<T>) -> Self {
        let end = vec.len();
        Self { vec, range: Range { start: 0, end } }
    }

    /// Returns the amount of remaining elements to yield by the iterator.
    fn remaining(&self) -> usize {
        self.range.len()
    }
}

impl<'a, T> IterMut<'a, T>
where
    T: BorshSerialize + BorshDeserialize,
{
    fn get_mut<'b>(&'b mut self, at: u32) -> Option<&'a mut T> {
        self.vec.get_mut(at).map(|value| {
            //* SAFETY: The lifetime can be swapped here because we can assert that the iterator
            //*         will only give out one mutable reference for every individual item
            //*         during the iteration, and there is no overlap. This must be checked
            //*         that no element in this iterator is ever revisited during iteration.
            unsafe { &mut *(value as *mut T) }
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T>
where
    T: BorshSerialize + BorshDeserialize,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        <Self as Iterator>::nth(self, 0)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining();
        (remaining, Some(remaining))
    }

    fn count(self) -> usize {
        self.remaining()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let idx = self.range.nth(n)?;
        Some(self.get_mut(idx).unwrap_or_else(|| env::panic_str(ERR_INDEX_OUT_OF_BOUNDS)))
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> where T: BorshSerialize + BorshDeserialize {}
impl<'a, T> FusedIterator for IterMut<'a, T> where T: BorshSerialize + BorshDeserialize {}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T>
where
    T: BorshSerialize + BorshDeserialize,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        <Self as DoubleEndedIterator>::nth_back(self, 0)
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let idx = self.range.nth_back(n)?;
        Some(self.get_mut(idx).unwrap_or_else(|| env::panic_str(ERR_INDEX_OUT_OF_BOUNDS)))
    }
}
