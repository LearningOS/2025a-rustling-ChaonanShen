/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/

//I AM NOT DONE
use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    // 不断左右找，找到应插入节点，然后调用TreeNode::insert插入
    fn insert(&mut self, value: T) {
        match self.root.as_mut() {
            None => {
                self.root = Some(Box::new(TreeNode::new(value)));
            },
            Some(root_node) => {
                root_node.insert(value);
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        match self.root.as_ref() {
            None => {
                false
            },
            Some(mut root_node) => {
                loop {
                    let  root_value = &root_node.value;
                    if *root_value == value {
                        return true;
                    } else if *root_value < value {
                        match root_node.left.as_ref() {
                            None => {
                                return false;
                            },
                            Some(root_left_value) => { // 继续向左子树搜索
                                root_node = root_left_value;
                            },
                        }
                    } else {
                        match root_node.right.as_ref() {
                            None => {
                                return false;
                            },
                            Some(root_right_value) => {
                                root_node = root_right_value;
                            }
                        }
                    }
                }
            },
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    pub fn insert(&mut self, value: T) {
        //TODO
        let node_value = &self.value;
        if *node_value == value {
            // 不允许重复插入，直接返回
            return;
        } if *node_value < value { // 插入左子树
            match self.left.as_mut() {
                None => { // 没有左子树，直接插入
                    self.left = Some(Box::new(TreeNode::new(value)));
                },
                Some(left_node) => { // 递归插入左子树
                    left_node.insert(value);
                }
            }
        } else { // 插入右子树
            match self.right.as_mut() {
                None => {
                    self.right = Some(Box::new(TreeNode::new(value)));
                },
                Some(right_node) => {
                    right_node.insert(value);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


