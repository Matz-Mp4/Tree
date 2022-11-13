use std::{
    cmp::Ordering,
    fmt::Display,
    mem::{replace, swap},
};

pub type Tree<T> = Option<Box<Node<T>>>;

pub struct Node<T: Ord> {
    pub left: Tree<T>,
    pub right: Tree<T>,
    pub data: T,
    pub balance_fac: i8,
}

impl<T: Ord + Display> Node<T> {
    pub fn new(data: T) -> Self {
        Self {
            left: None,
            right: None,
            data,
            //right_height - left_height
            balance_fac: 0,
        }
    }

    pub fn update_balance_fac_insert(&mut self, value_insert: &T) -> i8 {
        let old_balance_fac = self.balance_fac.clone();

        if self.data.cmp(value_insert) == Ordering::Greater {
            self.balance_fac += -1;
        } else if self.data.cmp(value_insert) == Ordering::Less {
            self.balance_fac += 1;
        }
        old_balance_fac
    }

    pub fn update_balance_fac_remove(&mut self, value_insert: &T) -> i8 {
        let old_balance_fac = self.balance_fac.clone();

        let result = self.data.cmp(value_insert);
        if result == Ordering::Greater {
            self.balance_fac += 1;
        } else if result == Ordering::Less || result == Ordering::Equal {
            self.balance_fac += -1;
        }
        old_balance_fac
    }

