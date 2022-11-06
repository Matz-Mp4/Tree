pub mod avl_iterator {
    use std::iter::Map;

    use crate::alv_tree::{
        avl::avl_set::AvlTree,
        node::avl_node::{Node, Tree},
    };

    pub struct AvlNodeIterator<'a, T: Ord> {
        prev_nodes: Vec<&'a Node<T>>,
        current_tree: &'a Tree<T>,
    }

    impl<'a, T: 'a + Ord> Iterator for AvlNodeIterator<'a, T> {
        type Item = &'a Node<T>;

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
                                Some(&prev_node)
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
                                data = Some(&current_node);
                            } else {
                                data = Some(&current_node);
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
        pub fn node_iter(&'a self) -> AvlNodeIterator<'a, T> {
            AvlNodeIterator {
                prev_nodes: Vec::new(),
                current_tree: &self.root,
            }
        }

        ///Return all the elements of the tree inside of a iterator
        ///
        /// ```compile_fail
        /// # Examples
        ///
        /// let mut tree = AvlTree::new();
        ///
        ///  for i in 1..5 {
        ///     tree.add(i);
        ///  }
        ///  let tree_iter = tree.iter();
        ///
        ///  for i in tree_iter {
        ///     print!("{} ", i);
        ///  }
        ///
        ///  Output: 1 2 3 4
        /// ```

        pub fn iter(&'a self) -> Map<AvlNodeIterator<'a, T>, fn(&'a Node<T>) -> &T> {
            /*
                Takes a closure and creates an iterator which calls that closure on element.
                
                `map()` transforms one iterator into another, by means of its argument:
                something that implements [`FnMut`]. It produces a new iterator which
                calls this closure on each element of the original iterator.
            */

            self.node_iter().map(|node| &node.data)
        }
    }
}
