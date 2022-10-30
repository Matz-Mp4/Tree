pub mod avl_node {
    /* use std::cmp::Ordering; */

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
