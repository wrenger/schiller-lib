use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::iter::FusedIterator;

/// Just a simple wrapper for a sortable list
#[derive(Clone, Debug)]
pub struct Sorted<T, F: Fn(&T, &T) -> Ordering + Copy> {
    cmp: F,
    heap: BinaryHeap<SortedItem<T, F>>,
}

impl<T, F: Fn(&T, &T) -> Ordering + Copy> Sorted<T, F> {
    pub fn new(cmp: F) -> Self {
        Self {
            heap: BinaryHeap::new(),
            cmp,
        }
    }

    pub fn push(&mut self, item: T) {
        self.heap.push(SortedItem {
            item,
            cmp: self.cmp,
        })
    }

    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|s| s.item)
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }
    #[allow(unused)]
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}

impl<T, F: Fn(&T, &T) -> Ordering + Copy> IntoIterator for Sorted<T, F> {
    type Item = T;
    type IntoIter = IntoIterSorted<T, F>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterSorted { inner: self }
    }
}

#[derive(Clone, Debug)]
struct SortedItem<T, F: Fn(&T, &T) -> Ordering> {
    item: T,
    cmp: F,
}

impl<T, F: Fn(&T, &T) -> Ordering> Ord for SortedItem<T, F> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.cmp)(&self.item, &other.item).reverse()
    }
}
impl<T, F: Fn(&T, &T) -> Ordering> PartialOrd for SortedItem<T, F> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T, F: Fn(&T, &T) -> Ordering> PartialEq for SortedItem<T, F> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl<T, F: Fn(&T, &T) -> Ordering> Eq for SortedItem<T, F> {}

// TODO: remove in favor of binary_heap_into_iter_sorted #59278
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Clone, Debug)]
pub struct IntoIterSorted<T, F: Fn(&T, &T) -> Ordering + Copy> {
    inner: Sorted<T, F>,
}

impl<T, F: Fn(&T, &T) -> Ordering + Copy> Iterator for IntoIterSorted<T, F> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.inner.pop()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.inner.len();
        (exact, Some(exact))
    }
}

impl<T, F: Fn(&T, &T) -> Ordering + Copy> ExactSizeIterator for IntoIterSorted<T, F> {}
impl<T, F: Fn(&T, &T) -> Ordering + Copy> FusedIterator for IntoIterSorted<T, F> {}
