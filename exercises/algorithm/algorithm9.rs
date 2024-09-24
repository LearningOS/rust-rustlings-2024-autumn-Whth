/*
    heap
    This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;
use std::fmt::{Debug, Display};

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
    current: usize,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
            current: 1,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;
        self.rectify()
    }

    fn rectify(&mut self) {
        let mut need_rectify = self.count > 1;
        while need_rectify {
            need_rectify = false;
            for i in (2..self.count + 1).rev() {
                let parent_idx = self.parent_idx(i);

                if (self.comparator)(&self.items[i], &self.items[parent_idx]) {
                    println!("swap {} and {}", i, parent_idx);
                    self.items.swap(i, parent_idx);
                    need_rectify = true;

                    if parent_idx == 1 {
                        self.current = 1
                    }
                }
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let smallest_child = if self.children_present(idx) {
            let left_child_idx = self.left_child_idx(idx);
            let right_child_idx = self.right_child_idx(idx);
            if right_child_idx <= self.count
                && (self.comparator)(&self.items[left_child_idx], &self.items[right_child_idx])
            {
                left_child_idx
            } else {
                right_child_idx
            }
        } else {
            idx
        };
        smallest_child
    }
    fn show(&self)
    where
        T: Display + Debug,
    {
        for element in &self.items {
            println!("{}", element);
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        println!("cur:{}", self.current);
        if self.count == 0 {
            return None;
        }
        let val = self.items[self.current];
        if self.count % 2 == 0 {
            self.current += 1;
        } else {
            self.current -= 1;
            self.current *= 2;
        }

        return Some(val);
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_heap_with_one_element() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        for i in heap.items.iter() {
            println!("{}", i);
        }
        assert_eq!(heap.len(), 1);
        assert_eq!(heap.next(), Some(4));
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        heap.show();

        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        heap.show();
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        heap.show();
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);

        assert_eq!(heap.next(), Some(2));
    }
}
