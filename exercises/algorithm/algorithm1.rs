/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/
use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
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

impl<T: std::cmp::PartialOrd> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: std::cmp::PartialOrd> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> u32 {
        self.length
    }

    // 给外界用的，向末尾加入一个元素
    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        // Box::into_raw将Box<T>转换为裸指针*mut T，转移所有权，之后允许用户手动管理这个指针(这里用NonNull管理)，最终用Box::from_raw回收
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });  // node_ptr是个Option<NonNull<xx>>
        
        // self.start/end为None时（也即空链表），双方都指向node_ptr；不为空时就插入末尾（self.end指向该新节点）
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr }, // end_ptr就是个NonNull，而这就是NonNull访问其中元素val/next的办法
        }
        self.end = node_ptr;

        self.length += 1;
    }

    // 直接拿出一个节点
    fn pop_front_node(&mut self) -> Option<NonNull<Node<T>>> {
        match self.start {
            None => None, // 空链表
            Some(start_ptr) => { // end_ptr是NonNull<xx>
                self.start = unsafe { (*start_ptr.as_ptr()).next };
                self.length -= 1;
                if self.length == 0 {
                    self.end = None;
                }
                Some(start_ptr)
            },
        }
    }

    fn push_back_node(&mut self, node_ptr: Option<NonNull<Node<T>>>) {
        let new_end_node = node_ptr.unwrap(); // NonNull<xx> 
        if self.is_empty() { // 没有节点，直接插入
            self.start = node_ptr;
        } else {
            let end_node = self.end.unwrap();
            unsafe { (*end_node.as_ptr()).next = node_ptr; }
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    // 递归查找 - 注意只是返回值的引用
    fn get_ith_node(&self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    // 两个ordered list合并成一个ordered list
    // 注意这里传入list_a/list_b不是引用，所以直接有所有权转移
    // 把两个list中NonNull节点一个个拿出来，然后插入新链表中
	pub fn merge(mut list_a:LinkedList<T>, mut list_b:LinkedList<T>) -> Self
	{
        if list_a.is_empty() {
            return list_b;
        } else if list_b.is_empty() {
            return list_a;
        }

        let mut list = LinkedList::<T>::new();

		while !list_a.is_empty() && !list_b.is_empty() { // 两个都非空，就比大小
            let na = list_a.get(0).unwrap();
            let nb = list_b.get(0).unwrap();
            if na < nb {
                list.push_back_node(list_a.pop_front_node());
            } else {
                list.push_back_node(list_b.pop_front_node());
            }
        }

        while !list_a.is_empty() { // 把list_b元素一个个插入
            list.push_back_node(list_a.pop_front_node());
        }

        while !list_b.is_empty() {
            list.push_back_node(list_b.pop_front_node());
        }

        list
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
    fn test_merge_linked_list_with_empty_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![];
		let target_vec = vec![1,3,5,7];  

 		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}       
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
    }

    #[test]
    fn test_merge_linked_list_with_empty_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![];
		let vec_b = vec![1,3,5,7];
		let target_vec = vec![1,3,5,7];  

 		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}       
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let list_c = LinkedList::<i32>::merge(list_a,list_b);
        assert_eq!(target_vec.len(), list_c.len() as usize);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}

	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let list_c = LinkedList::<i32>::merge(list_a,list_b);
        assert_eq!(target_vec.len(), list_c.len() as usize);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}