/*
	double linked list reverse
	This problem requires you to reverse a doubly linked list
*/
// I AM NOT DONE

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn len(&self) -> u32 {
        self.length
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        node.prev = self.end;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    pub fn reverse(&mut self) {
        let mut i_next = self.start;
        let mut j_next = self.end;
        if self.len() > 2 {
            let mut i = 0;
            let mut j = self.len() - 1;
            while i < j {
                let i_node = i_next; // 注意 i_node一定是非None，但i_next不一定
                let j_node: Option<NonNull<Node<T>>> = j_next;

                // unsafe { println!("i={} j={}, i_node: {:?} j_node: {:?}", i, j, *i_node.unwrap().as_ptr(), *j_node.unwrap().as_ptr()); }

                i_next = unsafe { (*i_node.unwrap().as_ptr()).next };
                j_next = unsafe { (*j_node.unwrap().as_ptr()).prev };

                // unsafe { println!("i={} j={}, i_node: {:?} j_node: {:?}", i, j, *i_next.unwrap().as_ptr(), *j_next.unwrap().as_ptr()); }

                // 找到i j对应节点并交换
                // 其实只是交换里面指针 - 不对，只是这样交换两个Option/NonNull没用，因为里面Node中val/next/prev都跟着一切交换，我实际上是想交换val而已
                // 正确的做法应该还是找到这两个i_node/j_node之后，改变next/prev指针指向
                // std::mem::swap(&mut i_node, &mut j_node); 

                // 这里我作弊下，直接交换val
                // TODO: 未来还是要改成直接修改指针
                unsafe { std::mem::swap(&mut (*i_node.unwrap().as_ptr()).val, &mut (*j_node.unwrap().as_ptr()).val); }

                i += 1;
                j -= 1;
            }
        }
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_reverse_linked_list_1() {
		let mut list = LinkedList::<i32>::new();
		let original_vec = vec![2,3,5,11,9,7];
		let reverse_vec = vec![7,9,11,5,3,2];
		for i in 0..original_vec.len(){
			list.add(original_vec[i]);
		}
		println!("Linked List is {}", list);
		list.reverse();
		println!("Reversed Linked List is {}", list);
		for i in 0..original_vec.len(){
			assert_eq!(reverse_vec[i],*list.get(i as i32).unwrap());
		}
	}

	#[test]
	fn test_reverse_linked_list_2() {
		let mut list = LinkedList::<i32>::new();
		let original_vec = vec![34,56,78,25,90,10,19,34,21,45];
		let reverse_vec = vec![45,21,34,19,10,90,25,78,56,34];
		for i in 0..original_vec.len(){
			list.add(original_vec[i]);
		}
		println!("Linked List is {}", list);
		list.reverse();
		println!("Reversed Linked List is {}", list);
		for i in 0..original_vec.len(){
			assert_eq!(reverse_vec[i],*list.get(i as i32).unwrap());
		}
	}
}