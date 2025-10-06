/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self { // new传入比较器
        Self {
            count: 0,
            items: vec![T::default()], // 注意，0号位不存放真正元素
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.count += 1;
        self.items.push(value); // 注意，这之后value就无法在外边使用了

        let mut now_idx = self.count;
        // 调整这个元素到正确位置

        while now_idx > 1 {
            let parent_idx = self.parent_idx(now_idx);
            let parent_value = &self.items[parent_idx];
            let now_value = &self.items[now_idx];
            if (self.comparator)(now_value, parent_value) { // 需要调整
                self.items.swap(parent_idx, now_idx);
                now_idx = parent_idx;
            } else { // 调整结束
                break;
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
        //TODO
		0
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
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
		// 最顶上元素返回，如果没有就返回None
        match self.len() {
            0 => None,
            1 => { // 返回唯一的元素
                self.items.pop()
            },
            _ => {
                // 首位元素互换，然后安置好首元素
                self.items.swap(1, self.count);
                let ret = self.items.pop();
                self.count -= 1;
                
                // 将首元素下移
                let mut now_idx: usize = 1;
                while now_idx <= self.len() {
                    let left_idx: usize = self.left_child_idx(now_idx);
                    if left_idx < self.len() { // 存在左子节点
                        let right_idx = self.right_child_idx(now_idx);
                        if right_idx < self.len() { // 存在左右节点，选其中较小的交换（当然这是对小顶堆来说）
                            if (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
                                // 跟左边的交换
                                self.items.swap(left_idx, now_idx);
                                now_idx = left_idx;
                            } else {
                                // 跟右边交换
                                self.items.swap(right_idx, now_idx);
                                now_idx = right_idx;
                            }
                        } else { // 只存在左节点 那只需要跟左节点比较
                            if !(self.comparator)(&self.items[now_idx], &self.items[left_idx]) {
                                self.items.swap(left_idx, now_idx);
                                now_idx = left_idx;
                            }
                        }
                    } else { // 不存在左子节点了，直接返回
                        break;
                    }
                    // println!("after next: {:?}", self.items);
                }

                ret
            }
        }
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
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}