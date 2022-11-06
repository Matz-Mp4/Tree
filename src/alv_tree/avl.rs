pub mod avl_set {
    use crate::alv_tree::node::avl_node::Node;
    use std::{cmp::Ordering, collections::VecDeque, fmt::Display};

    type Tree<T> = Option<Box<Node<T>>>;

    ///An ordered AvlTree
    pub struct AvlTree<T: Ord> {
        pub root: Tree<T>,
    }

    impl<T: Ord + Display + Copy> AvlTree<T> {
        ///Create an empty tree
        pub fn new() -> Self {
            Self { root: None }
        }

        ///return true if there are no elements
        pub fn is_empty(&self) -> bool {
            match self.root {
                None => true,
                _ => false,
            }
        }

        /// Show all elemets in the tree in ascending order
        ///
        /// ```compile_fail
        /// # Examples
        ///   let mut tree = BinaryTree::new();
        ///   tree.add(2);
        ///   tree.add(1);
        ///   tree.add(3);
        ///   tree.add(-3);
        ///
        ///   tree.show_in_order()
        ///
        ///   Output: (-3)(1)(2)(3)
        ///
        /// ```
        ///
        pub fn show_in_order(&self, tree: &Tree<T>) {
            if let Some(node) = tree {
                self.show_in_order(&node.left);
                print!("({})", node.data);
                self.show_in_order(&node.right);
            }
        }

        /// Show all elemets in the tree in height level
        ///
        /// ```compile_fail
        /// # Examples
        ///   let mut tree = BinaryTree::new();
        ///   tree.add(2);
        ///   tree.add(1);
        ///   tree.add(3);
        ///   tree.add(4);
        ///
        ///           (2)
        ///           / \
        ///         (1) (3)
        ///              \
        ///              (4)
        ///   tree.show_in_order()
        ///
        ///   Output: (2)(1)(3)(4)
        ///
        /// ```
        ///

        pub fn show_in_level(&self) {
            let mut queue: VecDeque<&Tree<T>> = VecDeque::new();
            let mut current_tree;
            let mut string = String::new();

            if !self.is_empty() {
                queue.push_front(&self.root);
                while !queue.is_empty() {
                    current_tree = queue.pop_front().unwrap();

                    if let Some(current_node) = current_tree {
                        print!(" ({}) ", current_node.data);
                        string.push_str(current_node.data.to_string().as_str());
                        if current_node.left.is_some() {
                            queue.push_back(&current_node.left);
                        }

                        if current_node.right.is_some() {
                            queue.push_back(&current_node.right);
                        }
                    }
                }
            }
        }

        /// return some with the maximum value in tree or None wether tree is empty
        ///
        /// ```compile_fail
        /// # Examples
        ///   let mut tree = BinaryTree::new();
        ///   tree.add(2);
        ///   tree.add(1);
        ///   tree.add(3);
        ///   assert!(Some(&3), tree.get_maximum());
        /// ```

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

        /// return some with the minimum value in tree or None wether tree is empty
        ///
        /// ```compile_fail
        /// # Examples
        ///   let mut tree = BinaryTree::new();
        ///   tree.add(2);
        ///   tree.add(1);
        ///   tree.add(3);
        ///   assert!(Some(&1), tree.get_minimum());
        /// ```

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

        /// return some with the  value  or None wether tree is empty
        pub fn get(&mut self, value: &T) -> Option<&T> {
            let node = Self::search(self, &value);

            match node {
                Some(ptr_node) => Some(ptr_node.get_data()),
                None => None,
            }
        }

        /// return the amount of elemets  
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

        /// remove all the elements deallocating from memory
        ///
        /// ```compile_fail
        ///  # Examples
        ///  tree.add(2);
        ///  tree.add(3);
        ///  tree.add(1);
        ///
        ///  tree.clear();
        ///
        /// assert_eq!(None, tree.get(&3));
        /// assert_eq!(None, tree.get(&2));
        /// assert_eq!(None, tree.get(&1));
        /// ```

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

        pub fn add(&mut self, value: T) -> bool {
            let mut current_tree = &mut self.root;
            let mut parents_nodes = Vec::<*mut Node<T>>::new();

            while let Some(current_node) = current_tree {
                // Converting a mutable reference to a pointer
                //allow one secondary mutable reference after insertion
                //if we use let mut parents_nodes = Vec::<&mut Node<T>>::new();
                //we could not to push because of Borrow checker (borrow twice a mutable variable)
                parents_nodes.push(&mut **current_node);
                match current_node.data.cmp(&value) {
                    Ordering::Less => {
                        /* current_node.balance_fac += 1; */
                        current_tree = &mut current_node.right;
                    }
                    Ordering::Equal => {
                        return false;
                    }

                    Ordering::Greater => {
                        /* current_node.balance_fac += -1; */
                        current_tree = &mut current_node.left;
                    }
                }
            }
            *current_tree = Some(Box::new(Node::new(value)));

            let mut check_parent = false;
            let mut old_balance_fac;

            //We could change the type of Tree to Option<rc<RefCell<T>>>
            //to allow multiples mutable pointers but the complexity to manage would
            //increase and instead, I rather use unsafe to simplify

            //When a rotation occurs we don't need to check the remian nodes
            //It's similar to this f(f⁻¹(x)) = x => we change the balance_fac
            //and the rotation makes inverse process. So all the nodes remain will have
            //the same balance_fac after the insertion => !check_parent
            while !check_parent && !parents_nodes.is_empty() {
                //We only can derefence a raw pointer in unsafe rust
                let node = unsafe {
                    // Converting a mutable pointer back to a reference
                    &mut *(parents_nodes.pop().unwrap())
                };
                old_balance_fac = node.update_balance_fac(&value);
                check_parent = node.rebalance(old_balance_fac);
            }
            true
        }
    }
}
