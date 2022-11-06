#[cfg(test)]
mod tests {

    /* use tree::alv_tree::avl::avl_set::AvlTree; */

    mod avl {}

    mod node {
        /* use std::cmp::Ordering; */
        /* use tree::alv_tree::{avl::avl_set::AvlTree, node::avl_node::Node}; */
/* 
        fn add_no_update_balance_fac(tree: &mut AvlTree<i32>, value: i32) {
            let mut current_tree = &mut tree.root;

            while let Some(current_node) = current_tree {
                match current_node.data.cmp(&value) {
                    Ordering::Less => current_tree = &mut current_node.right,
                    Ordering::Equal => {
                        return;
                    }
                    Ordering::Greater => current_tree = &mut current_node.left,
                }
            }
            *current_tree = Some(Box::new(Node {
                data: value,
                left: None,
                right: None,
                balance_fac: 0,
            }));
        }

        pub fn add_no_rebalance(tree: &mut AvlTree<i32>, value: i32) {
            let mut current_tree = &mut tree.root;
            let mut parents_nodes = Vec::<*mut Node<i32>>::new();

            while let Some(current_node) = current_tree {
                parents_nodes.push(&mut **current_node);
                match current_node.data.cmp(&value) {
                    Ordering::Less => {
                        current_tree = &mut current_node.right;
                    }
                    Ordering::Equal => {
                        return;
                    }
                    Ordering::Greater => current_tree = &mut current_node.left,
                }
            }
            *current_tree = Some(Box::new(Node::new(value)));
            for current_node in parents_nodes.into_iter().rev() {
                let node = unsafe {
                    &mut *current_node // Converting a mutable pointer back to a reference
                };
                node.update_balance_fac();
            }
        }

        #[test]
        fn update_balance_fac_no_child() {
            let mut tree: AvlTree<i32> = AvlTree::new();
            add_no_update_balance_fac(&mut tree, 1);
            let root_node = &mut tree.root.unwrap();
            root_node.update_balance_fac();
            assert_eq!(0, root_node.balance_fac);
        }

        #[test]
        fn update_balance_fac_left_child() {
            let mut tree: AvlTree<i32> = AvlTree::new();
            add_no_update_balance_fac(&mut tree, 1);
            add_no_update_balance_fac(&mut tree, 0);
            let root_node = &mut tree.root.unwrap();
            root_node.update_balance_fac();
            assert_eq!(-1, root_node.balance_fac);
        }

        #[test]
        fn update_balance_fac_right_child() {
            let mut tree: AvlTree<i32> = AvlTree::new();
            add_no_update_balance_fac(&mut tree, 1);
            add_no_update_balance_fac(&mut tree, 2);
            let root_node = &mut tree.root.unwrap();
            root_node.update_balance_fac();
            assert_eq!(1, root_node.balance_fac);
        }

        #[test]
        fn update_balance_fac_left_and_right_children() {
            let mut tree: AvlTree<i32> = AvlTree::new();
            add_no_update_balance_fac(&mut tree, 1);
            add_no_update_balance_fac(&mut tree, 2);
            add_no_update_balance_fac(&mut tree, 0);
            let root_node = &mut tree.root.unwrap();
            root_node.update_balance_fac();
            assert_eq!(0, root_node.balance_fac);
        }

        #[test]
        fn rotation_right() {
            let mut tree: AvlTree<i32> = AvlTree::new();
            add_no_update_balance_fac(&mut tree, 2);
            add_no_update_balance_fac(&mut tree, 1);
            add_no_update_balance_fac(&mut tree, 0);
            let root_node = &mut tree.root.unwrap();
            root_node.rotation_right();
            assert_eq!(1, root_node.data);
            assert_eq!(0, root_node.left.as_mut().unwrap().data);
            assert_eq!(2, root_node.right.as_mut().unwrap().data);
        }

        #[test]
        fn rotation_left() {
            let mut tree: AvlTree<i32> = AvlTree::new();
            add_no_update_balance_fac(&mut tree, 0);
            add_no_update_balance_fac(&mut tree, 1);
            add_no_update_balance_fac(&mut tree, 2);
            let root_node = &mut tree.root.unwrap();
            root_node.rotation_left();

            assert_eq!(1, root_node.data);
            assert_eq!(0, root_node.left.as_mut().unwrap().data);
            assert_eq!(2, root_node.right.as_mut().unwrap().data);
        } */
/* 
        #[test]
        fn rebalance_one_rotation_right() {
            let mut tree: AvlTree<i32> = AvlTree::new();
            add_no_rebalance(&mut tree, 0);
            add_no_rebalance(&mut tree, 1);
            add_no_rebalance(&mut tree, 2);
            let root_node = &mut tree.root.unwrap();
            root_node.rebalance();

            assert_eq!(0, root_node.balance_fac);
            assert_eq!(1, root_node.data);
            assert_eq!(0, root_node.left.as_mut().unwrap().data);
            assert_eq!(2, root_node.right.as_mut().unwrap().data);
        }

        #[test]
        fn rebalance_one_rotation_left() {
            let mut tree: AvlTree<i32> = AvlTree::new();
            add_no_rebalance(&mut tree, 2);
            add_no_rebalance(&mut tree, 1);
            add_no_rebalance(&mut tree, 0);
            let root_node = &mut tree.root.unwrap();
            root_node.rebalance();
            assert_eq!(0, root_node.balance_fac);
            assert_eq!(1, root_node.data);
            assert_eq!(0, root_node.left.as_mut().unwrap().data);
            assert_eq!(2, root_node.right.as_mut().unwrap().data);
        }

        #[test]
        fn rebalance_double_rotation_right() {
            let mut tree: AvlTree<i32> = AvlTree::new();
            add_no_rebalance(&mut tree, 2);
            add_no_rebalance(&mut tree, 0);
            add_no_rebalance(&mut tree, 1);
            let root_node = &mut tree.root.unwrap();
            root_node.rebalance();

            assert_eq!(0, root_node.balance_fac);
            assert_eq!(1, root_node.data);
            assert_eq!(0, root_node.left.as_mut().unwrap().data);
            assert_eq!(2, root_node.right.as_mut().unwrap().data);
        }

        #[test]
        fn rebalance_double_rotation_left() {
            let mut tree: AvlTree<i32> = AvlTree::new();
            add_no_rebalance(&mut tree, 2);
            add_no_rebalance(&mut tree, 4);
            add_no_rebalance(&mut tree, 3);
            let root_node = &mut tree.root.unwrap();
            root_node.rebalance();

            assert_eq!(3, root_node.data);
            assert_eq!(2, root_node.left.as_mut().unwrap().data);
            assert_eq!(4, root_node.right.as_mut().unwrap().data);
        } */
    }
}
