#[allow(dead_code)]
pub mod bst {
    use std::collections::VecDeque;
    use std::mem::replace;

    use std::{cmp::Ordering, fmt::Display};

    type Tree<T> = Option<Box<Node<T>>>;

    pub struct Node<T: Ord> {
        left: Tree<T>,
        right: Tree<T>,
        data: T,
    }
    pub struct BinaryTree<T: Ord> {
        pub root: Tree<T>,
    }

    impl<T: Ord + Display> BinaryTree<T> {
        pub fn new() -> Self {
            Self { root: None }
        }

        pub fn is_empty(&self) -> bool {
            match self.root {
                None => true,
                _ => false,
            }
        }
        pub fn add(&mut self, value: T) -> bool {
            let mut current_tree = &mut self.root;

            while let Some(current_node) = current_tree {
                match current_node.data.cmp(&value) {
                    Ordering::Less => current_tree = &mut current_node.right,
                    Ordering::Equal => {
                        return false;
                    }
                    Ordering::Greater => current_tree = &mut current_node.left,
                }
            }
            *current_tree = Some(Box::new(Node {
                data: value,
                left: None,
                right: None,
            }));

            true
        }
        fn find_parent(&mut self, value: &T) -> Option<&mut Box<Node<T>>> {
            if !self.is_empty() {
                let mut current_tree = &mut self.root;

                while let Some(current_node) = current_tree {
                    if let Some(right_node) = &current_node.right {
                        if right_node.data.cmp(&value) == Ordering::Equal {
                            return Some(current_node);
                        }
                    } else if let Some(left_node) = &current_node.left {
                        if left_node.data.cmp(&value) == Ordering::Equal {
                            return Some(current_node);
                        }
                    }
                    match current_node.data.cmp(&value) {
                        Ordering::Less => current_tree = &mut current_node.right,
                        Ordering::Equal => {
                            return Some(current_node);
                        }
                        Ordering::Greater => current_tree = &mut current_node.left,
                    }
                }
            }
            None
        }
        fn find(&mut self, value: &T) -> Option<&mut Box<Node<T>>> {
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
            let node = Self::find(self, &value);

            match node {
                Some(ptr_node) => Some(&ptr_node.data),
                None => None,
            }
        }
        pub fn get_amount_nodes(&self) -> i32 {
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

        pub fn clear(&mut self) {
            let mut queue: VecDeque<Tree<T>> = VecDeque::new();
            queue.push_front(self.root.take());
            while let Some(mut current_tree) = queue.pop_front() {
                if let Some(mut current_node) = current_tree.take() {
                    if current_node.left.is_some() {
                        queue.push_front(current_node.left.take());
                    }

                    if current_node.right.is_some() {
                        queue.push_front(current_node.right.take());
                    }
                }
            }
        }

        pub fn remove(&mut self, value: &T) -> bool {
            let parent_tree = self.find_parent(value);
            let mut veri = false;
            let mut on_right = false;

            if let Some(parent_node) = parent_tree {
                let target_tree;

                //Defining the node that need to be removed
                if let Some(right_node) = &mut parent_node.right {
                    if right_node.data.cmp(value) == Ordering::Equal {
                        target_tree = Some(right_node);
                        on_right = true;
                    } else {
                        target_tree = Some(parent_node);
                    }
                } else if let Some(left_node) = &mut parent_node.left {
                    if left_node.data.cmp(value) == Ordering::Equal {
                        target_tree = Some(left_node);
                        on_right = true;
                    } else {
                        target_tree = Some(parent_node);
                    }
                } else {
                    target_tree = Some(parent_node);
                }

                if let Some(target_node) = target_tree {
                    //There are only a child on left
                    if target_node.left.is_some() && target_node.right.is_none() {
                        if let Some(left_node) = target_node.left.take() {
                            let _x = replace(target_node, left_node);
                        }
                    //There are only a child on left
                    } else if target_node.left.is_none() && target_node.right.is_some() {
                        if let Some(right_node) = target_node.right.take() {
                            let _x = replace(target_node, right_node);
                        }
                    //There are no child
                    } else if target_node.left.is_none() && target_node.right.is_none() {
                        if parent_node.data.cmp(value) == Ordering::Equal {
                            self.root = None;
                        } else if on_right == true {
                            parent_node.right.take();
                        } else {
                            parent_node.left.take();
                        }
                        //There are 2 children
                    } else {
                        //Getting the most lef-right node
                        let mut most_left_right = &mut target_node.left;
                        let mut ver = false;
                        let mut has_left_child = false;

                        while ver == false {
                            if let Some(node_left_right) = most_left_right {
                                if node_left_right.right.is_none() {
                                    has_left_child = node_left_right.left.is_some();
                                    ver = true;
                                }
                            }
                            //Because of Borrow Checker
                            if ver == false {
                                if let Some(node_left_right) = most_left_right {
                                    most_left_right = &mut node_left_right.right;
                                }
                            }
                        }

                        let data = {
                            if has_left_child == true {
                                let left_tree = most_left_right.as_mut().unwrap().left.take();
                                let _temp = replace(most_left_right, left_tree);
                                _temp.unwrap().data
                            } else {
                                most_left_right.take().unwrap().data
                            }
                        };

                        let _x = replace(&mut target_node.data, data);
                    }
                    veri = true;
                }
            }
            veri
        }

        pub fn show_in_order(&self, tree: &Tree<T>) {
            if let Some(node) = tree {
                self.show_in_order(&node.left);
                print!("({})", node.data);
                self.show_in_order(&node.right);
            }
        }
    }

    pub struct BinaryTreeIterator<'a, T: Ord> {
        prev_nodes: Vec<&'a Node<T>>,
        current_tree: &'a Tree<T>,
    }

    impl<'a, T: 'a + Ord> Iterator for BinaryTreeIterator<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            let mut data: Option<Self::Item> = None;
            let mut veri = false;

            while veri == false {
                match *self.current_tree {
                    None => {
                        data = match self.prev_nodes.pop() {
                            None => {
                                veri = true;
                                None
                            }

                            Some(ref prev_node) => {
                                veri = true;
                                self.current_tree = &prev_node.right;
                                Some(&prev_node.data)
                            }
                        };
                    }

                    Some(ref current_node) => {
                        if veri == false {
                            if current_node.left.is_some() {
                                self.prev_nodes.push(&current_node);
                                self.current_tree = &current_node.left;
                            } else if current_node.right.is_some() {
                                veri = true;
                                self.current_tree = &current_node.right;
                                data = Some(&current_node.data);
                            } else {
                                data = Some(&current_node.data);
                                veri = true;
                                self.current_tree = &None;
                            }
                        }
                    }
                }
            }

            data
        }
    }

    impl<'a, T: Ord + Display> BinaryTree<T> {
        pub fn iter(&'a self) -> BinaryTreeIterator<'a, T> {
            BinaryTreeIterator {
                prev_nodes: Vec::new(),
                current_tree: &self.root,
            }
        }
    }
}
