pub mod avl_node {
    /* use std::cmp::Ordering; */

    use std::mem::{replace, swap};

    pub type Tree<T> = Option<Box<Node<T>>>;

    pub struct Node<T: Ord> {
        pub left: Tree<T>,
        pub right: Tree<T>,
        pub data: T,
        pub balance_fac: i8,
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

        fn update_balance_fac(&mut self) {
            if self.left.is_some() && self.right.is_none() {
                self.balance_fac = -1;
            } else if self.left.is_none() && self.right.is_some() {
                self.balance_fac = 1;
            } else {
                let right_node = self.left.as_ref().unwrap();
                let left_node = self.left.as_ref().unwrap();

                self.balance_fac = right_node.balance_fac - left_node.balance_fac;
            }
        }

        /// return true wether tree has a left child or false whether it does not have  
        ///
        ///               (D)             (B)    
        ///              /  \    =>      /  \        
        ///            (B)  (E)        (A)  (D)
        ///            / \                  / \       
        ///          (A) (C)              (C) (E)                   

        pub fn rotation_right(&mut self) -> bool {
            if self.left.is_none() {
                return false;
            }

            //Take nodes A and C
            let left_node = self.left.as_mut().unwrap();
            let left_right_tree = left_node.right.take();
            let left_left_tree = left_node.left.take();

            //Link A node to left node (B)
            let mut new_right_tree = replace(&mut self.left, left_left_tree);
            // Swap B and D node value to avoid moving the root
            swap(&mut self.data, &mut new_right_tree.as_mut().unwrap().data);
            //Take node E
            let right_tree = self.right.take();

            // Link C and E nodes to swapped D node
            let new_right_node = new_right_tree.as_mut().unwrap();
            new_right_node.left = left_right_tree;
            new_right_node.right = right_tree;
            // 6. Link swapped D node to root right node
            self.right = new_right_tree;

            //update_balance_fac only for B(Root) and D(Right_Node)
            self.update_balance_fac();
            let right_node = self.right.as_mut().unwrap();
            right_node.update_balance_fac();

            true
        }

        /// return true wether tree has a right child or false whether it does not have  
        ///
        ///               (D)             (B)    
        ///              /  \    =>      /  \        
        ///           (E)  (B)         (D)  (A)
        ///                / \         / \       
        ///              (C) (A)     (E) (C)                   

        pub fn rotation_left(&mut self) -> bool {
            if self.right.is_none() {
                return false;
            }

            //Take nodes A and C
            let right_node = self.right.as_mut().unwrap();
            let right_right_tree = right_node.right.take();
            let right_left_tree = right_node.left.take();

            //We can use unwrap here because we are assuming they exist
            //Link A node to left node (B)
            let mut new_left_tree = replace(&mut self.right, right_right_tree);
            // Swap B and D node value to avoid moving the root
            swap(&mut self.data, &mut new_left_tree.as_mut().unwrap().data);
            //Take node E
            let left_tree = self.left.take();

            // Link C and E nodes to swapped D node
            let new_left_node = new_left_tree.as_mut().unwrap();
            new_left_node.right = right_left_tree;
            new_left_node.left = left_tree;
            // 6. Link swapped D node to root left node
            self.left = new_left_tree;

            //update_balance_fac only for B(Root) and D(Right_Node)
            self.update_balance_fac();
            let left_node = self.left.as_mut().unwrap();
            left_node.update_balance_fac();

            true
        }

        /// return true wether tree need to rebalance or false whether it does not need
        ///
        /// Case I
        ///
        ///                (X) balance_fac = -1
        ///               /                        (rotation to right)        (Y)  balance_fac = 0
        ///             (Y) balance_fac = -1              =>                  / \                
        ///             /                                                   (Y) (Z) balance_fac = 0
        ///           (Z)  balance_fac = 0
        ///
        /// Case II
        ///
        ///           (X) balance_fac =  1
        ///             \                          (rotation to right)        (Y)  balance_fac = 0
        ///             (Y) balance_fac =  1              =>                  / \                
        ///              \                                                   (X) (Z) balance_fac = 0
        ///              (Z)  balance_fac = 0
        ///      
        /// Case III
        ///
        ///            (ii) (X) balance_fac =  1     (rotation to right) (i)
        ///                  \                       (rotation to left) (ii)        (Z)  balance_fac = 0
        ///             (i) (Y) balance_fac = -1               =>                  / \                
        ///                 /                                                   (X) (Y) balance_fac = 0
        ///               (Z)  balance_fac = 0
        ///
        /// Case IV
        ///
        ///            (ii) (X) balance_fac =  -1    (rotation to left) (i)
        ///                 /                       (rotation to right) (ii)      (Z)  balance_fac = 0
        ///           (i) (Y) balance_fac = 1               =>                    / \                
        ///                \                                                   (Y) (Z) balance_fac = 0
        ///               (Z)  balance_fac = 0
        ///
        pub fn rebalance(&mut self) -> bool {
            match self.balance_fac {
                -1 => {
                    if let Some(ref mut left_node) = self.left {
                        //Root is heavy on left side
                        if left_node.balance_fac == -1 {
                            //Case I
                            self.rotation_right();
                            return true;

                        //Root is also heavy on left side and need double rotation
                        } else if left_node.balance_fac == 1 {
                            //Case IV
                            left_node.rotation_left();
                            self.rotation_right();
                            return true;
                        }
                    }
                    return false;
                }
                1 => {
                    if let Some(ref mut right_node) = self.right {
                        //Root is heavy on right side
                        if right_node.balance_fac == 1 {
                            //Case II
                            self.rotation_left();
                            return true;

                        //Root is also heavy on right side and need double rotation
                        } else if right_node.balance_fac == -1 {
                            //Case III
                            right_node.rotation_right();
                            self.rotation_left();
                            return true;
                        }
                    }
                    return false;
                }

                _ => false,
            }
        }

        /// return some with the next value in ascendent order or None wether tree is empty
        ///
        /// # Examples
        ///   let mut tree = BinaryTree::new();
        ///   tree.add(2);
        ///   tree.add(1);
        ///   tree.add(3);
        ///   tree.add(4);
        ///   assert!(Some(&3), tree.get_next(&2));

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