    /// return true wether tree has a left child or false whether it does not have  
    ///
    /// ```compile_fail
    /// # Examples
    ///   let mut tree = AvlTree::new();
    ///   tree.add(4);
    ///   tree.add(2);
    ///   tree.add(5);
    ///   tree.add(1);
    ///   tree.add(3);
    ///
    ///   let mut node_root = &mut tree.root.unwrap();
    ///
    ///   node_root.rotation_right();
    ///   assert_eq!(2, node_root.data);
    ///
    ///               (4)             (2)    
    ///              /  \    =>      /  \        
    ///            (2)  (5)        (1)  (4)
    ///            / \                  / \       
    ///          (1) (3)              (3) (5)                   
    /// ```

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
        true
    }

    /// return true wether tree has a right child or false whether it does not have  
    /// ```compile_fail
    ///   let mut tree = AvlTree::new();
    ///   tree.add(2);
    ///   tree.add(4);
    ///   tree.add(1);
    ///   tree.add(3);
    ///   tree.add(5);
    ///
    ///   let mut node_root = &mut tree.root.unwrap();
    ///
    ///   assert_eq!(4, node_root.data);
    ///
    ///               (2)             (4)
    ///              /  \    =>      /  \
    ///           (1)  (4)         (2)  (5)
    ///                / \         / \
    ///              (3) (5)     (1) (3)
    /// ```

    pub fn rotation_left(&mut self) -> bool {
        if self.right.is_none() {
            return false;
        }

        //Take nodes 3 and 5
        let right_node = self.right.as_mut().unwrap();
        let right_right_tree = right_node.right.take();
        let right_left_tree = right_node.left.take();

        //We can use unwrap here because we are assuming they exist
        //Link 5 node to left node (4)
        let mut new_left_tree = replace(&mut self.right, right_right_tree);
        // Swap B and D node value to avoid moving the root
        swap(&mut self.data, &mut new_left_tree.as_mut().unwrap().data);
        //Take node 1
        let left_tree = self.left.take();

        // Link 1 and 1 nodes to swapped 2 node
        let new_left_node = new_left_tree.as_mut().unwrap();
        new_left_node.right = right_left_tree;
        new_left_node.left = left_tree;
        // 6. Link swapped 2 node to root left node
        self.left = new_left_tree;
        true
    }

    /// return true wether tree need to rebalance or false whether it does not need
    // ```
    // Case I
    //
    //         (X) balance_fac = -2
    //         /                        (rotation to right)        (Y)  balance_fac = 0
    //       (Y) balance_fac = -1              =>                  / \
    //       /                                                   (Y) (Z) balance_fac = 0
    //     (Z)  balance_fac = 0
    //
    //     (X) balance_fac =  2
    //      \                          (rotation to right)        (Y)  balance_fac = 0
    //      (Y) balance_fac =  1              =>                  / \
    //       \                                                  (X) (Z) balance_fac = 0
    //       (Z)  balance_fac = 0
    //
    // Case III
    //
    //  (ii) (X) balance_fac =  2     (rotation to right) (i)
    //         \                       (rotation to left) (ii)        (Z)  balance_fac = 0
    //    (i) (Y) balance_fac = -1               =>                  / \
    //         /                                                   (X) (Y) balance_fac = 0
    //       (Z)  balance_fac = 0
    //
    // Case IV
    //
    //     (ii) (X) balance_fac =  -2    (rotation to left) (i)
    //          /                       (rotation to right) (ii)      (Z)  balance_fac = 0
    //    (i) (Y) balance_fac = 1               =>                    / \
    //          \                                                   (Y) (Z) balance_fac = 0
    //         (Z)  balance_fac = 0
    // ```
    //
    pub fn rebalance(&mut self, old_balance_fac: i8) -> bool {
        match self.balance_fac {
            -2 => {
                if let Some(ref mut left_node) = self.left {
                    //Root is heavy on left side
                    if left_node.balance_fac == -1 {
                        //Case I
                        self.balance_fac = 0;
                        left_node.balance_fac = 0;
                        self.rotation_right();
                        return true;

                    //Root is also heavy on left side and need double rotation
                    } else if left_node.balance_fac == 1 {
                        //Case IV
                        self.balance_fac = 0;
                        left_node.balance_fac = 0;
                        let left_right_node = left_node.right.as_mut().unwrap();
                        left_right_node.balance_fac = 0;

                        left_node.rotation_left();
                        self.rotation_right();
                        return true;
                    } else {
                        self.balance_fac = old_balance_fac;
                    }
                }

                return false;
            }
            2 => {
                if let Some(ref mut right_node) = self.right {
                    //Root is heavy on right side
                    if right_node.balance_fac == 1 {
                        //Case II
                        self.balance_fac = 0;
                        right_node.balance_fac = 0;
                        self.rotation_left();
                        return true;

                    //Root is also heavy on right side and need double rotation
                    } else if right_node.balance_fac == -1 {
                        //Case III
                        self.balance_fac = 0;
                        right_node.balance_fac = 0;
                        let right_left_node = right_node.left.as_mut().unwrap();
                        right_left_node.balance_fac = 0;
                        right_node.rotation_right();
                        self.rotation_left();

                        return true;
                    } else {
                        self.balance_fac = old_balance_fac;
                    }
                }

                return false;
            }

            _ => false,
        }
    }

    /// return true wether tree need to rebalance or false whether it does not need
    //
    // h = height
    // b = balance factor
    //
    // Case I
    //
    // b = -2    (X)                                            (Y)     b = 0
    //          /  \              (rotation to right)           / \
    // b = -1  (Y) [h-1]                    =>                 /  (X)   b = 0
    //        /  \                                             /   / \
    //       /   [h-1]                                       [h] [h-1] [h-1]
    //      /
    //    [h]
    //
    // Case II
    //
    //           (X) b = 2                                       b = 0    (Y)
    //          /  \              (rotation to left)                      / \
    //       [h-1] (Y) b = 1             =>                   b = 0     (X)  \
    //             / \                                                 / \    \
    //        [h-1]   \                                           [h-1] [h-1] [h]
    //                 \
    //                 [h]
    //
    //
    // Case III
    //
    // b = -2     (X)                                                 b = 0  (Z)
    //            / \                                                       /  \
    // b = 1   (Y)  [h-1]         (rotation to left)              b = 0  (Y)   (X) b = 0
    //         / \                (rotation to right)                   /  \   /  \
    //    [h-1]  (Z)                     =>                        [h-1][h-1][h-1][h-1]
    //           / \
    //       [h-1] [h-1]
    //
    //
    // Case IV
    //
    //       (X)     b = 2                                      b = 0  (Z)
    //      /  \                                                      /  \
    //   [h-1] (Y)   b = -1       (rotation to right)        b = 0  (X)   (Y)  b = 0
    //         / \                (rotation to left)               /  \   /  \
    //       (Z) [h-1]                    =>                  [h-1][h-1][h-1][h-1]
    //       / \
    //  [h-1] [h-1]
    //
    //
    //  Case V
    //
    // b = -2    (X)                                            (Y)     b = 1
    //          /  \              (rotation to right)           / \
    // b =  0  (Y) [h-1]                    =>                 /  (X)   b = -1
    //        /  \                                            /   / \
    //       /    \                                         [h]  / [h-1]
    //      /      \                                            /
    //    [h]      [h]                                         [h]
    //
    // Case VI
    //
    //           (X) b = 2                                       b = -1    (Y)
    //          /  \              (rotation to left)                      / \
    //       [h-1] (Y) b = 0             =>                   b = 1     (X)  \
    //             / \                                                 / \    \
    //            /   \                                           [h-1]   \   [h]
    //           /     \                                                   \  
    //          [h]    [h]                                                [h]  
 

    pub fn rebalance_remove(&mut self) -> bool {
        match self.balance_fac {
            -2 => {
                if let Some(ref mut left_node) = self.left {
                    //Root is heavy on left side
                    if left_node.balance_fac == -1 {
                        //Case I
                        self.balance_fac = 0;
                        left_node.balance_fac = 0;
                        self.rotation_right();
                        return true;

                    //Root is also heavy on left side and need double rotation
                    } else if left_node.balance_fac == 1 {
                        //Case IV
                        self.balance_fac = 0;
                        left_node.balance_fac = 0;
                        let left_right_node = left_node.right.as_mut().unwrap();
                        left_right_node.balance_fac = 0;

                        left_node.rotation_left();
                        self.rotation_right();
                        return true;
                    } else if left_node.balance_fac == 0 {
                        //Case V
                        self.balance_fac = 1;
                        left_node.balance_fac = -1;
                        self.rotation_right();
                        return true;
                    }
                }

                return false;
            }
            2 => {
                if let Some(ref mut right_node) = self.right {
                    //Root is heavy on right side
                    if right_node.balance_fac == 1 {
                        //Case II
                        self.balance_fac = 0;
                        right_node.balance_fac = 0;
                        self.rotation_left();
                        return true;

                    //Root is also heavy on right side and need double rotation
                    } else if right_node.balance_fac == -1 {
                        //Case III
                        self.balance_fac = 0;
                        right_node.balance_fac = 0;
                        let right_left_node = right_node.left.as_mut().unwrap();
                        right_left_node.balance_fac = 0;
                        right_node.rotation_right();
                        self.rotation_left();

                        return true;
                    } else if right_node.balance_fac == 0 {
                        //Case VI
                        self.balance_fac = -1;
                        right_node.balance_fac = 1;
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
    /// ```compile_fail
    ///   let mut tree = AvlTree::new();  
    ///   tree.add(2);  
    ///   tree.add(1);
    ///   tree.add(3);
    ///   tree.add(4);
    ///
    ///   let root_node = &tree.root;
    ///
    ///   assert_eq!(Some(&3), root.get_next().unwrap());
    /// ```
    pub fn get_next(&mut self) -> &mut Tree<T> {
        let mut next = &mut self.left;
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
                    next = &mut current_node.right;
                }
            }
        }
        next
    }

    pub fn get_data(&self) -> &T {
        &self.data
    }
}
