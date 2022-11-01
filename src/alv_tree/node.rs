pub mod avl_node {
    /* use std::cmp::Ordering; */

    use std::mem::{replace, swap};

    type Tree<T> = Option<Box<Node<T>>>;

    pub struct Node<T: Ord> {
        pub left: Tree<T>,
        pub right: Tree<T>,
        pub data: T,
        pub balance_fac: usize,
    }

    impl<T: Ord> Node<T> {
        pub fn new(data: T) -> Self {
            Self {
                left: None,
                right: None,
                data,
                balance_fac: 0,
            }
        }

        pub fn rotation_right(&mut self) -> bool {
            if self.left.is_none() {
                return false;
            }

            let left_node = self.left.as_mut().unwrap();
            let left_right_tree = left_node.right.take();
            let left_left_tree = left_node.left.take();

            let mut new_right_tree = replace(&mut self.left, left_left_tree);
            swap(&mut self.data, &mut new_right_tree.as_mut().unwrap().data);

            let right_tree = self.right.take();

            let new_right_node = new_right_tree.as_mut().unwrap();
            new_right_node.left = left_right_tree;
            new_right_node.right = right_tree;

            self.right = new_right_tree;

            true
        }
        pub fn get_next(&self) -> &Tree<T> {
            let mut next = &self.left;
            let mut ver = false;

            while ver == false {
                if let Some(current_node) = next {
                    if current_node.right.is_none() {
                        ver = true;
                    }
                }
                //Because of Borrow Checker
                if ver == false {
                    if let Some(current_node) = next {
                        next = &current_node.right;
                    }
                }
            }
            next
        }

        pub fn get_data(&self) -> &T {
            &self.data
        }
    }
}
