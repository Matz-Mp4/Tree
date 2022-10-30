pub mod avl_set {
    use crate::alv_tree::node::avl_node::Node;
    use std::{cmp::Ordering, collections::VecDeque};

    type Tree<T> = Option<Box<Node<T>>>;

    pub struct AvlTree<T: Ord> {
        root: Tree<T>,
    }

    impl<T: Ord> AvlTree<T> {
        pub fn new() -> Self {
            Self { root: None }
        }

        pub fn is_empty(&self) -> bool {
            match self.root {
                None => true,
                _ => false,
            }
        }

        pub fn get_maximum(&self) -> Option<&T> {
            let mut current_tree = &self.root;
            let mut veri = false;

            while veri == false {
                if let Some(current_node) = current_tree {
                    if current_node.right.is_none() {
                        veri = true;
                    }
                }

                if veri == false {
                    if let Some(current_node) = current_tree {
                        current_tree = &current_node.right;
                    }
                }
            }

            if let Some(current_node) = current_tree {
                return Some(&current_node.data);
            }

            None
        }

        pub fn get_minimum(&self) -> Option<&T> {
            let mut current_tree = &self.root;
            let mut veri = false;

            while veri == false {
                if let Some(current_node) = current_tree {
                    if current_node.left.is_none() {
                        veri = true;
                    }
                }

                if veri == false {
                    if let Some(current_node) = current_tree {
                        current_tree = &current_node.left;
                    }
                }
            }

            if let Some(current_node) = current_tree {
                return Some(&current_node.data);
            }

            None
        }

        fn search(&mut self, value: &T) -> Option<&mut Box<Node<T>>> {
            if !self.is_empty() {
                let mut current_tree = &mut self.root;

                while let Some(current_node) = current_tree {
                    match current_node.data.cmp(&value) {
                        Ordering::Less => {
                            current_tree = &mut current_node.right;
                        }
                        Ordering::Equal => {
                            return Some(current_node);
                        }
                        Ordering::Greater => {
                            current_tree = &mut current_node.left;
                        }
                    }
                }
            }
            None
        }
        pub fn get(&mut self, value: &T) -> Option<&T> {
            let node = Self::search(self, &value);

            match node {
                Some(ptr_node) => Some(ptr_node.get_data()),
                None => None,
            }
        }

        pub fn len(&self) -> i32 {
            let mut amount = 0;
            let mut queue: VecDeque<&Tree<T>> = VecDeque::new();
            let mut current_tree;

            if !self.is_empty() {
                queue.push_front(&self.root);
                amount = 1;
                while !queue.is_empty() {
                    current_tree = queue.pop_front().unwrap();

                    if let Some(current_node) = current_tree {
                        if current_node.left.is_some() {
                            queue.push_front(&current_node.left);
                            amount = amount + 1;
                        }

                        if current_node.right.is_some() {
                            queue.push_front(&current_node.right);
                            amount = amount + 1;
                        }
                    }
                }
            }
            amount
        }
    }
}
