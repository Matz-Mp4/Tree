pub mod avl_iterator {
    use crate::alv_tree::{
        avl::avl_set::AvlTree,
        node::avl_node::{Node, Tree},
    };

    pub struct AvlIterator<'a, T: Ord> {
        prev_nodes: Vec<&'a Node<T>>,
        current_tree: &'a Tree<T>,
    }

    impl<'a, T: 'a + Ord> Iterator for AvlIterator<'a, T> {
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

    impl<'a, T: Ord> AvlTree<T> {
        pub fn iter(&'a self) -> AvlIterator<'a, T> {
            AvlIterator {
                prev_nodes: Vec::new(),
                current_tree: &self.root,
            }
        }
    }
}
